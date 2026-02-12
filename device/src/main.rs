use kameo::prelude::*;
use proto::command_service_server::{CommandService, CommandServiceServer};
use proto::{
    Command, ConnectedResponse, Response, command::Command as CommandType,
    response::Response as ResponsePackage,
};
use tonic::transport::Server;
use tonic::{Request, Response as TonicResponse, Status};
use tonic_reflection::server::Builder;

use crate::actor::{Connect, ConnectCommand, Controller, DeviceActor};

mod actor;

//#[derive(Clone)]
pub struct CommandServiceImpl {
    controller: Controller,
}

impl CommandServiceImpl {
    pub fn new() -> Self {
        CommandServiceImpl {
            controller: Controller::new(),
        }
    }
}

#[tonic::async_trait]
impl CommandService for CommandServiceImpl {
    async fn send_command(
        &self,
        request: Request<Command>,
    ) -> Result<TonicResponse<Response>, Status> {
        let request = request.into_inner();

        match request.command.unwrap() {
            CommandType::Connect(_) => {
                println!("Connect command received");
                match self.controller.send_command(ConnectCommand {}).await {
                    Ok(s) => println!("{}", s),
                    Err(e) => println!("{:?}", e),
                }
            }
            CommandType::Disconnect(_) => {
                println!("Disconnect command received");
            }
        }

        Ok(TonicResponse::new(Response {
            response: Option::from(ResponsePackage::Connected(ConnectedResponse {})),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = format!("0.0.0.0:{}", 8787).parse()?;
    let service = CommandServiceImpl::new();

    let reflection_service = Builder::configure()
        .register_encoded_file_descriptor_set(proto::reflection::FILE_DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        .add_service(CommandServiceServer::new(service))
        .add_service(reflection_service)
        .serve(address)
        .await?;

    println!("Test");

    Ok(())
}
