use crate::imagestorage::image_storage_client::ImageStorageClient;
use crate::imagestorage::{MessageIdentifier, Size};
use std::time::{Duration, Instant};
use tonic::transport::Channel;

mod imagestorage;

pub struct MessageResponse {
    text: String,
}

pub struct ImageResponse {
    image: Vec<u8>,
}

impl MessageResponse {
    pub fn new(text: String) -> MessageResponse {
        MessageResponse { text }
    }
    pub fn text(&self) -> String {
        self.text.clone()
    }
}

impl ImageResponse {
    pub fn new(image: Vec<u8>) -> ImageResponse {
        ImageResponse { image }
    }

    pub fn image(&self) -> Vec<u8> {
        self.image.clone()
    }
}

#[tokio::main]
async fn main() {
    let mut client = ImageStorageClient::connect("http://localhost:50051")
        .await
        .unwrap();

    retrieve_message(&mut client).await;

    // retrieve_image(&mut client, "Small".to_string()).await;
    // retrieve_image(&mut client, "Medium".to_string()).await;
    retrieve_image(&mut client, "Large".to_string()).await;
    // retrieve_image(&mut client, "Original".to_string()).await;
}

async fn retrieve_message(client: &mut ImageStorageClient<Channel>) {
    let message_request = MessageIdentifier { id: "".into() };

    let mut min_message_time: Duration = Duration::from_millis(u64::MAX);
    let mut max_message_time: Duration = Duration::from_millis(u64::MIN);
    let mut total_message_time: Duration = Duration::from_millis(u64::MIN);

    for _ in 0..100 {
        let start = Instant::now();
        client
            .get_message(message_request.clone())
            .await
            .unwrap()
            .into_inner();
        let message_time = start.elapsed();

        total_message_time += message_time;
        if message_time < min_message_time {
            min_message_time = message_time;
        }
        if message_time > max_message_time {
            max_message_time = message_time;
        }
    }

    println!("Message times: ");
    println!("Min Message time: {:?}", min_message_time);
    println!("Max Message time: {:?}", max_message_time);
    println!("Avg Message time: {:?}", total_message_time / 100);
}

async fn retrieve_image(client: &mut ImageStorageClient<Channel>, size: String) {
    let image_request = Size {
        size: size.clone().into(),
    };

    let mut min_image_time: Duration = Duration::from_millis(u64::MAX);
    let mut max_image_time: Duration = Duration::from_millis(u64::MIN);
    let mut total_image_time: Duration = Duration::from_millis(u64::MIN);

    println!("Image size: {}", size);

    for _ in 0..100 {
        let start = Instant::now();
        client
            .get_image(image_request.clone())
            .await
            .unwrap()
            .into_inner();
        let image_time = start.elapsed();

        total_image_time += image_time;
        if image_time < min_image_time {
            min_image_time = image_time;
        }
        if image_time > max_image_time {
            max_image_time = image_time;
        }
    }

    println!("Image times: ");
    println!("Min Image time: {:?}", min_image_time);
    println!("Max Image time: {:?}", max_image_time);
    println!("Avg Image time: {:?}", total_image_time / 100);
}
