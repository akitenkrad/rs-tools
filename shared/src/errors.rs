use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    // wordcloud errors
    #[error("WordCloud Error: {0}")]
    WordCloudError(String),
}

pub type AppResult<T> = Result<T, AppError>;
