#[derive(Debug, thiserror::Error, specta::Type)]
enum Error {
    #[error("io error: {0}")]
    Io(String),
    #[error("failed to parse as string: {0}")]
    Utf8(String),
}

#[derive(serde::Serialize)]
#[serde(tag = "kind", content = "message")]
#[serde(rename_all = "camelCase")]
enum ErrorKind {
    Io(String),
    Utf8(String),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let error_message = self.to_string();
        let error_kind = match self {
            Self::Io(_) => ErrorKind::Io(error_message),
            Self::Utf8(_) => ErrorKind::Utf8(error_message),
        };
        error_kind.serialize(serializer)
    }
}

fn main() {
    assert_eq!(
        specta_typescript::export::<Error>(&Default::default()).unwrap(),
        "export type Error = { kind: \"io\"; message: string } | { kind: \"utf8\"; message: string }".to_string()
    );
    println!("ok!");
}
