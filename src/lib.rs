use gloo::{console::log, events::EventListener};
use rand::{thread_rng, Rng};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Blob, Request, RequestInit, Response, Url};

#[wasm_bindgen(start)]
pub async fn main() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let clear_btn = document.get_element_by_id("clear-all").unwrap();
    let on_click_clear = EventListener::new(&clear_btn, "click", move |_| {
        document
            .get_element_by_id("images")
            .unwrap()
            .set_inner_html("");
    });

    on_click_clear.forget();
}

#[wasm_bindgen]
pub async fn generate(link_length: i32) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let generate_btn = document.get_element_by_id("button").unwrap();
    let spinner = document.get_element_by_id("loading").unwrap();
    let images = document.get_element_by_id("images").unwrap();

    log!("Starting image search");
    spinner.remove_attribute("hidden").unwrap();
    generate_btn.set_attribute("disabled", "").unwrap();

    loop {
        log!("Generating...");
        if let Ok(r) = fetch_image(link_length).await {
            log!("Found image");
            
            let img = document.create_element("img").unwrap();
            img.set_attribute("src", &r).unwrap();
            images.prepend_with_node_1(&img).unwrap();
            break;
        }
    }

    spinner.set_attribute("hidden", "").unwrap();
    generate_btn.remove_attribute("disabled").unwrap();
}

async fn fetch_image(link_length: i32) -> Result<String, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");

    let window = web_sys::window().unwrap();

    let request = Request::new_with_str_and_init(&generate_code(link_length, "https://i.imgur.com/"), &opts)?;
    let response_raw = JsFuture::from(window.fetch_with_request(&request)).await?;
    let response: Response = response_raw.clone().dyn_into()?;

    let blob: Blob = JsFuture::from(response.blob().unwrap()).await?.dyn_into()?;

    log!(&response.url());
    if response.url() == "https://i.imgur.com/removed.png" {
        Err(JsValue::from_str("Image removed"))
    } else {
        Ok(Url::create_object_url_with_blob(&blob).unwrap())
    }
}

fn generate_code(length: i32, site: &str) -> String {
    let mut rng = thread_rng();

    const CHAR_POOL: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    let code: String = (0..length)
        .map(|_| CHAR_POOL[rng.gen_range(0..CHAR_POOL.len())] as char)
        .collect();

    format!("{}{}.jpg", site, code)
}