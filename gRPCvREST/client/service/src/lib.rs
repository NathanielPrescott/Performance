use crate::imagestorage::{Image, MessageIdentifier};
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
    image: Image,
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
    let size_in_bits = buf.len() * 8;

    MessageResponse {
        text: statement.clone().text,
        size: size_in_bits as u32,
    }
}

#[wasm_bindgen]
pub fn get_image() -> String {
    log("get_image, imagestorage-wasm!");
    "get_image".to_string()
}
