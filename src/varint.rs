
/// Decode varint and return (varint, bytes_read)
/// https://carlmastrangelo.com/blog/lets-make-a-varint
/// Varint: a way of compressing down ints into smaller space than normally needed.
/// Trade-off of varint
///     - spend more bits on larger numbers, fewer bits on smaller number
///     - e.g. a 64 bit integer that is almost always less than 256 would be wasting the top 56 bits of a fixed width representation
pub fn decode_varint(bytes: &[u8]) -> (i64, usize) {
    let mut varint = 0;
    let mut bytes_read = 0;

    for (i, byte) in bytes.iter().enumerate().take(9) {
        bytes_read += 1;

        if i == 8 {
            varint = (varint << 8) | *byte as i64;
            break;
        } else {
            varint = (varint << 7) | (*byte & 0b0111_1111) as i64;
            if *byte < 0b1000_0000 {
                break;
            }
        }
    }

    (varint, bytes_read)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_one_byte_varint() {
        // tuple.2 is number of bytes read
        assert_eq!(decode_varint(&[0b0000_0001]), (1, 1));
        assert_eq!(decode_varint(&[0b0000_0011]), (3, 1));
        // 27 = 16 + 8 + 3 + 1
        // 16   0001 0000
        // 8    0000 1000
        // 2    0000 0010
        // 1    0000 0001
        // -- sum bit by bit
        // 27   0001 1011
        assert_eq!(decode_varint(&[0b0001_1011]), (27, 1));
        // we can encode i64 127 with only 1 byte instead of 8 bytes normally for i64
        assert_eq!(decode_varint(&[0b0111_1111]), (127, 1));
    }

    #[test]
    fn read_two_byte_varint() {
        assert_eq!(decode_varint(&[0b1000_0001, 0b0000_0000]), (128, 2));
        assert_eq!(decode_varint(&[0b1000_0001, 0b0000_0001]), (129, 2));
        assert_eq!(decode_varint(&[0b1000_0001, 0b0111_1111]), (255, 2));
    }

    #[test]
    fn read_nine_byte_varint() {
        assert_eq!(decode_varint(&vec![0xff; 9]), (-1, 9));
    }

    #[test]
    fn read_varint_from_longer_bytes() {
        assert_eq!(decode_varint(&vec![0x01; 10]), (1, 1));
        assert_eq!(decode_varint(&vec![0xff; 10]), (-1, 9));
    }
}