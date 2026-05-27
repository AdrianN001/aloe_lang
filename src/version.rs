
pub const CURRENT_VERSION: AloeVersion = AloeVersion::Avocado;

pub enum AloeVersion{
    Avocado,             // v1
}

impl AloeVersion{
    pub fn to_string(&self) -> String{
        match self{
            AloeVersion::Avocado => "avocado".to_string()
        }
    }
}