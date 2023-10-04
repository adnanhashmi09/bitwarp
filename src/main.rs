use bitwarp::packer::{BitDescriptor, BitPacker};
use bitwarp::unpacker::Unpacker;

fn main() {
    let bit_descriptors = vec![
        BitDescriptor::from_string("101").unwrap(),
        BitDescriptor::from_string("00001000").unwrap(),
        BitDescriptor::from_string("1111111111").unwrap(),
        BitDescriptor::from_string("0").unwrap(),
        BitDescriptor::from_string("111").unwrap(),
    ];

    let packed = BitPacker::pack(bit_descriptors).unwrap();
    let bin_strings = packed
        .iter()
        .map(|e| format!("{:08b}", e))
        .collect::<Vec<String>>();
    println!("{:#?}", bin_strings);

    let unpacker = Unpacker::new(packed.clone(), &move |s: String| -> Option<String> {
        Some(s)
    });
    let unpacked_buffer = unpacker.map(|e| e).collect::<String>();
    println!("{:#?}", unpacked_buffer);

    let unpacker = Unpacker::new(packed.clone(), &move |s: String| -> Option<String> {
        // println!("{:#?}", s);
        match s.as_str() {
            "101" => Some("2".to_string()),
            "00001000" => Some("3".to_string()),
            "1111111111" => Some("4".to_string()),
            "0" => Some("0".to_string()),
            "111" => Some("5".to_string()),
            _ => Some("".to_string()),
        }
    });
    let unpacked_buffer = unpacker.map(|e| e).collect::<String>();
    println!("{:#?}", unpacked_buffer);
}
