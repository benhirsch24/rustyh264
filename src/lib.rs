use std::fs::File;
mod golomb;
pub mod h264nalreader;

#[derive(Debug)]
pub enum H264NalParseError {
    NotEnoughBytes,
    StartCodeParseError,
    UnknownFormat,
    Unimplemented,
    GenericParseError
}


pub struct H264NalParser {
    data: Vec<u8>,
    size: usize,
    pos: usize,
    pub format: H264NalFormat
}

pub enum H264NalFormat {
    BYTESTREAM, AVC, UNKNOWN
}

#[derive(Debug)]
pub enum H264NalUnitType {
    SPS,
    PPS,
    IDR,
    P,
    UNKNOWN
}

#[derive(Debug)]
pub struct H264NalUnitSPS {
    pub profile_idc: u8,
    pub constraints: Vec<u8>,
    pub level_idc: u8,
    pub seq_parameter_set_id: u32
}

#[derive(Debug)]
pub struct H264NalUnitPPS {
}

#[derive(Debug)]
pub struct H264NalUnitIDR {
    pub first_mb_in_slice: u64
}

#[derive(Debug)]
pub struct H264NalUnit {
    pub name: String,
    pub sc_offset: usize,
    pub data_offset: usize,
    pub size: usize,

    /* H264 Nal Unit Fields */
    pub nal_ref_idc: u8,
    pub nal_unit_type_num: u8,
    pub nal_unit_type: H264NalUnitType
}

impl H264NalUnit {
    pub fn new(sc_offset: usize,
               data_offset: usize,
               size: usize,
               ref_idc: u8,
               unit_type: u8) -> H264NalUnit
    {
        let nal_unit_type = match unit_type {
            5 => H264NalUnitType::IDR,
            7 => H264NalUnitType::SPS,
            8 => H264NalUnitType::PPS,
            _ => H264NalUnitType::UNKNOWN
        };
        H264NalUnit {
            name: "Unit".to_string(),
            sc_offset: sc_offset,
            data_offset: data_offset,
            size: size,
            nal_ref_idc: ref_idc,
            nal_unit_type_num: unit_type,
            nal_unit_type: nal_unit_type
        }
    }
}

macro_rules! check_size {
    ( $self: expr, $offset: expr, $length: expr ) => {
        if $self.size < $offset + $length {
            return Err(H264NalParseError::NotEnoughBytes);
        }
    }
}

impl H264NalParser {
    pub fn new(path: &str) -> std::io::Result<H264NalParser> {
        let mut file = File::open(path)?;
        let mut data = Vec::new();
        let size = file.read_to_end(&mut data)?;
        Ok(H264NalParser {
            data: data,
            size: size,
            pos: 0,
            format: H264NalFormat::UNKNOWN
        })
    }

    pub fn parse_sps(&mut self, offset: usize) -> Result<H264NalUnitSPS, H264NalParseError> {
        let reader = h264nalreader::H264NalReader::new(self.data[self.pos..], self.pos);
        let profile_idc = reader.read_u8(8);
        {
            let constraint_0 = reader.read_u8(1).unwrap();
            let constraint_1 = reader.read_u8(1).unwrap();
            let constraint_2 = reader.read_u8(1).unwrap();
            let constraint_3 = reader.read_u8(1).unwrap();
            let constraint_4 = reader.read_u8(1).unwrap();
            let constraint_5 = reader.read_u8(1).unwrap();
            reader.read_u8(2); // reserved 0 bits
        }
        let level_idc = reader.read_u8(8);

        let sps_id = reader.read_golomb();
        Err(H264NalParseError::Unimplemented)
    }

    pub fn parse_idr(&self, offset: usize) -> Result<H264NalUnitIDR, H264NalParseError> {
        let first_mb_in_slice = golomb::golombdec(&self.data[offset..]);
        Ok(H264NalUnitIDR{ first_mb_in_slice: first_mb_in_slice })
    }

    fn parse_startcode(&self, sc_offset: usize) -> Result<usize, H264NalParseError> {
        check_size!(self, sc_offset, 3);
        if self.data[sc_offset] != 0 || self.data[sc_offset+1] != 0 {
            return Err(H264NalParseError::StartCodeParseError);
        }
        if self.data[sc_offset+2] == 1 {
            return Ok(3);
        }
        check_size!(self, sc_offset, 4);
        if self.data[sc_offset+2] == 0 && self.data[sc_offset+3] == 1 {
            return Ok(4);
        }
        Err(H264NalParseError::StartCodeParseError)
    }

    fn parse_bytestream(&self, sc_offset: usize) -> Result<H264NalUnit, H264NalParseError> {
        let sc_size = match self.parse_startcode(sc_offset) {
            Ok(sc_size) => sc_size,
            Err(e) => return Err(e)
        };
        let data_offset = sc_offset + sc_size;

        let mut cursor = 0;
        let byte = self.data[data_offset];
        if (byte & 0x80) == 0x80 {
            return Err(H264NalParseError::GenericParseError);
        }
        let ref_idc = (byte & 0x60) >> 5;
        let unit_type = byte & 0x1F;
        cursor += 1;
        let mut header_bytes = 1;
        cursor += 1;
        let mut size = self.size - data_offset - sc_size;
        for i in (cursor + data_offset)..self.size {
            if self.size - i < 3 {
                break;
            }
            match self.parse_startcode(i) {
                Ok(_)  => {
                    println!("i: {} sc_offset: {}", i, sc_offset);
                    size = i - sc_offset;
                    break;
                }
                Err(_) => {}
            }
        }

        Ok(H264NalUnit::new(sc_offset, data_offset, size, ref_idc, unit_type))
    }

    fn parse_avc(&self, sc_offset: usize) -> Result<H264NalUnit, H264NalParseError> {
        check_size!(self, sc_offset, 4);
        Err(H264NalParseError::UnknownFormat)
    }

    pub fn parse_nalunit(&mut self, offset: usize) -> Result<H264NalUnit, H264NalParseError> {
        match self.format {
            H264NalFormat::BYTESTREAM => self.parse_bytestream(offset),
            H264NalFormat::AVC => self.parse_avc(offset),
            H264NalFormat::UNKNOWN => {
                match self.parse_bytestream(offset) {
                    Ok(unit) => {
                        self.format = H264NalFormat::BYTESTREAM;
                        Ok(unit)
                    },
                    Err(_) => match self.parse_avc(offset) {
                        Ok(unit) => {
                            self.format = H264NalFormat::AVC;
                            Ok(unit)
                        },
                        Err(_) => Err(H264NalParseError::UnknownFormat)
                    }
                }
            }
        }
    }
}
