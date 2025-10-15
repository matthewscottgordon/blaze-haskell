pub type Result<T> = std::result::Result<T, Error>;

pub struct Error {
    msg: String,
}

impl Error {
    pub fn new<T: Into<String>>(msg: T) -> Self {
        Self { msg: msg.into() }
    }
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, self.msg).into_response()
    }
}
