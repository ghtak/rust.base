use thiserror::Error;

#[derive(Error, Debug)]
enum Error{

    #[error("cannot deserialize JSON")]
    CannotDeserializeJson(#[source] serde_json::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error)
}