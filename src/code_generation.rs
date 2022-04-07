use std::str;

use rand::distributions::Alphanumeric;
use rand::Rng;
use rand::rngs::ThreadRng;

pub struct CodeGenerator {
    rng: ThreadRng,
    output: Vec<u8>,
    code_length: usize,
}

impl CodeGenerator {
    pub fn new(code_length: usize) -> Self {
        CodeGenerator {
            rng: rand::thread_rng(),
            output: format!("https://i.imgur.com/{:code_length$}.jpg", 0).into_bytes(),
            code_length,
        }
    }

    pub fn generate(&mut self) -> &str {
        for c in self.output.iter_mut().skip(20).take(self.code_length) {
            *c = self.rng.sample(Alphanumeric);
        }

        str::from_utf8(&self.output).unwrap()
    }
}