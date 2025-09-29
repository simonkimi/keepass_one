pub mod writer;

pub fn b64_original_length(base64_str: &str) -> usize {
    let len = base64_str.len();
    let bytes = base64_str.as_bytes();
    let padding = match len {
        0 => 0,
        _ => {
            let last = bytes[len - 1];
            let penultimate = if len > 1 { bytes[len - 2] } else { 0 };
            ((last == b'=') as usize) * (1 + ((penultimate == b'=') as usize))
        }
    };
    (len * 3) / 4 - padding
}
