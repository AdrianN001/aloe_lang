use crate::doc::symbol::documentation::Documentation;

impl Documentation {
    pub fn export_to_json_str(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}
