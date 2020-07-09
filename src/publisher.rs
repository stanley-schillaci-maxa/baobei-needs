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

    #[derive(Clone, Debug, PartialEq)]
    struct Message(i32);

    #[test]
    fn test_multiple_subscribers() {
        let mut publisher = Publisher::new();

        let receiver1 = publisher.subscribe();
        let receiver2 = publisher.subscribe();

        publisher.publish(Message(12));
        publisher.publish(Message(34));

        assert_eq!(receiver1.recv(), Ok(Message(12)));
        assert_eq!(receiver1.recv(), Ok(Message(34)));
        assert_eq!(receiver1.try_recv(), Err(TryRecvError::Empty));

        assert_eq!(receiver2.recv(), Ok(Message(12)));
        assert_eq!(receiver2.recv(), Ok(Message(34)));
        assert_eq!(receiver2.try_recv(), Err(TryRecvError::Empty));

        publisher.publish(Message(45));

        assert_eq!(receiver1.recv(), Ok(Message(45)));
        assert_eq!(receiver1.try_recv(), Err(TryRecvError::Empty));

        assert_eq!(receiver2.recv(), Ok(Message(45)));
        assert_eq!(receiver2.try_recv(), Err(TryRecvError::Empty));
    }

    #[test]
    fn test_disconnected_channels() {
        let mut publisher = Publisher::new();

        let receiver = publisher.subscribe();
        let disconnected_receiver = publisher.subscribe();

        drop(disconnected_receiver);

        publisher.publish(62);

        assert_eq!(receiver.recv(), Ok(62));
        assert_eq!(receiver.try_recv(), Err(TryRecvError::Empty));
    }
}
