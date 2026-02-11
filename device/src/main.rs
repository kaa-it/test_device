use proto::command_service_server::{CommandService, CommandServiceServer};
use proto::{Command, command::Command as CommandType, Response, response::Response as ResponsePackage, ConnectedResponse};
use tonic::{Request, Response as TonicResponse, Status};
use tonic::transport::Server;

#[derive(Clone)]
pub struct CommandServiceImpl {}

impl CommandServiceImpl {
    pub fn new() -> Self {
        CommandServiceImpl {}
    }
}

#[tonic::async_trait]
impl CommandService for CommandServiceImpl {
    async fn send_command(
        &self,
        request: Request<Command>,
    ) -> Result<TonicResponse<Response>, Status>
    {
        let request = request.into_inner();

        match request.command.unwrap() {
            CommandType::Connect(_) => {
                println!("Connect command received");
            }
            CommandType::Disconnect(_) => {
                println!("Disconnect command received");
            }
        }

        Ok(TonicResponse::new(
            Response
            {
                response: Option::from(
                    ResponsePackage::Connected(
                        ConnectedResponse {}
                    )
                )
            }
        ))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = format!("0.0.0.0:{}", 8787).parse()?;
    let service = CommandServiceImpl::new();

    Server::builder()
        .add_service(CommandServiceServer::new(service))
        .serve(address).await?;

    Ok(())
}
