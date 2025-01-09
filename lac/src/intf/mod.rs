
pub enum LacError {
    SdkError,
    Uninitialized,
}

pub type LacResult<T> = Result<T, LacError>;

pub fn lac_init() -> LacResult<()> {
    Ok(())
}

pub fn lac_query_chip_info() -> LacResult<()> {
    Ok(())
}