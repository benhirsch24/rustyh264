struct H264NalReader {
    data: Vec<u8>,
    size: usize,
    next_byte: u8,
    byte_cache: u32,
    bits_in_cache: u32,
    pos: usize,
}

impl H264NalReader {
    pub fn new(data: Vec<u8>, pos: usize) {
        H264NalReader {
            data: data,
            size: data.length(),
            next_byte: 0xFF,
            cache: 0xFF,
            bits_in_cache: 0,
            pos: pos
        }
    }

    /// Meant to update the cache before reading any bits so
    /// that the nal parser can ensure there are at least nbits bits
    /// in the cache for reading.
    fn read_update(&mut self, u32 nbits) -> bool {
        if nbits > 32 {
            println!("Wanted to read {} bits but can read no more than 32",
                     nbits);
            return false;
        }
        if nbits > self.bits_in_cache + (self.size - self.pos) * 8 {
            println!("Wanted to read {} bits but there's only {} left",
                     nbits, self.bits_in_cache + (self.size - self.pos) * 8);
            return false;
        }
        let mut ret : u32 = 0;
        let mut check_three_byte = true;
        while self.bits_in_cache < nbits {
            let byte = self.data[pos++];
            if check_three_byte && byte == 0x03 && self.next_byte == 0x00 && (self.cache & 0xFF == 0x00) {
                // This is an emulation byte, don't check the next byte even if it's 0x03
                self.num_epb++;
                check_three_byte = false;
            } else {
                check_three_byte = true;
                // push next byte into the cache
                self.cache = (self.cache << 8) | self.next_byte;
                self.next_byte = byte;
                self.bits_in_cache += 8;
            }
        }
        true
    }

    /// Reads nbits from the cache and then returns that as a u8
    pub fn read_u8(&mut self, nbits: usize) -> Option<u8> {
        if !self.read_update(self, nbits) {
            return None;
        }
        if nbits > 8 {
            println!("Tried to read {} bits but can only read 8", nbits);
            return None;
        }
        let mut val : u8 = 0;
        let shift = self.bits_in_cache - nbits;
        val = self.next_byte >> shift;
        val |= self.cache << (8 - shift);
        if nbits > 8 {
            val &= (0x01 << nbits) - 1;
        }
        self.bits_in_cache = shift;
        Some(val)
    }

    /// Reads nbits from the cache and then returns that as a u16
    pub fn read_u16(&mut self, nbits: usize) -> Option<u16> {
        if !self.read_update(self, nbits) {
            return None;
        }
        if nbits > 16 {
            println!("Tried to read {} bits but can only read 8", nbits);
            return None;
        }
        let mut val : u16 = 0;
        let shift = self.bits_in_cache - nbits;
        val = self.next_byte >> shift;
        val |= self.cache << (8 - shift);
        if nbits > 8 {
            val &= (0x01 << nbits) - 1;
        }
        self.bits_in_cache = shift;
        Some(val)
    }

    /// Reads nbits from the cache and then returns that as a u32
    pub fn read_u32(&mut self, nbits: usize) -> Option<u32> {
        if !self.read_update(self, nbits) {
            return None;
        }
        if nbits > 32 {
            println!("Tried to read {} bits but can only read 8", nbits);
            return None;
        }
        let mut val : u32 = 0;
        let shift = self.bits_in_cache - nbits;
        val = self.next_byte >> shift;
        val |= self.cache << (8 - shift);
        if nbits > 8 {
            val &= (0x01 << nbits) - 1;
        }
        self.bits_in_cache = shift;
        Some(val)
    }

    pub fn read_ue(&mut self, nbits: usize) -> Option<u32> {
        let mut leadingZeros = -1;
        let bit = self.read_u32(1);
        while bit == 0 {
        }
    }
}
