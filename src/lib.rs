use gloo::{console::log, events::EventListener};
use wasm_bindgen::{JsCast, prelude::*};
use wasm_bindgen_futures::{JsFuture, spawn_local};
use web_sys::{Blob, Request, RequestInit, RequestRedirect, Response, Url, Window};

use crate::code_generation::CodeGenerator;

mod code_generation;

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

    let on_click_generate = EventListener::new(&generate_btn, "click", move |_| {
        log!("Starting image search");
        spawn_local(generate(1));
        log!("Finished image search");
    });

    let on_click_generate_ten = EventListener::new(&generate_ten_btn, "click", move |_| {
        log!("Starting image search");
        spawn_local(generate(10));
        log!("Finished image search");
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

    spinner.remove_attribute("hidden").unwrap();

    clear_btn.set_attribute("disabled", "").unwrap();
    generate_btn.set_attribute("disabled", "").unwrap();
    generate_ten_btn.set_attribute("disabled", "").unwrap();
    link_length_input.set_attribute("disabled", "").unwrap();

    let mut generator = CodeGenerator::new(link_length as usize);

    let mut opts = RequestInit::new();
    opts.method("GET").redirect(RequestRedirect::Manual);

    let mut found: u16 = 0;
    while found < amount as u16 {
        if let Ok(res) = fetch(&opts, &window, &mut generator).await {
            log!("Found image", &res.origin_url);

            let a = document.create_element("a").unwrap();
            a.set_attribute("href", &res.origin_url).unwrap();

            let img = document.create_element("img").unwrap();
            img.set_class_name("generated_image");
            img.set_attribute("src", &res.blob).unwrap();

            a.append_child(&img).unwrap();

            images.prepend_with_node_1(&a).unwrap();
            found += 1;
        }
    }

    spinner.set_attribute("hidden", "").unwrap();

    clear_btn.remove_attribute("disabled").unwrap();
    generate_btn.remove_attribute("disabled").unwrap();
    generate_ten_btn.remove_attribute("disabled").unwrap();
    link_length_input.remove_attribute("disabled").unwrap();
}

struct FetchResult {
    blob: String,
    origin_url: String,
}

async fn fetch(request_options: &RequestInit, window: &Window, generator: &mut CodeGenerator) -> Result<FetchResult, JsValue> {
    let request = Request::new_with_str_and_init(generator.generate(), request_options)?;
    let response_raw = JsFuture::from(window.fetch_with_request(&request)).await?;
    let response: Response = response_raw.clone().dyn_into()?;

    match response.status() {
        429 => {
            Err(JsValue::from("Rate limited by imgur"))
        },
        302 => {
            Err(JsValue::from("Invalid image"))
        }
        200 => {
            let blob: Blob = JsFuture::from(response.blob().unwrap()).await?.dyn_into()?;

            Ok(FetchResult {
                blob: Url::create_object_url_with_blob(&blob).unwrap(),
                origin_url: response.url(),
            })
        }
        _ => Err(JsValue::from("Unexpected")),
    }
}