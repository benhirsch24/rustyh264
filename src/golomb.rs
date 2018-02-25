pub fn read_bit(byte: u8, offset: usize) -> u8 {
    let pos = 7 - offset;
    let mask = 0x01 << pos;
    let bit = byte & mask;
    bit >> pos
}

pub fn golombdec(data: &[u8]) -> u64 {
    let mut idx = 0;
    let mut byte = data[idx];
    let mut leadingZeroBits : i64 = -1;
    let mut b;
    let mut offset = 0; // from the left
    loop {
        b = read_bit(byte, offset);
        if b == 1 {
            break;
        }
        leadingZeroBits += 1;
        if offset == 7 {
            offset = 0;
            idx += 1;
            byte = data[idx];
        } else {
            offset += 1;
        }
    }
    println!("Leading zeros: {}", leadingZeroBits);

    let mut leadingZeroIntoNum : u64 = 0;
    idx = 0;
    byte = data[idx];
    offset = 0;
    for _ in 0..leadingZeroBits {
        b = read_bit(byte, offset);
        leadingZeroIntoNum |= b as u64;
        leadingZeroIntoNum = leadingZeroIntoNum << 1;
        if offset == 7 {
            offset = 0;
            idx += 1;
            byte = data[idx];
        } else {
            offset += 1;
        }
    }

    2u64.pow(leadingZeroBits as u32) - 1 + leadingZeroIntoNum
}
