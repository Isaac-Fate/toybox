use thiserror::Error;

pub type IntervalSetResult<T> = std::result::Result<T, IntervalSetError>;

#[derive(Error, Debug)]
pub enum IntervalSetError {
    #[error("invalid interval")]
    InvalidInterval,

    #[error("separated intervals cannot be merged")]
    MergeSeparatedIntervals,
}
