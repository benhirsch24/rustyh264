use std::fs::File;
use std::io::Read;
pub mod h264nalreader;
pub mod types;
pub use types::{H264NalUnit, H264NalUnitType, H264NalUnitSPS, H264NalFormat, H264NalUnitIDR, H264NalUnitPPS};

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
    pub format: H264NalFormat
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
            format: H264NalFormat::UNKNOWN
        })
    }

    pub fn parse_sps(&mut self, offset: usize) -> Result<H264NalUnitSPS, H264NalParseError> {
        let mut reader = h264nalreader::H264NalReader::new(&self.data[offset+1..]);
        let mut unit = H264NalUnitSPS::new();
        unit.profile_idc = reader.read_u8(8).unwrap();
        {
            unit.constraint_0_flag = reader.read_u8(1).unwrap();
            unit.constraint_1_flag = reader.read_u8(1).unwrap();
            unit.constraint_2_flag = reader.read_u8(1).unwrap();
            unit.constraint_3_flag = reader.read_u8(1).unwrap();
            unit.constraint_4_flag = reader.read_u8(1).unwrap();
            unit.constraint_5_flag = reader.read_u8(1).unwrap();
            reader.read_u8(2); // reserved 0 bits
        }
        unit.level_idc = reader.read_u8(8).unwrap();
        unit.seq_parameter_set_id = reader.read_ue().unwrap();

        // depending on the profile we parse various other flags.
        let certain_profiles : Vec<u8> = vec![100, 110, 122, 244, 44, 83, 86, 118, 128, 138, 139, 134, 135];
        if certain_profiles.contains(&unit.profile_idc) {
            unit.chroma_format_idc = reader.read_ue().unwrap();
            if unit.chroma_format_idc == 3 {
                unit.separate_colour_plane_flag = reader.read_u8(1).unwrap();
            }
            unit.bit_depth_luma_minus8 = reader.read_ue().unwrap();
            unit.bit_depth_chroma_minus8 = reader.read_ue().unwrap();
            unit.qpprime_y_zero_transform_bypass_flag = reader.read_u8(1).unwrap();
            unit.seq_scaling_matrix_present_flag = reader.read_u8(1).unwrap();
            if unit.seq_scaling_matrix_present_flag == 1 {
                let scaling_lists = if unit.chroma_format_idc != 3 { 8 } else { 12 };
                unit.seq_scaling_list_present_flag.reserve(scaling_lists);
                for i in 0..scaling_lists {
                    unit.seq_scaling_list_present_flag[i] = reader.read_u8(1).unwrap();
                    if unit.seq_scaling_list_present_flag[i] == 1 {
                        if i < 6 {
                        } else {
                        }
                    }
                }
            }
        }

        unit.log2_max_frame_num_minus4 = reader.read_ue().unwrap();
        unit.pic_order_cnt_type = reader.read_ue().unwrap();
        if unit.pic_order_cnt_type == 0 {
            unit.log2_max_pic_order_cnt_lsb_minus4 = reader.read_ue().unwrap();
        } else if unit.pic_order_cnt_type == 1 {
            unit.delta_pic_order_always_zero_flag = reader.read_u8(1).unwrap();
            unit.offset_for_non_ref_pic = reader.read_se().unwrap();
            unit.offset_for_top_to_bottom_field = reader.read_se().unwrap();
            unit.num_ref_frames_in_pic_order_cnt_cycle = reader.read_ue().unwrap();
            unit.offset_for_ref_frame.reserve(unit.num_ref_frames_in_pic_order_cnt_cycle as usize);
            for i in 0..unit.num_ref_frames_in_pic_order_cnt_cycle {
                unit.offset_for_ref_frame[i as usize] = reader.read_se().unwrap();
            }
        }
        unit.max_num_ref_frames = reader.read_ue().unwrap();
        unit.gaps_in_frame_num_value_allowed_flag = reader.read_u8(1).unwrap();
        unit.pic_width_in_mbs_minus1 = reader.read_ue().unwrap();
        unit.pic_height_in_map_units_minus1 = reader.read_ue().unwrap();
        unit.frame_mbs_only_flag = reader.read_u8(1).unwrap();
        if unit.frame_mbs_only_flag == 0 {
            unit.mb_adaptive_frame_field_flag = reader.read_u8(1).unwrap();
        }
        unit.direct_8x8_inference_flag = reader.read_u8(1).unwrap();
        unit.frame_cropping_flag = reader.read_u8(1).unwrap();
        if unit.frame_cropping_flag == 1 {
            unit.frame_crop_left_offset = reader.read_ue().unwrap();
            unit.frame_crop_right_offset = reader.read_ue().unwrap();
            unit.frame_crop_top_offset = reader.read_ue().unwrap();
            unit.frame_crop_bottom_offset = reader.read_ue().unwrap();
        }
        unit.vui_parameters_present_flag = reader.read_u8(1).unwrap();

        Ok(unit)
    }

    pub fn parse_idr(&self, offset: usize) -> Result<H264NalUnitIDR, H264NalParseError> {
        let mut reader = h264nalreader::H264NalReader::new(&self.data[offset+1..]);
        let first_mb_in_slice = reader.read_ue().unwrap();
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
        cursor += 1; // header byte
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
