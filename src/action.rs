use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

use anyhow::{bail, Context};
use colored::*;
use glob::GlobError;

use crate::model::{Codemodel, Index, Target, TargetType};

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

    Command::new("cmake")
        .args(options)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait()?;

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

pub fn list(build: &str, target_type: TargetType) -> anyhow::Result<()> {
    let path = index_path(build)?;
    let index: Index = serde_json::from_reader(std::fs::File::open(path).unwrap())?;
    let path = &index.reply.client_cmoid.codemodel.json_file;
    let codemodel: Codemodel = serde_json::from_reader(read(build, path)?)?;

    for target in codemodel.configurations[0].targets.iter() {
        let target: Target = serde_json::from_reader(read(build, &target.json_file)?)?;
        if target.target_type == target_type {
            println!("{}", target.name);
        }
        dbg!(target);
    }

    // let codemodel: Vec<&Object> = index
    //     .objects
    //     .iter()
    //     .filter(|object| object.kind == ObjectKind::Codemodel)
    //     .collect();
    //
    // if codemodel.len() > 1 {
    //     bail!(
    //         "Index file in {} is corrupted, please run {} again",
    //         build.bold(),
    //         "cmoid config".bold()
    //     );
    // }
    // if codemodel.is_empty() {
    //     bail!(
    //         "Index file in {} is doesn't contain {}, please run {} again",
    //         build.bold(),
    //         "codemodel".underline(),
    //         "cmoid config".bold()
    //     );
    // }
    //
    // Safe unwrap since we already made sure that there is exactly 1 codemodel
    // let codemodel = {
    //     let reader = std::fs::File::open(format("{}/{}")) * codemodel.first().unwrap();
    // };
    // println!("{}", codemodel.json_file);

    Ok(())
}
