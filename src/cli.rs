use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::model::TargetType;

#[derive(Parser, Debug)]
#[command(author = "Uy Ha", version, about, long_about = "CMake on steroid")]
pub struct Cmoid {
    #[command(subcommand)]
    pub comamnd: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Config(ConfigArgs),
    List(ListArgs),
    Build(BuildArgs),
    Schema { schema: TargetSchema },
}

#[derive(Args, Debug)]
pub struct ConfigArgs {
    #[arg(default_value_t = String::from("."))]
    pub source: String,
    #[arg(default_value_t = String::from("build"))]
    pub build: String,
    #[arg(last = true)]
    pub options: Vec<String>,
}

#[derive(Args, Debug)]
pub struct ListArgs {
    #[arg(value_enum)]
    pub target_type: Vec<TargetType>,

    #[arg(short, long, default_value_t = String::from("build"))]
    pub build: String,
}

#[derive(Args, Debug)]
pub struct BuildArgs {
    #[arg()]
    pub targets: Vec<String>,

    #[arg(short, long, default_value_t = String::from("build"))]
    pub build: String,

    #[arg(last = true)]
    pub options: Vec<String>,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum TargetSchema {
    Index,
    Codemodel,
    Directory,
    Target,
    Backtrace,
    ConfigureLog,
    Cache,
    CmakeFiles,
    Toolchains,
}
