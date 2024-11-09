use std::collections::LinkedList;
use alfred_rs::connection::{Receiver, Sender};
use alfred_rs::error::Error;
use alfred_rs::AlfredModule;
use alfred_rs::message::{Message, MessageType};
use alfred_rs::tokio;
use scanf::scanf;

const MODULE_NAME: &str = "console";
const INPUT_TOPIC: &str = "console";

#[tokio::main]
#[allow(clippy::print_stdout)]
async fn main() -> Result<(), Error> {
    let module = AlfredModule::new(MODULE_NAME).await?;
    let mut publisher = module.connection.publisher;
    let mut subscriber = module.connection.subscriber;
    subscriber.listen(INPUT_TOPIC).await?;
    tokio::spawn(async move {
        async move {
            loop {
                let mut topic = String::new();
                let mut text = String::new();
                let mut response_topics = String::new();
                println!("Insert message to send to a specific topic (topic: text): ");
                if scanf!("{}: {}", topic, text).is_err() { continue; }
                println!("Insert responseTopics separated by whitespace (blank if none): ");
                if scanf!("{}", response_topics).is_err() { continue; }
                let response_topics = if response_topics.is_empty() {
                    LinkedList::from([INPUT_TOPIC.to_string()])
                } else {
                    response_topics.split(' ').map(String::from).collect()
                };
                let message = Message {
                    text: text.clone(),
                    message_type: MessageType::Text,
                    response_topics,
                    ..Message::default()
                };
                if publisher.send(topic.as_str(), &message).await.is_ok() {
                    println!(" > {topic}: {text}");
                } else {
                    println!(" * Error sending message \"{text}\" to {topic}");
                }
            }
        }.await;
    });
    loop {
        let (topic, message) = subscriber.receive().await?;
        match message.message_type {
            MessageType::Text => {
                println!(" < {}: {}", topic, message.text);
            },
            MessageType::Unknown | MessageType::Audio | MessageType::Photo | MessageType::ModuleInfo => {
                println!(" < {}[{}]: {}", topic, message.message_type, message.text);
            }
        }

    }
}
