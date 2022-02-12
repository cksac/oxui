#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Clip {
    None,
    HardEdge,
    AntiAlias,
    AntiAliasWithSaveLayer,
}
