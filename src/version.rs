pub const CURRENT_VERSION: AloeVersion = AloeVersion::Avocado;

#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub enum AloeVersion {
    Avocado, // v1
}

impl AloeVersion {
    pub fn to_string(&self) -> String {
        match self {
            AloeVersion::Avocado => "avocado".to_string(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            AloeVersion::Avocado => vec![0x01],
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        match bytes {
            [0x01] => Some(AloeVersion::Avocado),
            _ => None,
        }
    }
}
