use std::{
    cell::RefCell,
    error::Error,
    fmt::{Display, Formatter},
};

use gloo::{console, file::ObjectUrl};
use wasm_bindgen::JsCast;
use web_sys::{
    Document, Element, HtmlAnchorElement, HtmlButtonElement, HtmlImageElement, HtmlInputElement,
    HtmlSelectElement,
};

use crate::{
    site::{self, FetchError},
    sites::Sites,
};

#[derive(Debug, Clone)]
pub enum GeneratorError {
    NoElementFound,
    ConvertFailed,
}
impl Error for GeneratorError {}

impl From<Element> for GeneratorError {
    fn from(_: Element) -> Self {
        Self::ConvertFailed
    }
}

impl Display for GeneratorError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            GeneratorError::NoElementFound => writeln!(f, "Couldn't find element!"),
            GeneratorError::ConvertFailed => writeln!(f, "Failed to convert element to sub type"),
        }
    }
}

pub struct Generator {
    pub should_stop_now: RefCell<bool>,
    pub site: RefCell<site::Site>,
    blobs: RefCell<Vec<ObjectUrl>>,

    document: Document,

    spinner: Element,
    images: Element,
    clear_btn: HtmlButtonElement,
    generate_btn: HtmlButtonElement,
    generate_ten_btn: HtmlButtonElement,
    link_length_input: HtmlInputElement,
    site_selection: HtmlSelectElement,
    stop_btn: HtmlButtonElement,
}

impl Generator {
    pub fn new() -> Result<Self, GeneratorError> {
        let document = gloo::utils::document();
        Ok(Self {
            should_stop_now: RefCell::new(false),
            site: RefCell::new(site::Site::new(Sites::Imgur, Sites::Imgur.default_length())),
            blobs: RefCell::new(Vec::new()),

            spinner: document
                .get_element_by_id("spinner")
                .ok_or(GeneratorError::NoElementFound)?,
            images: document
                .get_element_by_id("images")
                .ok_or(GeneratorError::NoElementFound)?,
            clear_btn: document
                .get_element_by_id("clear-all")
                .ok_or(GeneratorError::NoElementFound)?
                .dyn_into()?,
            generate_btn: document
                .get_element_by_id("generate")
                .ok_or(GeneratorError::NoElementFound)?
                .dyn_into()?,
            generate_ten_btn: document
                .get_element_by_id("generate-ten")
                .ok_or(GeneratorError::NoElementFound)?
                .dyn_into()?,
            link_length_input: document
                .get_element_by_id("link-length")
                .ok_or(GeneratorError::NoElementFound)?
                .dyn_into()?,
            site_selection: document
                .get_element_by_id("site-selection")
                .ok_or(GeneratorError::NoElementFound)?
                .dyn_into()?,
            stop_btn: document
                .get_element_by_id("stop")
                .ok_or(GeneratorError::NoElementFound)?
                .dyn_into()?,
            document,
        })
    }

    pub fn clear_blobs(&self) {
        *self.blobs.borrow_mut() = Vec::new()
    }

    pub async fn generate(&self, amount: i32) {
        *self.should_stop_now.borrow_mut() = false;

        self.spinner.set_class_name("");

        self.clear_btn.set_disabled(true);
        self.generate_btn.set_disabled(true);
        self.generate_ten_btn.set_disabled(true);
        self.link_length_input.set_disabled(true);
        self.site_selection.set_disabled(true);
        self.stop_btn.set_disabled(false);

        console::log!(format!(
            "Starting search for {} images of {} length",
            amount,
            self.site.borrow().code_length
        ));

        let mut found: u16 = 0;
        while found < amount as u16 && !*self.should_stop_now.borrow(){
            match self.site.borrow_mut().fetch().await {
                Ok(res) => {
                    console::log!(format!(
                        "Found {}/{}: {}",
                        found + 1,
                        amount,
                        &res.origin_url
                    ));

                    let a: HtmlAnchorElement = self
                        .document
                        .create_element("a")
                        .unwrap()
                        .dyn_into()
                        .unwrap();
                    a.set_href(&res.origin_url);

                    let img: HtmlImageElement = self
                        .document
                        .create_element("img")
                        .unwrap()
                        .dyn_into()
                        .unwrap();
                    img.set_class_name("generated_image");
                    img.set_src(&res.blob);

                    a.append_child(&img).unwrap();

                    self.images.prepend_with_node_1(&a).unwrap();
                    found += 1;
                    self.blobs.borrow_mut().push(res.blob);
                }
                Err(e) => {
                    if let FetchError::InvalidImage = e {
                    } else {
                        console::log!(format!("Error: {e}"));
                        *self.should_stop_now.borrow_mut() = true;
                    }
                }
            };
        }

        self.spinner.set_class_name("hidden");
        self.clear_btn.set_disabled(false);
        self.generate_btn.set_disabled(false);
        self.generate_ten_btn.set_disabled(false);
        self.link_length_input.set_disabled(false);
        self.site_selection.set_disabled(false);
        self.stop_btn.set_disabled(true);

        console::log!("Finished image search");
    }
}
