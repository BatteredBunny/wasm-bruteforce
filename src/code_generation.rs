use std::str;

use rand::distributions::Alphanumeric;
use rand::rngs::ThreadRng;
use rand::Rng;

use crate::sites;

pub struct CodeGenerator {
    rng: ThreadRng,
    output: Vec<u8>,
    code_length: usize,
}

impl CodeGenerator {
    pub fn new(site: &sites::Sites, code_length: usize) -> Self {
        match *site {
            sites::Sites::Imgur => CodeGenerator {
                rng: rand::thread_rng(),
                output: format!("https://i.imgur.com/{:code_length$}.jpg", 0).into_bytes(),
                code_length,
            },
            sites::Sites::Lightshot => CodeGenerator {
                rng: rand::thread_rng(),
                output: format!("https://prnt.sc/{:code_length$}", 0).into_bytes(),
                code_length,
            },
        }
    }

    pub fn generate(&mut self) -> &str {
        for c in self.output.iter_mut().skip(20).take(self.code_length) {
            *c = self.rng.sample(Alphanumeric);
        }

        str::from_utf8(&self.output).unwrap()
    }
}
