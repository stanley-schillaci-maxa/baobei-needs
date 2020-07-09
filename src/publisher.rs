//! A simple pub-sub for broadcasting messages.

use crossbeam_channel::{unbounded, Receiver, Sender};

/// Publisher that can broadcast messages to subscribers.
pub struct Publisher<TMessage> {
    /// Senders to open channels.
    senders: Vec<Sender<TMessage>>,
}

impl<TMessage: Clone> Publisher<TMessage> {
    /// Creates a publisher with no subscriber.
    pub fn new() -> Self {
        Self {
            senders: Vec::new(),
        }
    }

    /// Creates a channel and returns the related receiver.
    pub fn subscribe(&mut self) -> Receiver<TMessage> {
        let (sender, receiver) = unbounded();
        self.senders.push(sender);
        receiver
    }

    /// Broadcasts a clone of the `message` to all subscribers.
    pub fn publish(&mut self, message: TMessage) {
        self.senders
            .retain(|sender| sender.try_send(message.clone()).is_ok());
        //   ^ removes failing channels, mainly disconnected ones.
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossbeam_channel::TryRecvError;

    #[test]
    fn it_works() {
        let mut bus = Publisher::new();

        let receiver1 = bus.subscribe();
        let receiver2 = bus.subscribe();

        bus.publish(12);

        assert_eq!(receiver1.recv(), Ok(12));
        assert_eq!(receiver2.recv(), Ok(12));

        assert_eq!(receiver1.try_recv(), Err(TryRecvError::Empty));
        assert_eq!(receiver2.try_recv(), Err(TryRecvError::Empty));

        drop(receiver1);

        bus.publish(62);

        assert_eq!(receiver2.recv(), Ok(62));
        assert_eq!(receiver2.try_recv(), Err(TryRecvError::Empty));
    }
}
