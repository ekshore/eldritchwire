use std::alloc;
#[derive(Debug, Clone, PartialEq)]
pub enum ShieldError<E> {
    Transport(E),
    InvalidResponse,
    OutOfRange,
    MemoryAllocationError(alloc::LayoutError),
    // add others as needed
}

impl<E> From<E> for ShieldError<E> {
    fn from(err: E) -> Self {
        ShieldError::Transport(err)
    }
}

