

pub fn serialize_i64(dest: &mut [u8], mut value: i64) {
    if dest.len() != 4 {
        panic!("Slice of length 4 expected");
    }
    if value < 0 {
        panic!("Only positive i64 supported");
    }

    dest[3] = (value & 0xff) as u8;
    value >>= 8;
    dest[2] = (value & 0xff) as u8;
    value >>= 8;
    dest[1] = (value & 0xff) as u8;
    value >>= 8;
    dest[0] = (value & 0xff) as u8;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_i64() {
        let mut arr = [0u8; 8];
        serialize_i64(&mut arr[2..6], 0x01020304);
        assert_eq!(arr, [0, 0, 1, 2, 3, 4, 0, 0]);
    }
}