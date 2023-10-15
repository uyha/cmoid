use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

use anyhow::{bail, Context};
use colored::*;
use glob::GlobError;

use crate::model::{Codemodel, Index, Target, TargetType};

fn cmake() -> Command {
    let mut command = Command::new("cmake");
    command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    command
}

pub fn config(source: &str, build: &str, options: &[String]) -> anyhow::Result<()> {
    let options = {
        let mut result = options.to_vec();
        result.extend_from_slice(&[
            String::from("-S"),
            source.to_owned(),
            String::from("-B"),
            build.to_owned(),
        ]);
        result
    };

    std::fs::create_dir_all(format!("{}/.cmake/api/v1/query/client-cmoid/", build))?;
    std::fs::File::create(format!(
        "{}/.cmake/api/v1/query/client-cmoid/codemodel-v2",
        build
    ))?;

    cmake().args(options).spawn()?.wait()?;

    Ok(())
}

fn read(build: &str, path: &str) -> anyhow::Result<impl std::io::Read> {
    Ok(std::fs::File::open(format!(
        "{}/.cmake/api/v1/reply/{}",
        build, path
    ))?)
}

fn index_path(build: &str) -> anyhow::Result<String> {
    let mut path: Vec<Result<PathBuf, GlobError>> =
        glob::glob(&format!("{}/.cmake/api/v1/reply/index*.json", build))
            .unwrap()
            .collect();

    if path.is_empty() {
        bail!(
            "Could not find the index file in {}, please run {}",
            build.bold(),
            "cmoid config".bold(),
        );
    }
    if path.len() > 1 {
        bail!(
            "Too many index files detected in {}, please run {} again",
            build.bold(),
            "cmoid config".bold()
        );
    }

    Ok(path
        .swap_remove(0)
        .unwrap()
        .to_str()
        .context("Path is not a valid UTF-8")?
        .to_string())
}

pub fn list(build: &str, target_types: &[TargetType]) -> anyhow::Result<()> {
    let path = index_path(build)?;
    let index: Index = serde_json::from_reader(std::fs::File::open(path).unwrap())?;
    let path = &index.reply.client_cmoid.codemodel.json_file;
    let codemodel: Codemodel = serde_json::from_reader(read(build, path)?)?;

    for target in codemodel.configurations[0].targets.iter() {
        let target: Target = serde_json::from_reader(read(build, &target.json_file)?)?;
        if target_types.is_empty()
            || target_types
                .iter()
                .any(|target_type| *target_type == target.target_type)
        {
            println!("{}", target.name);
        }
    }

    Ok(())
}

pub fn build(build: &str, targets: &[String], options: &[String]) -> anyhow::Result<()> {
    let mut args = vec![String::from("--build"), build.to_owned()];
    if !targets.is_empty() {
        args.push(String::from("--target"));
        args.extend_from_slice(targets);
    }
    args.extend_from_slice(options);

    cmake().args(args).spawn()?.wait()?;

    Ok(())
}
