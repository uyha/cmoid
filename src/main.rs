use clap::Parser;
use cmoid::{
    action::{config, list},
    cli::{Cmoid, Commands},
    model::{Codemodel, Index},
};
use schemars::schema_for;

fn main() {
    let cmoid = Cmoid::parse();

    match &cmoid.comamnd {
        Commands::Config(args) => {
            if let Some(msg) = config(&args.source, &args.build, &args.options).err() {
                eprintln!("{}", msg);
            }
        }
        Commands::List(args) => {
            if let Some(msg) = list(&args.build, &args.target_type).err() {
                eprintln!("{}", msg);
            }
        }
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
    }
}
