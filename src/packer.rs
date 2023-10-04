use regex::Regex;

pub struct BitDescriptor {
    value: u64,
    number_of_bits: u64,
}

impl BitDescriptor {
    pub fn new(value: u64, number_of_bits: u64) -> Result<Self, &'static str> {
        if !(number_of_bits > 0) {
            return Err("There cannot be 0 number of bits");
        }

        Ok(Self {
            value,
            number_of_bits,
        })
    }

    pub fn from_string(string_value: &str) -> Result<Self, &'static str> {
        let re = Regex::new("^(0|1)+$").unwrap();
        if !re.is_match(string_value) {
            return Err("Not a valid binary number");
        }

        Ok(Self {
            value: u64::from_str_radix(string_value, 2).unwrap(),
            number_of_bits: string_value.len() as u64,
        })
    }
}
pub struct BitPacker;

impl BitPacker {
    pub fn pack(bit_descriptors: Vec<BitDescriptor>) -> Result<Vec<u8>, &'static str> {
        let value_bit_size = bit_descriptors
            .iter()
            .map(|e| e.number_of_bits)
            .reduce(|acc, e| acc + e)
            .unwrap();

        let num_of_bytes = ((value_bit_size + 7) >> 3) as usize;
        let mut buffer: Vec<u8> = vec![0; num_of_bytes];

        BitPacker::pack_into_buffer(bit_descriptors, &mut buffer);
        Ok(buffer)
    }

    fn pack_into_buffer(bit_descriptors: Vec<BitDescriptor>, buffer: &mut Vec<u8>) {
        let mut index = 0;
        let mut bit_index: i64 = 7;
        let buffer_size = buffer.len();

        for bit_desc in bit_descriptors {
            let value = bit_desc.value;
            let mut bits_to_pack = bit_desc.number_of_bits;

            while bits_to_pack > 0 && index < buffer_size {
                let empty_space = (bit_index + 1) as u64;
                if bits_to_pack <= empty_space {
                    let mask = (1 << bits_to_pack) - 1;
                    buffer[index] |= ((value as u8) & mask) << (empty_space - bits_to_pack);

                    bit_index -= bits_to_pack as i64;
                    if bit_index == -1 {
                        bit_index = 7;
                        index += 1;
                    }

                    bits_to_pack = 0;
                } else {
                    let mask = ((1 << empty_space) - 1) << (bits_to_pack - empty_space) as u64;
                    buffer[index] |= ((value & mask) >> (bits_to_pack - empty_space)) as u8;
                    bit_index = 7;
                    index += 1;
                    bits_to_pack -= empty_space;
                }
            }
        }
    }
}
