use gloo::{console::log, events::EventListener};
use rand::{thread_rng, Rng};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::{JsFuture, spawn_local};
use web_sys::{Blob, Request, RequestInit, Response, Url};

#[wasm_bindgen(start)]
pub async fn main() {   
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let clear_btn = document.get_element_by_id("clear_all").unwrap();
    let generate_btn = document.get_element_by_id("generate").unwrap();
    let generate_ten_btn = document.get_element_by_id("generate_ten").unwrap();

    let on_click_clear = EventListener::new(&clear_btn, "click", move |_| {
        document.get_element_by_id("images").unwrap().set_inner_html("");
    });

    let on_click_generate = EventListener::new(&generate_btn, "click", move |_|  {
        spawn_local(generate(1));
    });

    let on_click_generate_ten = EventListener::new(&generate_ten_btn, "click", move |_| {
        spawn_local(generate(10));
    });

    on_click_clear.forget();
    on_click_generate.forget();
    on_click_generate_ten.forget();
}

async fn generate(amount: i32) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let spinner = document.get_element_by_id("loading").unwrap();
    let images = document.get_element_by_id("images").unwrap();
    let clear_btn = document.get_element_by_id("clear_all").unwrap();
    let generate_btn = document.get_element_by_id("generate").unwrap();
    let generate_ten_btn = document.get_element_by_id("generate_ten").unwrap();
    let link_length_input = document.get_element_by_id("link_length").unwrap();
    let link_length: i32 = link_length_input.get_attribute("value").unwrap().parse().unwrap();

    log!("Starting image search");
    spinner.remove_attribute("hidden").unwrap();

    clear_btn.set_attribute("disabled", "").unwrap();
    generate_btn.set_attribute("disabled", "").unwrap();
    generate_ten_btn.set_attribute("disabled", "").unwrap();
    link_length_input.set_attribute("disabled", "").unwrap();

    let mut counter = 0;
    while amount > counter {
        log!("Generating...");
        if let Ok((blob, source)) = fetch_image(link_length).await {
            log!("Found image");

            let a = document.create_element("a").unwrap();
            a.set_attribute("href", &source).unwrap();

            let img = document.create_element("img").unwrap();
            img.set_class_name("generated_image");
            img.set_attribute("src", &blob).unwrap();

            a.append_child(&img).unwrap();

            images.prepend_with_node_1(&a).unwrap();
            counter += 1;
        }   
    }

    spinner.set_attribute("hidden", "").unwrap();

    clear_btn.remove_attribute("disabled").unwrap();
    generate_btn.remove_attribute("disabled").unwrap();
    generate_ten_btn.remove_attribute("disabled").unwrap();
    link_length_input.remove_attribute("disabled").unwrap();
}

async fn fetch_image(link_length: i32) -> Result<(String, String), JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");

    let window = web_sys::window().unwrap();

    let request = Request::new_with_str_and_init(&generate_code(link_length, "https://i.imgur.com/"), &opts)?;
    let response_raw = JsFuture::from(window.fetch_with_request(&request)).await?;
    let response: Response = response_raw.clone().dyn_into()?;

    let blob: Blob = JsFuture::from(response.blob().unwrap()).await?.dyn_into()?;

    let url = response.url();
    log!(&url);

    if &url == "https://i.imgur.com/removed.png" {
        Err(JsValue::from_str("Image removed"))
    } else {
        Ok(
            (Url::create_object_url_with_blob(&blob).unwrap(), url)
        )
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