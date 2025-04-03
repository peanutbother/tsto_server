use axum::{
    async_trait,
    body::Bytes,
    extract::{rejection::BytesRejection, FromRequest, Request},
    http::{self, header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use axum_core::{
    __composite_rejection as composite_rejection, __define_rejection as define_rejection,
};
use bytes::BytesMut;
use prost::Message;

pub const PROTOBUF_CONTENT_TYPE: &str = "application/x-protobuf";

#[derive(Debug, Clone, Copy, Default)]
pub struct Protobuf<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for Protobuf<T>
where
    T: Message + Default,
    S: Send + Sync,
{
    type Rejection = ProtobufRejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let mut bytes = Bytes::from_request(req, state).await?;

        match T::decode(&mut bytes) {
            Ok(value) => Ok(Protobuf(value)),
            Err(err) => Err(ProtobufDecodeError::from_err(err).into()),
        }
    }
}

axum_core::__impl_deref!(Protobuf);

impl<T> From<T> for Protobuf<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}

impl<T> IntoResponse for Protobuf<T>
where
    T: Message + Default,
{
    fn into_response(self) -> Response {
        let mut buf = BytesMut::new();
        match &self.0.encode(&mut buf) {
            Ok(()) => {
                let mut res = buf.into_response();
                res.headers_mut().insert(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static(PROTOBUF_CONTENT_TYPE),
                );

                res
            }
            Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
        }
    }
}

define_rejection! {
    #[status = UNPROCESSABLE_ENTITY]
    #[body = "Failed to decode the body"]
    /// Rejection type for [`Protobuf`].
    ///
    /// This rejection is used if the request body couldn't be decoded into the target type.
    pub struct ProtobufDecodeError(Error);
}

composite_rejection! {
    /// Rejection used for [`Protobuf`].
    ///
    /// Contains one variant for each way the [`Protobuf`] extractor
    /// can fail.
    pub enum ProtobufRejection {
        ProtobufDecodeError,
        BytesRejection,
    }
}
