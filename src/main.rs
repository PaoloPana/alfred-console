use std::collections::LinkedList;
use alfred_rs::connection::{Publisher, Subscriber};
use alfred_rs::error::Error;
use alfred_rs::message::{Message, MessageType};
use alfred_rs::module::Module;
use alfred_rs::tokio;
use scanf::scanf;

const MODULE_NAME: &'static str = "console";
const INPUT_TOPIC: &'static str = "console";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let module = Module::new(MODULE_NAME.to_string()).await?;
    let mut publisher = module.connection.publisher;
    let mut subscriber = module.connection.subscriber;
    subscriber.subscribe(INPUT_TOPIC.to_string()).await?;
    tokio::spawn(async move {
        async move {
            loop {
                let mut topic = String::new();
                let mut text = String::new();
                println!("Insert message to send to a specific topic (topic: text): ");
                if scanf!("{}: {}", topic, text).is_err() { continue; }
                let mut message = Message::empty();
                message.text = text.clone();
                message.message_type = MessageType::TEXT;
                message.response_topics = LinkedList::from([INPUT_TOPIC.to_string()]);
                match publisher.publish(topic.clone(), &message).await.is_ok() {
                    true => println!(" > {topic}: {text}"),
                    false => println!(" * Error sending message \"{text}\" to {topic}")
                }
            }
        }.await;
    });
    loop {
        let (topic, message) = subscriber.get_message().await?;
        match message.message_type {
            MessageType::TEXT => {
                println!(" < {}: {}", topic, message.text)
            },
            _ => {
                println!(" < {}[{}]: {}", topic, message.message_type, message.text)
            }
        }

    }
}
