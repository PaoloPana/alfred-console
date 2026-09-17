use std::collections::LinkedList;
use alfred_core::error::Error;
use alfred_core::AlfredModule;
use alfred_core::message::{Message, MessageType};
use alfred_core::tokio;
use scanf::scanf;

const MODULE_NAME: &str = "console";
const INPUT_TOPIC: &str = "console";

fn get_message_type(type_id: i8) -> MessageType {
    match type_id {
        1 => MessageType::Text,
        2 => MessageType::Audio,
        3 => MessageType::Photo,
        4 => MessageType::ModuleInfo,
        _ => MessageType::Unknown,
    }
}

#[tokio::main]
#[allow(clippy::print_stdout)]
async fn main() -> Result<(), Error> {
    let mut module = AlfredModule::new(MODULE_NAME, env!("CARGO_PKG_VERSION")).await?;
    module.listen(INPUT_TOPIC).await?;
    let publisher = module.connection.clone();
    let subscriber = module.connection.clone();
    tokio::spawn(async move {
        async move {
            loop {
                let mut message_type_id = 0;
                let mut topic = String::new();
                let mut text = String::new();
                let mut response_topics = String::new();
                println!("Insert the message type (1: Text, 2: Audio, 3: Photo, 4: ModuleInfo): ");
                if scanf!("{}", message_type_id).is_err() { continue; }
                let message_type = get_message_type(message_type_id);
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
                    message_type: message_type.clone(),
                    response_topics,
                    ..Message::default()
                };
                if publisher.send(topic.as_str(), &message).await.is_ok() {
                    println!(" > [{message_type:?}] {topic}: {text}");
                } else {
                    println!(" * Error sending message \"{text}\" to {topic}");
                }
            }
        }.await;
    });
    loop {
        let (topic, message) = subscriber.receive_all().await?;
        match message.message_type {
            MessageType::Text => {
                println!(" < {}: {}", topic, message.text);
            },
            _ => {
                println!(" < {}[{}]: {}", topic, message.message_type, message.text);
            }
        }
        message.params.iter().for_each(|(k, v)| {
            println!("   - {k}: {v}");
        });
    }
}
