#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Null {}

impl Null {
    pub fn get_type(&self) -> String {
        "null".into()
    }

    pub fn inspect(&self) -> String {
        "null".into()
    }
}
