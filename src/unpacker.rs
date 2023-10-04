#![allow(dead_code)]

pub struct Unpacker<F>
where
    F: Fn(String) -> Option<String>,
{
    buffer: Vec<u8>,
    unpack_fn: F,
    index: usize,
    bit_index: i8,
    pattern: String,
}

impl<F> Unpacker<F>
where
    F: Fn(String) -> Option<String>,
{
    pub fn new(buffer: Vec<u8>, unpack_fn: F) -> Self {
        Unpacker {
            buffer,
            unpack_fn,
            index: 0,
            bit_index: 7,
            pattern: String::new(),
        }
    }
}

impl<F> Iterator for Unpacker<F>
where
    F: Fn(String) -> Option<String>,
{
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.buffer.len() {
            return None;
        }

        let bit =
            (self.buffer[self.index] & (1 << (self.bit_index as u8))) >> (self.bit_index as u8);

        self.bit_index -= 1;
        self.pattern.push_str(&format!("{:01b}", bit));
        let pat = self.pattern.clone();

        if self.bit_index == -1 {
            self.bit_index = 7;
            self.index += 1;
        }

        let transformed_bit_value = (self.unpack_fn)(pat.to_string());

        if let Some(val) = transformed_bit_value {
            if val != "".to_string() {
                self.pattern = String::new();
            }
            return Some(val.to_string());
        }

        None
    }
}
