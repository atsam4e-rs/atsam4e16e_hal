mod bmcr;
pub use bmcr::*;

pub trait MediaIndependentInterface {
    fn bmcr(&self) -> &BMCR;
}