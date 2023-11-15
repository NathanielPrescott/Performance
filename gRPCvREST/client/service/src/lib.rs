use crate::imagestorage::{MessageIdentifier, Size};
use imagestorage::image_storage_client::ImageStorageClient;
use tonic::codegen::tokio_stream::StreamExt;
use tonic_web_wasm_client::Client;
use wasm_bindgen::prelude::*;

mod imagestorage;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = window, js_name = current_time)]
    fn current_time() -> f64;
}

#[wasm_bindgen]
pub struct MessageResponse {
    text: String,
}

#[wasm_bindgen]
pub struct ImageResponse {
    image: js_sys::Uint8Array,
}

#[wasm_bindgen]
impl MessageResponse {
    #[wasm_bindgen(constructor)]
    pub fn new(text: String) -> MessageResponse {
        MessageResponse { text }
    }
    #[wasm_bindgen(getter)]
    pub fn text(&self) -> String {
        self.text.clone()
    }
}

#[wasm_bindgen]
impl ImageResponse {
    #[wasm_bindgen(constructor)]
    pub fn new(image: js_sys::Uint8Array) -> ImageResponse {
        ImageResponse { image }
    }

    #[wasm_bindgen(getter)]
    pub fn image(&self) -> js_sys::Uint8Array {
        self.image.clone()
    }
}

fn build_client() -> ImageStorageClient<Client> {
    let base_url = "http://localhost:50051".to_string();
    let wasm_client = Client::new(base_url);

    ImageStorageClient::new(wasm_client).max_decoding_message_size(1024 * 1024 * 50)
}

#[wasm_bindgen]
pub async fn get_message() -> String {
    let mut client = build_client();
    let request = MessageIdentifier { id: "".into() };

    let response = client.get_message(request).await;

    response.unwrap().into_inner().text
}

#[wasm_bindgen]
pub async fn get_image(image_size: String) -> ImageResponse {
    let mut client = build_client();
    let request = Size {
        size: image_size.clone(),
    };

    let mut image_stream = client.get_image(request).await.unwrap().into_inner();

    let mut image_data = Vec::new();
    while let Some(image_response) = image_stream.next().await {
        let image_chunk = image_response.unwrap().image;
        image_data.extend_from_slice(&image_chunk);
    }
    ImageResponse {
        image: js_sys::Uint8Array::from(&image_data[..]),
    }
}
