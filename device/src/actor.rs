use std::time::Duration;

use kameo::prelude::*;
use tokio::sync::mpsc::UnboundedSender;

#[derive(Actor)]
pub struct DeviceActor;

pub struct ConnectCommand;

pub struct Connect {
    tx: UnboundedSender<String>,
    command: ConnectCommand,
}

impl Message<Connect> for DeviceActor {
    type Reply = Result<(), String>;

    async fn handle(&mut self, msg: Connect, _: &mut Context<Self, Self::Reply>) -> Self::Reply {
        println!("Handling...");

        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(20)).await;
            msg.tx.send("Connected".to_string());
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

    pub async fn send_command(&self, command: ConnectCommand) -> Result<String, String> {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();

        self.device_actor
            .ask(Connect { tx, command })
            .await
            .map_err(|e| e.to_string())?;

        rx.recv().await.ok_or("Failed".to_string())
    }
}
