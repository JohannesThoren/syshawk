pub enum Status {
    SUCCESS,
    UNREACHABLE,
}

impl Status {
    pub fn to_u8(&self) -> u8{
        match self {
            Status::SUCCESS => {0}
            Status::UNREACHABLE => {1}
        }
    }
}