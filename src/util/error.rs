#[macro_export]
macro_rules! json_error {
    () => {
        $crate::util::ErrorMessage::internal_error()
    };
    ($message:expr) => {
        $crate::json_error!(400, $message)
    };
    ($code:expr, $message:expr) => {
        $crate::util::ErrorMessage::new($message).code($code)
    };
}

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct ErrorMessage {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    #[serde(skip)]
    pub code: u16,
}

impl ErrorMessage {
    pub fn new(message: impl AsRef<str>) -> Self {
        Self {
            message: message.as_ref().to_owned(),
            error_description: None,
            code: 400,
        }
    }

    pub fn internal_error() -> Self {
        ErrorMessage {
            message: "Internal Server Error".to_owned(),
            code: 500,
            error_description: None,
        }
    }

    pub fn message(mut self, message: impl AsRef<str>) -> Self {
        self.message = message.as_ref().to_owned();
        self
    }

    pub fn error_description(mut self, error_description: Option<impl AsRef<str>>) -> Self {
        self.error_description = error_description.map(|e| e.as_ref().to_owned());
        self
    }

    pub fn code(mut self, code: u16) -> Self {
        self.code = code;
        self
    }
}

impl From<&str> for ErrorMessage {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

#[cfg(feature = "server")]
impl axum::response::IntoResponse for ErrorMessage {
    fn into_response(self) -> axum::response::Response {
        let code = self.code;
        let mut res = axum::Json(self).into_response();

        *res.status_mut() =
            axum::http::StatusCode::from_u16(code).expect("valid u16 status code provided");

        res
    }
}
