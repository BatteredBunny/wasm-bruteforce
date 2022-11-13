use std::{
    cell::RefCell,
    error::Error,
    fmt::{Display, Formatter},
};

use gloo::console;
use wasm_bindgen::JsCast;
use web_sys::{
    Document, Element, HtmlAnchorElement, HtmlButtonElement, HtmlImageElement, HtmlInputElement,
};

use crate::{site::{self, FetchError}, sites::Sites};

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
    pub site: RefCell<site::Site>,

    document: Document,

    spinner: Element,
    images: Element,
    clear_btn: HtmlButtonElement,
    generate_btn: HtmlButtonElement,
    generate_ten_btn: HtmlButtonElement,
    link_length_input: HtmlInputElement,
}

impl Generator {
    pub fn new() -> Result<Self, GeneratorError> {
        let document = gloo::utils::document();
        Ok(Self {
            site: RefCell::new(site::Site::new(
                Sites::Imgur,
            Sites::Imgur.default_length(),
            )),

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
            document,
        })
    }

    pub async fn generate(&self, amount: i32) {
        self.spinner.set_class_name("");

        self.clear_btn.set_disabled(true);
        self.generate_btn.set_disabled(true);
        self.generate_ten_btn.set_disabled(true);
        self.link_length_input.set_disabled(true);

        console::log!(format!(
            "Starting search for {} images of {} length",
            amount,
            self.site.borrow().code_length
        ));

        let mut found: u16 = 0;
        'image: while found < amount as u16 {
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
                }
                Err(e) => {
                    if let FetchError::InvalidImage = e {
                    } else {
                        console::log!(format!("Error: {e}"));
                        break 'image;
                    }
                }
            };
        }

        self.spinner.set_class_name("hidden");
        self.clear_btn.set_disabled(false);
        self.generate_btn.set_disabled(false);
        self.generate_ten_btn.set_disabled(false);
        self.link_length_input.set_disabled(false);

        console::log!("Finished image search");
    }
}
