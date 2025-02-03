#[no_mangle]
pub extern "C" fn _start() {
    // 1) Example hex input (3 pixels: R=FF0000, G=00FF00, B=0000FF)
    let hex_input = "FF000000FF000000FF";
    
    // 2) Parse the string into a vector of RGB bytes
    let data = parse_hex(hex_input);

    // 3) Convert the RGB data to grayscale
    let grayscale = rgb_to_grayscale(&data, 3, 1);

    // 4) Print the result (will require WAMR to have printing support)
    println!("Original RGB data: {:?}", data);
    println!("Grayscale output: {:?}", grayscale);
}

/// Parses a hexadecimal string (e.g. "FF0000") into a vector of bytes.
fn parse_hex(hex_str: &str) -> Vec<u8> {
    hex_str
        .as_bytes()
        .chunks(2)
        .filter_map(|chunk| {
            let hex_byte = std::str::from_utf8(chunk).ok()?;
            u8::from_str_radix(hex_byte, 16).ok()
        })
        .collect()
}

/// Converts a slice of RGB triplets into grayscale values using the classic formula:
///   Gray = 0.299 * R + 0.587 * G + 0.114 * B
fn rgb_to_grayscale(data: &[u8], width: u32, height: u32) -> Vec<u8> {
    let mut result = Vec::with_capacity((width * height) as usize);

    for pixel in data.chunks(3) {
        if pixel.len() == 3 {
            let gray_val = (pixel[0] as f32 * 0.299)
                + (pixel[1] as f32 * 0.587)
                + (pixel[2] as f32 * 0.114);
            result.push(gray_val as u8);
        }
    }

    result
}
