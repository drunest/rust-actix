// https://github.com/rust-awesome-app/template-app-base/blob/main/src-tauri/src/error.rs

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Fail to get Ctx")]
    CtxFail,

    #[error("Fetching from Untis failed")]
    UntisError,

    #[error("Value not of type '{0}'")]
    XValueNotOfType(&'static str),

    #[error("Property '{0}' not found")]
    XPropertyNotFound(String),

    #[error("Fail to create. Cause: {0}")]
    StoreFailToCreate(String),

    #[error(transparent)]
    Surreal(#[from] surrealdb::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    JSON(#[from] serde_json::Error),

    #[error(transparent)]
    Actix(#[from] actix_web::Error),
}
