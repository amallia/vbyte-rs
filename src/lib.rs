pub fn vbyte_encode(n: u32, output: &mut Vec<u8>) {
    let mut num = n;
    while num >= 128 {
        output.push((num & 0x7F) as u8 | 0x80);
        num >>= 7;
    }
    output.push(num as u8);
}

pub fn vbyte_decode(bytes: &[u8]) -> Option<u32> {
    let mut n = 0u32;
    let mut shift = 0;
    for &byte in bytes {
        if shift > 24 {
            return None; // Overflow
        }
        n |= ((byte & 0x7F) as u32) << shift;
        if byte & 0x80 == 0 {
            return Some(n);
        }
        shift += 7;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vbyte_encode_single_byte() {
        let mut output = Vec::new();
        vbyte_encode(5, &mut output);
        assert_eq!(output, vec![5]);

        output.clear();
        vbyte_encode(127, &mut output);
        assert_eq!(output, vec![127]);
    }

    #[test]
    fn test_vbyte_encode_two_bytes() {
        let mut output = Vec::new();
        vbyte_encode(128, &mut output);
        assert_eq!(output, vec![0x80, 0x01]);

        output.clear();
        vbyte_encode(16383, &mut output);
        assert_eq!(output, vec![0xFF, 0x7F]);
    }

    #[test]
    fn test_vbyte_encode_three_bytes() {
        let mut output = Vec::new();
        vbyte_encode(16384, &mut output);
        assert_eq!(output, vec![0x80, 0x80, 0x01]);

        output.clear();
        vbyte_encode(2097151, &mut output);
        assert_eq!(output, vec![0xFF, 0xFF, 0x7F]);
    }

    #[test]
    fn test_vbyte_encode_four_bytes() {
        let mut output = Vec::new();
        vbyte_encode(2097152, &mut output);
        assert_eq!(output, vec![0x80, 0x80, 0x80, 0x01]);

        output.clear();
        vbyte_encode(268435455, &mut output);
        assert_eq!(output, vec![0xFF, 0xFF, 0xFF, 0x7F]);
    }

    #[test]
    fn test_vbyte_encode_five_bytes() {
        let mut output = Vec::new();
        vbyte_encode(268435456, &mut output);
        assert_eq!(output, vec![0x80, 0x80, 0x80, 0x80, 0x01]);

        output.clear();
        vbyte_encode(u32::MAX, &mut output);
        assert_eq!(output, vec![0xFF, 0xFF, 0xFF, 0xFF, 0x0F]);
    }
}
