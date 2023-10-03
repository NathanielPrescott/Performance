use crate::imagestorage::{MessageIdentifier, Size};
use imagestorage::image_storage_client::ImageStorageClient;
use prost::Message;
use tonic_web_wasm_client::Client;
use wasm_bindgen::prelude::*;

mod imagestorage;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct MessageResponse {
    text: String,
    size: u32,
}

#[wasm_bindgen]
pub struct ImageResponse {
    image: Vec<u8>,
    size: u32,
}

#[wasm_bindgen]
impl MessageResponse {
    #[wasm_bindgen(constructor)]
    pub fn new(text: String, size: u32) -> MessageResponse {
        MessageResponse { text, size }
    }

    #[wasm_bindgen(getter)]
    pub fn text(&self) -> String {
        self.text.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn size(&self) -> u32 {
        self.size
    }
}

#[wasm_bindgen]
impl ImageResponse {
    #[wasm_bindgen(constructor)]
    pub fn new(image: Vec<u8>, size: u32) -> ImageResponse {
        ImageResponse { image, size }
    }

    #[wasm_bindgen(getter)]
    pub fn image(&self) -> Vec<u8> {
        self.image.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn size(&self) -> u32 {
        self.size
    }
}

fn build_client() -> ImageStorageClient<Client> {
    let base_url = "http://localhost:50051".to_string();
    let wasm_client = Client::new(base_url);

    ImageStorageClient::new(wasm_client)
}

#[wasm_bindgen]
pub async fn get_message() -> MessageResponse {
    let mut client = build_client();
    let request = MessageIdentifier { id: "".into() };

    let response = client.get_message(request).await;

    let statement = &response.unwrap().into_inner().clone();
    let mut buf = Vec::new();
    statement.clone().encode(&mut buf).unwrap();
    let size_in_bits = buf.len();

    MessageResponse {
        text: statement.clone().text,
        size: size_in_bits as u32,
    }
}

#[wasm_bindgen]
pub async fn get_image(image_size: String) -> ImageResponse {
    let mut client = build_client();
    let request = Size {
        size: image_size.clone(),
    };

    log(&format!("Requesting image of size: {}", image_size.clone()));

    let response = client.get_image(request).await;

    log(&format!(
        "Received image of size: {}",
        response.iter().clone().len()
    ));

    let statement = &response.unwrap().into_inner().clone();
    let mut buf = Vec::new();
    statement.clone().encode(&mut buf).unwrap();
    let size_in_bits = buf.len();

    ImageResponse {
        image: statement.clone().image,
        size: size_in_bits as u32,
    }
}
