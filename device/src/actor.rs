use crate::domain;
use std::time::Duration;

use kameo::prelude::*;
use tokio::sync::mpsc::UnboundedSender;

#[derive(Actor)]
pub struct DeviceActor;

pub struct Command {
    tx: UnboundedSender<Result<domain::Response, domain::Error>>,
    command: domain::Command,
}

impl Message<Command> for DeviceActor {
    type Reply = Result<(), domain::Error>;

    async fn handle(&mut self, msg: Command, _: &mut Context<Self, Self::Reply>) -> Self::Reply {
        println!("Handling...");

        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(20)).await;
            // TODO: temporarily ignored
            _ = msg.tx.send(Ok(domain::Response::Connected {}));
        });

        println!("Queued");

        Ok(())
    }
}

pub struct Controller {
    device_actor: ActorRef<DeviceActor>,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            device_actor: DeviceActor::spawn(DeviceActor),
        }
    }

    pub async fn send_command(
        &self,
        command: domain::Command,
    ) -> Result<domain::Response, domain::Error> {
        let (tx, mut rx) =
            tokio::sync::mpsc::unbounded_channel::<Result<domain::Response, domain::Error>>();

        // TODO: temporarily ignored
        _ = self.device_actor.ask(Command { tx, command }).await;

        rx.recv().await.unwrap()
    }
}
