mod content;
mod group;
mod header;
mod layout;
mod split;
mod tabs;

use super::Command;
use async_channel::Sender;
use futures::executor::block_on;

pub use content::*;
pub use group::*;
pub use header::*;
pub use layout::*;
pub use split::*;
pub use tabs::*;

#[derive(Clone)]
pub struct CommandBus {
    sender: Sender<Command>,
}
impl CommandBus {
    fn new(sender: Sender<Command>) -> Self {
        Self { sender }
    }

    async fn send(&self, command: Command) {
        self.sender.send(command).await.unwrap();
    }

    fn send_blocking(&self, command: Command) {
        block_on(self.send(command))
    }
}
impl PartialEq for CommandBus {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
