#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main() {
    // hex to rgb
    let hex_input = "FF0000FF00FF00FF"; 

    // parse hex
    let mut data_buf = [0u8; 16];
    let data_size = parse_hex_fixed(hex_input, &mut data_buf);

    // convert to grayscale
    let mut grayscale = rgb_to_grayscale(&data_buf[..data_size]);

    // apply brightness offset, for demonstration
    let brightness_offset = 50;
    apply_brightness(&mut grayscale, brightness_offset);

}

// convert hex to str
fn parse_hex_fixed(hex_str: &str, buf: &mut [u8]) -> usize {
    let mut idx = 0;
    let bytes = hex_str.as_bytes();
    for chunk in bytes.chunks(2) {
        if idx >= buf.len() {
            break;
        }
        if let Ok(s) = core::str::from_utf8(chunk) {
            if let Ok(val) = u8::from_str_radix(s, 16) {
                buf[idx] = val;
                idx += 1;
            }
        }
    }
    idx
}

// convert RGB data to grayscale, returns a fixed-size array [u8; 256]
fn rgb_to_grayscale(data: &[u8]) -> [u8; 256] {
    let mut result = [0u8; 256];
    let mut idx = 0;

    for chunk in data.chunks(3) {
        if chunk.len() < 3 || idx >= result.len() {
            break;
        }

        let mut gray_val = 
            (77  * chunk[0] as i32 +
             150 * chunk[1] as i32 +
             29  * chunk[2] as i32) >> 8;
        
        // clamp to [0,255]
        if gray_val < 0 {
            gray_val = 0;
        } else if gray_val > 255 {
            gray_val = 255;
        }

        result[idx] = gray_val as u8;
        idx += 1;
    }

    result
}

// add a brightness offset to the grayscale image
fn apply_brightness(grayscale: &mut [u8], brightness_offset: i16) {
    for px in grayscale.iter_mut() {
        let val = (*px as i16) + brightness_offset;
        // clamp to [0,255]
        let val_clamped = if val < 0 {
            0
        } else if val > 255 {
            255
        } else {
            val as u8
        };
        *px = val_clamped as u8;
    }
}
