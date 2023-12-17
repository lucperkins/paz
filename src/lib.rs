pub type Result<T> = color_eyre::eyre::Result<T, PazError>;

#[derive(Debug, thiserror::Error)]
pub enum PazError {
    #[error(transparent)]
    Http(#[from] reqwest::Error),

    #[error(transparent)]
    Report(#[from] color_eyre::eyre::ErrReport),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
