use proto::command_service_server::{CommandService, CommandServiceServer};
use proto::{Command, Response};
use tonic::transport::Server;
use tonic::{Request, Response as TonicResponse, Status};
use tonic_reflection::server::Builder;

use crate::actor::Controller;

mod actor;
mod domain;

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

        println!("Command received");

        let command: domain::Command = request.command.unwrap().into();

        match self.controller.send_command(command).await {
            Ok(r) => {
                println!("{:?}", r);
                Ok(TonicResponse::new(r.into()))
            }
            Err(e) => {
                println!("{:?}", e);
                Ok(TonicResponse::new(e.into()))
            }
        }
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
