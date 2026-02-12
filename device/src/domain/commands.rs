use proto::command::Command as ProtoCommand;

#[derive(Debug)]
pub enum Command {
    Connect,
    Disconnect,
}

impl From<ProtoCommand> for Command {
    fn from(value: ProtoCommand) -> Self {
        match value {
            ProtoCommand::Connect(_) => Command::Connect {},
            ProtoCommand::Disconnect(_) => Command::Disconnect {},
        }
    }
}
