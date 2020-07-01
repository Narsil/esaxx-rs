pub type Bucket = [usize];
pub type StringT = [char];
pub type SArray = [usize];

#[derive(Debug)]
pub enum SuffixError {
    InvalidLength,
    Internal,
    IntConversion(std::num::TryFromIntError),
}

impl From<std::num::TryFromIntError> for SuffixError {
    fn from(err: std::num::TryFromIntError) -> Self {
        Self::IntConversion(err)
    }
}
