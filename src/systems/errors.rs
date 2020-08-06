use juniper::graphql_value;
use juniper::FieldError;
pub use juniper::IntoFieldError;

#[derive(Debug)]
pub enum ErrorKind {
    NotFound(String, String),
    InternalError,
}

impl IntoFieldError for ErrorKind {
    fn into_field_error(self) -> FieldError {
        use ErrorKind::{InternalError, NotFound};
        match self {
            NotFound(error, subtype) => FieldError::new(
                error,
                graphql_value!({
                    "type": "NOT_FOUND",
                    "subtype": subtype
                }),
            ),
            InternalError => FieldError::new(
                "Internal error",
                graphql_value!({
                    "type": "INTERNAL_ERROR"
                }),
            ),
        }
    }
}
