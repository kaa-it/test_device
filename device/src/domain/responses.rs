use proto::response::Response as ProtoResponse;
use proto::{ConnectedResponse, DisconnectedResponse};

#[derive(Debug)]
pub enum Response {
    Connected,
    Disconnected,
}

impl From<Response> for proto::Response {
    fn from(value: Response) -> Self {
        match value {
            Response::Connected => proto::Response {
                response: Some(ProtoResponse::Connected(ConnectedResponse {})),
            },
            Response::Disconnected => proto::Response {
                response: Some(ProtoResponse::Disconnected(DisconnectedResponse {})),
            },
        }
    }
}
