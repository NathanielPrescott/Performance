use wasm_bindgen::prelude::wasm_bindgen;

pub mod proto {
    tonic::include_proto!("imagestorage");
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn get_message() -> String {
    log("get_message, imagestorage-wasm!");
    "get_message".to_string()
}

#[wasm_bindgen]
pub fn get_image() -> String {
    log("get_image, imagestorage-wasm!");
    "get_image".to_string()
}
