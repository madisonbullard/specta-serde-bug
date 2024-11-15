#[derive(Debug, thiserror::Error, specta::Type)]
pub enum Error {
    #[error("API call failed: {0}")]
    ApiError(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
}

#[derive(Debug, serde::Serialize)]
#[serde(tag = "code", content = "message", rename_all = "camelCase")]
pub enum ErrorKind {
    ApiError(String),
    Unauthorized(String),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let error_message = self.to_string();
        let error_kind = match self {
            Self::ApiError(_) => ErrorKind::ApiError(error_message),
            Self::Unauthorized(_) => ErrorKind::Unauthorized(error_message),
        };
        error_kind.serialize(serializer)
    }
}

fn main() {
    assert_eq!(
        specta_typescript::export::<Error>(&Default::default()).unwrap(),
        "export type Error = { code: \"apiError\", message: string } | { code: \"unauthorized\", message: string }".to_string()
    );
    println!("ok!");
}
