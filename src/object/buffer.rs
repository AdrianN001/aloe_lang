//TODO: implement Iteration
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Buffer {
    pub data: Box<[u8]>,
    pub size: usize,
}

impl Buffer {
    pub fn get_type(&self) -> String {
        String::from("buffer")
    }

    pub fn inspect(&self) -> String {
        String::from("blob")
    }

    pub fn new_empty() -> Self {
        Buffer {
            data: vec![].into_boxed_slice(),
            size: 0,
        }
    }
}
