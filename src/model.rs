use clap::ValueEnum;
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Deserialize, JsonSchema, Debug, PartialEq, Eq)]
#[serde(rename = "kind")]
pub enum ObjectKind {
    #[serde(rename = "codemodel")]
    Codemodel,
    #[serde(rename = "configureLog")]
    ConfigureLog,
    #[serde(rename = "cache")]
    Cache,
    #[serde(rename = "cmakeFiles")]
    CmakeFiles,
    #[serde(rename = "toolchains")]
    Toolchains,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct ObjectVersion {
    pub major: u16,
    pub minor: u16,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Object {
    pub kind: ObjectKind,
    pub version: ObjectVersion,
    pub json_file: String,
}

#[derive(Deserialize, JsonSchema, Debug, ValueEnum, Copy, Clone, PartialEq)]
#[serde(rename = "type")]
pub enum TargetType {
    #[serde(rename = "EXECUTABLE")]
    Executable,
    #[serde(rename = "STATIC_LIBRARY")]
    StaticLibrary,
    #[serde(rename = "SHARED_LIBRARY")]
    SharedLibrary,
    #[serde(rename = "MODULE_LIBRARY")]
    ModuleLibrary,
    #[serde(rename = "OBJECT_LIBRARY")]
    ObjectLibrary,
    #[serde(rename = "INTERFACE_LIBRARY")]
    InterfaceLibrary,
    #[serde(rename = "UTILITY")]
    Utility,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BacktraceNode {
    pub file: u16,
    pub line: u32,
    pub command: Option<usize>,
    pub parent: Option<usize>,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Backtrace {}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TargetFolder {
    pub name: String,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TargetPaths {
    pub source: String,
    pub build: String,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TargetArtifact {
    pub path: String,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TargetInstallPrefix {
    pub path: String,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TargetInstallDestination {
    pub path: String,
    pub backtrace: Option<u32>,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TargetInstall {
    pub prefix: TargetInstallPrefix,
    pub destinations: Vec<TargetInstallDestination>,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub enum TargetLinkCommandFragmentRole {
    #[serde(rename = "flags")]
    Flags,
    #[serde(rename = "libraries")]
    Libraries,
    #[serde(rename = "libraryPath")]
    LibraryPath,
    #[serde(rename = "frameworkPath")]
    FrameworkPath,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TargetLinkCommandFragment {
    pub fragment: String,
    pub role: TargetLinkCommandFragmentRole,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TargetLinkSysroot {
    pub path: String,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TargetLink {
    pub language: String,
    pub command_fragments: Vec<TargetLinkCommandFragment>,
    pub lto: Option<bool>,
    pub sysroot: Option<TargetLinkSysroot>,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub enum TargetArchiveCommandFragmentRole {
    #[serde(rename = "flags")]
    Flags,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TargetArchiveCommandFragment {
    pub fragment: String,
    pub role: TargetArchiveCommandFragmentRole,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TargetArchive {
    pub fragments: Option<Vec<TargetArchiveCommandFragment>>,
    pub lto: Option<bool>,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TargetDependency {
    pub id: String,
    pub backtrace: Option<u32>,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub enum TargetFileSetType {
    #[serde(rename = "HEADERS")]
    Headers,
    #[serde(rename = "CXX_MODULES")]
    CxxModules,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub enum TargetFileSetVisibility {
    #[serde(rename = "PUBLIC")]
    Public,
    #[serde(rename = "PRIVATE")]
    Private,
    #[serde(rename = "INTERFACE")]
    Interface,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TargetFileSet {
    pub name: String,
    #[serde(rename = "type")]
    pub file_set_type: TargetFileSetType,
    pub visibility: TargetFileSetVisibility,
    pub base_directories: Vec<String>,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TargetSource {
    pub path: String,
    pub compile_group_index: Option<u32>,
    pub source_group_index: Option<u32>,
    pub is_generated: Option<bool>,
    pub file_set_index: Option<u32>,
    pub backtrace: Option<u32>,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TargetSourceGroup {
    pub name: String,
    pub source_indexes: Vec<u32>,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TargetCompileGroupLanguageStandard {
    pub backtraces: Option<Vec<u32>>,
    pub standard: String,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TargetCompileGroupCompileCommandFragment {
    pub fragment: String,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TargetCompileGroupInclude {
    pub path: String,
    pub is_system: Option<bool>,
    pub backtrace: Option<u32>,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TargetCompileGroupFramework {
    pub path: String,
    pub is_system: Option<bool>,
    pub backtrace: Option<u32>,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TargetCompileGroupPrecompileHeader {
    pub header: String,
    pub backtrace: Option<u32>,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TargetCompileGroupDefine {
    pub define: String,
    pub backtrace: Option<u32>,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TargetCompileGroupSysroot {
    pub path: String,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TargetCompileGroup {
    pub source_indexes: Vec<u32>,
    pub language: String,
    pub language_standard: Option<TargetCompileGroupLanguageStandard>,
    pub compile_command_fragments: Option<Vec<TargetCompileGroupCompileCommandFragment>>,
    pub includes: Option<Vec<TargetCompileGroupInclude>>,
    pub frameworks: Option<Vec<TargetCompileGroupFramework>>,
    pub precompile_headers: Option<Vec<TargetCompileGroupPrecompileHeader>>,
    pub defines: Option<Vec<TargetCompileGroupDefine>>,
    pub sysroot: Option<TargetCompileGroupSysroot>,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TargetBacktraceGraphNode {
    pub file: u32,
    pub line: Option<u32>,
    pub command: Option<u32>,
    pub parent: Option<u32>,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TargetBacktraceGraph {
    pub nodes: Vec<TargetBacktraceGraphNode>,
    pub commands: Vec<String>,
    pub files: Vec<String>,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    pub name: String,
    pub id: String,
    #[serde(rename = "type")]
    pub target_type: TargetType,
    pub backtrace: Option<u32>,
    pub folder: Option<TargetFolder>,
    pub paths: TargetPaths,
    pub name_on_disk: Option<String>,
    pub artifacts: Option<Vec<TargetArtifact>>,
    pub is_generator_provided: Option<bool>,
    pub install: Option<TargetInstall>,
    pub link: Option<TargetLink>,
    pub archive: Option<TargetArchive>,
    pub dependencies: Option<Vec<TargetDependency>>,
    pub file_sets: Option<Vec<TargetFileSet>>,
    pub sources: Vec<TargetSource>,
    pub source_groups: Option<Vec<TargetSourceGroup>>,
    pub compile_groups: Option<Vec<TargetCompileGroup>>,
    pub backtrace_graph: TargetBacktraceGraph,
}

pub struct Directory {}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CodemodelPaths {
    pub source: String,
    pub build: String,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MinimumCmakeVersion {
    pub string: String,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CodemodelDirectory {
    pub source: String,
    pub build: String,
    pub parent_index: Option<u32>,
    pub child_indexes: Option<Vec<u32>>,
    pub project_index: u32,
    pub target_indexes: Option<Vec<u32>>,
    #[serde(rename = "minimumCMakeVersion")]
    pub minimum_cmake_version: Option<MinimumCmakeVersion>,
    pub has_install_rule: Option<bool>,
    pub json_file: String,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CodemodelProject {
    pub name: String,
    pub parent_index: Option<u32>,
    pub child_indexes: Option<Vec<u32>>,
    pub directory_indexes: Vec<u32>,
    pub target_indexes: Option<Vec<u32>>,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CodemodelTarget {
    pub name: String,
    pub id: String,
    pub directory_index: u32,
    pub project_index: u32,
    pub json_file: String,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CodemodelConfiguration {
    pub name: String,
    pub directories: Vec<CodemodelDirectory>,
    pub projects: Vec<CodemodelProject>,
    pub targets: Vec<CodemodelTarget>,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Codemodel {
    pub kind: ObjectKind,
    pub version: ObjectVersion,
    pub paths: CodemodelPaths,
    pub configurations: Vec<CodemodelConfiguration>,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CMakeVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
    pub suffix: String,
    pub string: String,
    pub is_dirty: bool,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CMakePaths {
    pub cmake: String,
    pub ctest: String,
    pub cpack: String,
    pub root: String,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CMakeGenerator {
    pub multi_config: bool,
    pub name: String,
    pub platform: Option<String>,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CMake {
    pub version: CMakeVersion,
    pub paths: CMakePaths,
    pub generator: CMakeGenerator,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct CMoidReply {
    #[serde(rename = "codemodel-v2")]
    pub codemodel: Object,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct CMoidContainer {
    #[serde(rename = "client-cmoid")]
    pub client_cmoid: CMoidReply,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct Index {
    pub cmake: CMake,
    pub objects: Vec<Object>,
    pub reply: CMoidContainer,
}
