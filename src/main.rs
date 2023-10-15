use clap::Parser;
use cmoid::{
    action::{build, config, list},
    cli::{Cmoid, Commands},
    model::{Codemodel, Index},
};
use schemars::schema_for;

fn main() -> anyhow::Result<()> {
    let cmoid = Cmoid::parse();

    match &cmoid.comamnd {
        Commands::Config(args) => config(&args.source, &args.build, &args.options)?,
        Commands::List(args) => list(&args.build, &args.target_type)?,
        Commands::Build(args) => build(&args.build, &args.targets, &args.options)?,
        Commands::Schema { schema } => match schema {
            cmoid::cli::TargetSchema::Index => {
                let schema = schema_for!(Index);
                println!("{}", serde_json::to_string_pretty(&schema).unwrap());
            }
            cmoid::cli::TargetSchema::Codemodel => {
                let schema = schema_for!(Codemodel);
                println!("{}", serde_json::to_string_pretty(&schema).unwrap());
            }
            cmoid::cli::TargetSchema::Directory => todo!(),
            cmoid::cli::TargetSchema::Target => todo!(),
            cmoid::cli::TargetSchema::Backtrace => todo!(),
            cmoid::cli::TargetSchema::ConfigureLog => todo!(),
            cmoid::cli::TargetSchema::Cache => todo!(),
            cmoid::cli::TargetSchema::CmakeFiles => todo!(),
            cmoid::cli::TargetSchema::Toolchains => todo!(),
        },
    };

    Ok(())
}
