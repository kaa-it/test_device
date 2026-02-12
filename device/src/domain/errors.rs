use proto::{
    CommandError, FailedResponse, command_error::Error as ProtoError,
    response::Response as ProtoResponse,
};

#[derive(Debug)]
pub enum Error {
    ConnectFailed(String),
    DisconnectFailed(String),
}

impl From<Error> for proto::Response {
    fn from(value: Error) -> Self {
        match value {
            Error::ConnectFailed(s) => proto::Response {
                response: Some(ProtoResponse::Failed(FailedResponse {
                    error: Some(CommandError {
                        error: Some(ProtoError::ConnectFailed(s)),
                    }),
                })),
            },
            Error::DisconnectFailed(s) => proto::Response {
                response: Some(ProtoResponse::Failed(FailedResponse {
                    error: Some(CommandError {
                        error: Some(ProtoError::DisconnectFailed(s)),
                    }),
                })),
            },
        }
    }
}
