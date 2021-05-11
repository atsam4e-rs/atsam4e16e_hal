mod descriptor;
use descriptor::RxDescriptor;

mod descriptor_block;
pub use descriptor_block::RxDescriptorBlock;

use super::{
    DescriptorBlock,
    DescriptorEntry,
    MTU,
    Receiver,
    VolatileReadWrite,
};

pub enum RxError {
    WouldBlock,
}