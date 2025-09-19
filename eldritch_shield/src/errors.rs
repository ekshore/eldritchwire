#[derive(Debug, Clone, PartialEq)]
pub enum ShieldError<E> {
    Transport(E),
    InvalidResponse,
    OutOfRange,
    // add others as needed
}

impl<E> From<E> for ShieldError<E> {
    fn from(err: E) -> Self {
        ShieldError::Transport(err)
    }
}

