//mod content;
mod group;
mod header;
mod layout;
mod split;
mod tabs;

use super::TabsCommand;
use async_channel::Sender;
use futures::executor::block_on;

//pub use content::*;
pub use group::*;
pub use header::*;
pub use layout::*;
pub use split::*;
pub use tabs::*;

/*
#[derive(Clone)]
pub struct TabsCommandBus {
    sender: Sender<TabsCommand>,
}
impl TabsCommandBus {
    fn new(sender: Sender<TabsCommand>) -> Self {
        Self { sender }
    }

    async fn send(&self, command: TabsCommand) {
        self.sender.send(command).await.unwrap();
    }

    fn send_blocking(&self, command: TabsCommand) {
        block_on(self.send(command))
    }
}
impl PartialEq for TabsCommandBus {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

 */
