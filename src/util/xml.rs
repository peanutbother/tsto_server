use axum::{
    http::{header, HeaderValue},
    response::{IntoResponse, Response},
};
use reqwest::StatusCode;

#[macro_export]
macro_rules! xml_response {
    () => {
        $crate::util::Xml::internal_error()
    };
    ($field:literal) => {
        $crate::xml_response!(400, $field)
    };
    ($code:literal, $field:literal) => {
        $crate::util::Xml(
            $code,
            concat!(
                r#"<?xml version="1.0" encoding="UTF-8"?><error code=""#,
                $code,
                r#"" type="BAD_REQUEST" field=""#,
                $field,
                r#""/>"#
            )
            .to_owned(),
        )
    };
}

#[derive(Debug, Clone, Default)]
pub struct Xml(pub u16, pub String);

impl<S, M> From<(S, M)> for Xml
where
    S: Into<u16>,
    M: Into<String>,
{
    fn from(value: (S, M)) -> Self {
        Self(value.0.into(), value.1.into())
    }
}

impl Xml {
    pub const CONTENT_TYPE: &str = "application/xml";

    pub fn internal_error() -> Xml {
        (
            500_u16,
            r#"<?xml version="1.0" encoding="UTF-8"?><error code="500" type="INTERNAL_SERVER_ERROR" />"#,
        )
            .into()
    }

    pub fn ok(content: impl Into<String>) -> Xml {
        (200_u16, content).into()
    }

    /// used by `Result:map_error`-calls to map the original error to an Xml error
    ///
    /// returns closure which prints the error prefixed by `message` using `tracing::error` then returns `Xml`
    pub fn log_with_message<Fmt, E>(&self, message: Fmt) -> impl Fn(E) -> Self + use<'_, Fmt, E>
    where
        Fmt: std::fmt::Display,
        E: std::error::Error,
    {
        move |error: E| -> Self {
            tracing::error!("{message}: {error:#?}");
            self.clone()
        }
    }
}

impl std::ops::Deref for Xml {
    type Target = String;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl std::ops::DerefMut for Xml {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.1
    }
}

impl IntoResponse for Xml {
    fn into_response(self) -> Response {
        let mut res = self.1.into_response();
        res.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static(Self::CONTENT_TYPE),
        );
        *res.status_mut() = StatusCode::from_u16(self.0).expect("valid u16 status code provided");

        res
    }
}
