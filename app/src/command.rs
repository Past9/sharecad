use async_channel::{Receiver, Sender};
use dioxus::prelude::*;
use futures::executor::block_on;

pub struct CommandBus<TCommand: std::fmt::Debug + 'static> {
    sender: Sender<TCommand>,
    receiver: Receiver<TCommand>,
}
impl<TCommand: std::fmt::Debug + 'static> CommandBus<TCommand> {
    pub fn new() -> Self {
        log::debug!("new command bus");
        let (sender, receiver) = async_channel::unbounded::<TCommand>();
        Self { sender, receiver }
    }

    pub async fn send(&self, command: TCommand) {
        self.sender.send(command).await.unwrap();
    }

    pub fn send_blocking(&self, command: TCommand) {
        block_on(self.send(command))
    }

    pub fn listen<'a, TProps, THandler: Fn(&TCommand)>(
        &self,
        cx: &Scoped<'a, TProps>,
        handler: THandler,
    ) -> &Self {
        //
        let next_command = use_state(&cx, || None);

        {
            to_owned![next_command, self.receiver];
            use_coroutine(&cx, |_rx: UnboundedReceiver<()>| async move {
                loop {
                    if let Ok(command) = receiver.recv().await {
                        log::debug!("command {:?}", command);
                        next_command.set(Some(command));
                    }
                }
            });
        }

        if let Some(command) = &**next_command {
            next_command.set(None);
            handler(command);
        }

        self
    }
}
impl<TCommand: std::fmt::Debug> PartialEq for CommandBus<TCommand> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
impl<TCommand: std::fmt::Debug> Clone for CommandBus<TCommand> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            receiver: self.receiver.clone(),
        }
    }
}
