pub struct H264NalReader<'a> {
    data: &'a[u8],
    size: usize,
    next_byte: u8,
    cache: u32,
    bits_in_cache: u32,
    pos: usize,
    num_epb: u32,
}

impl<'a> H264NalReader<'a> {
    pub fn new(data: &'a[u8]) -> H264NalReader<'a> {
        H264NalReader {
            data: data,
            size: data.len(),
            next_byte: 0xFF,
            cache: 0xFF,
            bits_in_cache: 0,
            pos: 0,
            num_epb: 0,
        }
    }

    /// Meant to update the cache before reading any bits so
    /// that the nal parser can ensure there are at least nbits bits
    /// in the cache for reading.
    fn read_update(&mut self, nbits : u32) -> bool {
        if nbits > 32 {
            println!("Wanted to read {} bits but can read no more than 32",
                     nbits);
            return false;
        }
        if nbits > self.bits_in_cache + (self.size - self.pos) as u32 * 8 {
            println!("Wanted to read {} bits but there's only {} left",
                     nbits, self.bits_in_cache + (self.size - self.pos) as u32 * 8);
            return false;
        }
        let mut check_three_byte = true;
        while self.bits_in_cache < nbits {
            let byte = self.data[self.pos];
            self.pos += 1;
            if check_three_byte && byte == 0x03 && self.next_byte == 0x00 && (self.cache & 0xFF == 0x00) {
                // This is an emulation byte, don't check the next byte even if it's 0x03
                self.num_epb += 1;
                check_three_byte = false;
            } else {
                check_three_byte = true;
                // push next byte into the cache
                self.cache = (self.cache << 8) | self.next_byte as u32;
                self.next_byte = byte;
                self.bits_in_cache += 8;
            }
        }
        true
    }

    /// Reads 1 bit from the cache and returns it as a boolean.
    pub fn read_flag(&mut self) -> Option<bool> {
        self.read_u8(1).map(|v| if v == 1 { true } else { false })
    }

    /// Reads nbits from the cache and then returns that as a u8.
    /// The cache is self.cache and self.next_byte. The first 8 bits
    /// of the cache are in self.next_byte, and the rest are in self.cache.
    pub fn read_u8(&mut self, nbits: u32) -> Option<u8> {
        if nbits > 8 {
            println!("Tried to read {} bits but can only read 8", nbits);
            return None;
        }
        self.read_u32(nbits).map(|v| v as u8)
    }

    /// Reads nbits from the cache and then returns that as a u16.
    /// The cache is self.cache and self.next_byte. The first 8 bits
    /// of the cache are in self.next_byte, and the rest are in self.cache.
    pub fn read_u16(&mut self, nbits: u32) -> Option<u16> {
        if nbits > 16 {
            println!("Tried to read {} bits but can only read 8", nbits);
            return None;
        }
        self.read_u32(nbits).map(|v| v as u16)
    }

    /// Reads nbits from the cache and then returns that as a u32.
    /// The cache is self.cache and self.next_byte. The first 8 bits
    /// of the cache are in self.next_byte, and the rest are in self.cache.
    pub fn read_u32(&mut self, nbits: u32) -> Option<u32> {
        if nbits > 32 {
            println!("Tried to read {} bits but can only read 8", nbits);
            return None;
        }
        if !self.read_update(nbits) {
            return None;
        }
        let shift = self.bits_in_cache - nbits;
        let mut val : u32 = self.next_byte as u32 >> shift;
        let nooverflow_cache = self.cache & ((0x01 << (7 - shift)) - 1);
        val |= nooverflow_cache << (8 - shift);
        let mask = if nbits == 32 {
            0xFFFFFFFF
        } else {
            (0x01 << nbits) - 1
        };
        val = val & mask;
        self.bits_in_cache = shift;
        Some(val)
    }

    pub fn read_ue(&mut self) -> Option<u32> {
        let mut leading_zeros = 0;
        let mut bit = match self.read_u8(1) {
            None => return None,
            Some(b) => b
        };
        while bit == 0 {
            leading_zeros += 1;
            bit = match self.read_u8(1) {
                None => return None,
                Some(b) => b
            };
        }
        if leading_zeros > 32 {
            println!("Reading UE and leading zeros > 32: {}", leading_zeros);
            return None;
        }
        let val = match self.read_u32(leading_zeros) {
            None => return None,
            Some(v) => v
        };
        Some((1 << leading_zeros) - 1 + val)
    }

    pub fn read_se(&mut self) -> Option<i32> {
        let ue = match self.read_ue() {
            None => return None,
            Some(v) => v
        };
        Some(
            if ue % 2 == 1 {
                (ue as i32 / 2) + 1
            } else {
                -(ue as i32 / 2)
            })
    }
}
