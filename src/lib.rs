use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::Generator;
use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use web_sys::HtmlSelectElement;

mod code_generation;
mod generator;
mod site;
mod sites;

#[wasm_bindgen]
pub async fn run() {
    let document = gloo::utils::document();
    let link_length_input = Rc::new(
        document
            .get_element_by_id("link-length")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap(),
    );
    let link_length_input2 = Rc::clone(&link_length_input);
    let link_length_input3 = Rc::clone(&link_length_input);
    let site_selection = Rc::new(
        document
            .get_element_by_id("site-selection")
            .unwrap()
            .dyn_into::<HtmlSelectElement>()
            .unwrap(),
    );
    let site_selection2 = Rc::clone(&site_selection);
    let clear_btn = document.get_element_by_id("clear-all").unwrap();
    let images = document.get_element_by_id("images").unwrap();
    let generate_btn = document.get_element_by_id("generate").unwrap();
    let generate_ten_btn = document.get_element_by_id("generate-ten").unwrap();
    let stop_btn = document.get_element_by_id("stop").unwrap();

    let generator = Rc::new(Generator::new().unwrap());
    let generator2 = Rc::clone(&generator);
    let generator3 = Rc::clone(&generator);
    let generator4 = Rc::clone(&generator);
    let generator5 = Rc::clone(&generator);
    let generator6 = Rc::clone(&generator);

    let input_state = Rc::new(RefCell::new(generator.site.borrow().code_length));
    let input_state2 = Rc::clone(&input_state);

    // Emergency stop button!
    EventListener::new(&stop_btn, "click", move |_| {
        *generator6.should_stop_now.borrow_mut() = true;
    }).forget();

    // Choosing a site will put default link length in input
    EventListener::new(&site_selection, "change", move |_| {
        let new_site = site_selection2.value().parse::<sites::Sites>().unwrap();
        let default_value = new_site.default_length();

        generator3.site.borrow_mut().set_site(new_site);
        link_length_input3.set_value_as_number(default_value);
        *input_state.borrow_mut() = default_value;
    })
    .forget();

    // Link length validator
    EventListener::new(&link_length_input, "input", move |_| {
        let new_length = link_length_input2.value_as_number();

        // If invalid sets previous value
        if new_length.is_nan() {
            link_length_input2.set_value_as_number(*input_state2.borrow());
        } else {
            *input_state2.borrow_mut() = new_length;
            generator4.site.borrow_mut().set_code_length(new_length)
        }
    })
    .forget();

    EventListener::new(&clear_btn, "click", move |_| {
        images.set_inner_html("");
        generator5.clear_blobs();
    })
    .forget();

    EventListener::new(&generate_btn, "click", move |_| {
        let temp_generator = Rc::clone(&generator);
        spawn_local(async move {
            temp_generator.generate(1).await;
        });
    })
    .forget();

    EventListener::new(&generate_ten_btn, "click", move |_| {
        let temp_generator = Rc::clone(&generator2);
        spawn_local(async move {
            temp_generator.generate(10).await;
        });
    })
    .forget();
}
