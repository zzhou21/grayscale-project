#[no_mangle]
pub extern "C" fn process_hex_image(hex_input: *const u8, len: usize) -> i32 {
   let hex_str = unsafe {
       if hex_input.is_null() {
           return -1;
       }
       std::slice::from_raw_parts(hex_input, len)
   };

   let data: Vec<u8> = hex_str
       .chunks(2)
       .filter_map(|chunk| {
           let hex_byte = std::str::from_utf8(chunk).ok()?;
           u8::from_str_radix(hex_byte, 16).ok()
       })
       .collect();

   let width = 100;
   let height = 100;
   let _grayscale = rgb_to_grayscale(&data, width, height);

   0
}

fn rgb_to_grayscale(data: &[u8], width: u32, height: u32) -> Vec<u8> {
   let mut result = Vec::with_capacity((width * height) as usize);

   for chunk in data.chunks(3) {
       if chunk.len() == 3 {
           let gray = ((chunk[0] as f32 * 0.299) +
                      (chunk[1] as f32 * 0.587) +
                      (chunk[2] as f32 * 0.114)) as u8;
           result.push(gray);
       }
   }

   result
}