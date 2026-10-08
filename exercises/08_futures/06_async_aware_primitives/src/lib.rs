/// Can you understand the sequence of events that can lead to a deadlock?
use tokio::sync::mpsc;

pub struct Message {
    payload: String,
    response_channel: mpsc::Sender<Message>,
}

/// Replies with `pong` to any message it receives, setting up a new
/// channel to continue communicating with the caller.
pub async fn pong(mut receiver: mpsc::Receiver<Message>) {
    loop {
        let msg = match receiver.recv().await {
            Some(msg) => msg,
            None => continue,
        };

        println!("Pong received: {}", msg.payload);

        let (sender, new_receiver) = mpsc::channel(1);
        msg.response_channel
            .send(Message {
                payload: "pong".into(),
                response_channel: sender,
            })
            .await.expect("");

        receiver = new_receiver;
    }
}

#[cfg(test)]
mod tests {
    use crate::{pong, Message};
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn ping() {
        let (sender, receiver) = mpsc::channel(1);
        let (response_sender, mut response_receiver) = mpsc::channel(1);
        sender
            .send(Message {
                payload: "pong".into(),
                response_channel: response_sender,
            }).await.expect("");

        tokio::spawn(pong(receiver));

        let answer = response_receiver.recv().await.unwrap().payload;
        assert_eq!(answer, "pong");
    }
}
