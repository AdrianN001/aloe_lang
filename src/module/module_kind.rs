#[derive(Default, Debug, Clone)]
pub enum ModuleKind {
    #[default]
    SourceFile, // .aloe,
    ArtifactFile, // .aloeo
    Prelude,
}
