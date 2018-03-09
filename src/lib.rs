use std::fs::File;
use std::io::Read;
mod h264nalreader;
use h264nalreader::{H264NalReader};
pub mod types;
pub use types::{H264NalUnit, H264NalUnitType, H264NalUnitSPS, H264NalFormat, H264NalUnitSlice, H264NalUnitPPS,
                H264VUIParameters, H264HDRParameters, EXTENDED_SAR,
                slice_type_is_b_slice, slice_type_is_p_slice, slice_type_is_sp_slice, slice_type_is_i_slice,
                slice_type_is_si_slice, ceil_log2};

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
    pub format: H264NalFormat,

    pub pps: Vec<H264NalUnitPPS>,
    pub sps: Vec<H264NalUnitSPS>,
}

macro_rules! check_size {
    ( $self: expr, $offset: expr, $length: expr ) => {
        if $self.size < $offset + $length {
            return Err(H264NalParseError::NotEnoughBytes);
        }
    }
}

const MAX_SPS_COUNT : usize = 32;
const MAX_PPS_COUNT : usize = 256;

impl H264NalParser {
    pub fn new(path: &str) -> std::io::Result<H264NalParser> {
        let mut file = File::open(path)?;
        let mut data = Vec::new();
        let size = file.read_to_end(&mut data)?;
        let mut pps_vec = Vec::new();
        pps_vec.reserve(MAX_PPS_COUNT);
        let mut sps_vec = Vec::new();
        sps_vec.reserve(MAX_SPS_COUNT);
        Ok(H264NalParser {
            data: data,
            size: size,
            format: H264NalFormat::UNKNOWN,
            pps: pps_vec,
            sps: sps_vec
        })
    }

    // SPS
    pub fn parse_sps(&mut self, offset: usize) -> Result<H264NalUnitSPS, H264NalParseError> {
        let mut reader = H264NalReader::new(&self.data[offset+1..]);
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
                unit.separate_colour_plane_flag = reader.read_flag().unwrap();
            }
            unit.bit_depth_luma_minus8 = reader.read_ue().unwrap();
            unit.bit_depth_chroma_minus8 = reader.read_ue().unwrap();
            unit.qpprime_y_zero_transform_bypass_flag = reader.read_u8(1).unwrap();
            unit.seq_scaling_matrix_present_flag = reader.read_u8(1).unwrap();
            if unit.seq_scaling_matrix_present_flag == 1 {
                let scaling_lists = if unit.chroma_format_idc != 3 { 8 } else { 12 };
                unit.seq_scaling_list_present_flag.reserve(scaling_lists);
                println!("Scaling list parsing not implemented");
                for i in 0..scaling_lists {
                    unit.seq_scaling_list_present_flag[i] = reader.read_u8(1).unwrap();
                    if unit.seq_scaling_list_present_flag[i] == 1 {
                        if i < 6 {
                            // TODO: I should do this
                        } else {
                            // TODO: I should do this
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
            unit.delta_pic_order_always_zero_flag = reader.read_flag().unwrap();
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
        unit.frame_mbs_only_flag = reader.read_flag().unwrap();
        if !unit.frame_mbs_only_flag {
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
        if unit.vui_parameters_present_flag == 1 {
            unit.vui_parameters = Some(self.parse_vui_params(&mut reader));
        }

        println!("SPS cap: {} id: {}", self.sps.capacity(),
            unit.seq_parameter_set_id);
        self.sps.push(unit.clone());
        Ok(unit)
    }

    fn parse_vui_params(&self, mut reader: &mut H264NalReader) -> H264VUIParameters {
        let mut params = H264VUIParameters::new();
        params.aspect_ratio_info_present_flag = reader.read_u8(1).unwrap();
        if params.aspect_ratio_info_present_flag == 1 {
            params.aspect_ratio_idc = reader.read_u8(8).unwrap();
            if params.aspect_ratio_idc == EXTENDED_SAR {
                params.sar_width = reader.read_u16(16).unwrap();
                params.sar_height = reader.read_u16(16).unwrap();
            }
        }
        params.overscan_info_present_flag = reader.read_u8(1).unwrap();
        if params.overscan_info_present_flag == 1 {
            params.overscan_appropriate_flag = reader.read_u8(1).unwrap();
        }
        params.video_signal_type_present_flag = reader.read_u8(1).unwrap();
        if params.video_signal_type_present_flag == 1 {
            params.video_format = reader.read_u8(3).unwrap();
            params.video_full_range_flag = reader.read_u8(1).unwrap();
            params.colour_description_present_flag = reader.read_u8(1).unwrap();
            if params.colour_description_present_flag == 1 {
                params.colour_primaries = reader.read_u8(8).unwrap();
                params.transfer_characteristics = reader.read_u8(8).unwrap();
                params.matrix_coefficients = reader.read_u8(8).unwrap();
            }
        }
        params.chroma_loc_info_present_flag = reader.read_u8(1).unwrap();
        if params.chroma_loc_info_present_flag == 1 {
            params.chroma_sample_loc_type_top_field = reader.read_ue().unwrap();
            params.chroma_sample_loc_type_bottom_field = reader.read_ue().unwrap();
        }
        params.timing_info_present_flag = reader.read_u8(1).unwrap();
        if params.timing_info_present_flag == 1 {
            params.num_units_in_tick = reader.read_u32(32).unwrap();
            params.time_scale = reader.read_u32(32).unwrap();
            params.fixed_frame_rate_flag = reader.read_u8(1).unwrap();
        }
        params.nal_hrd_parameters_present_flag = reader.read_u8(1).unwrap();
        if params.nal_hrd_parameters_present_flag == 1 {
            params.nal_hrd_parameters = Some(self.parse_hdr_params(&mut reader));
        }
        params.vcl_hrd_parameters_present_flag = reader.read_u8(1).unwrap();
        if params.vcl_hrd_parameters_present_flag == 1 {
            params.vcl_hrd_parameters = Some(self.parse_hdr_params(&mut reader));
        }
        if params.nal_hrd_parameters_present_flag == 1 || params.vcl_hrd_parameters_present_flag == 1 {
            params.low_delay_hrd_flag = reader.read_u8(1).unwrap();
        }
        params.pic_struct_present_flag = reader.read_u8(1).unwrap();
        params.bitstream_restriction_flag = reader.read_u8(1).unwrap();
        if params.bitstream_restriction_flag == 1 {
            params.motion_vectors_over_pic_boundaries_flag = reader.read_u8(1).unwrap();
            params.max_bytes_per_pic_denom = reader.read_ue().unwrap();
            params.max_bits_per_mb_denom = reader.read_ue().unwrap();
            params.log2_max_mv_length_horizontal = reader.read_ue().unwrap();
            params.log2_max_mv_length_vertical = reader.read_ue().unwrap();
            params.max_num_reorder_frames = reader.read_ue().unwrap();
            params.max_dec_frame_buffering = reader.read_ue().unwrap();
        }
        params
    }

    pub fn parse_hdr_params(&self, reader: &mut H264NalReader) -> H264HDRParameters {
        let mut hdr_params = H264HDRParameters::new();
        hdr_params.cpb_cnt_minus1 = reader.read_ue().unwrap();
        hdr_params.bit_rate_scale = reader.read_u8(4).unwrap();
        hdr_params.cpb_size_scale = reader.read_u8(4).unwrap();
        let cpb_cnt = (hdr_params.cpb_cnt_minus1 + 1) as usize;
        hdr_params.bit_rate_value_minus1.reserve(cpb_cnt);
        hdr_params.cpb_size_value_minus1.reserve(cpb_cnt);
        hdr_params.cbr_flag.reserve(cpb_cnt);
        for i in 0..cpb_cnt {
            hdr_params.bit_rate_value_minus1[i] = reader.read_ue().unwrap();
            hdr_params.cpb_size_value_minus1[i] = reader.read_ue().unwrap();
            hdr_params.cbr_flag[i] = reader.read_u8(1).unwrap();
        }
        hdr_params.initial_cpb_removal_delay_length_minus1 = reader.read_u8(5).unwrap();
        hdr_params.cpb_removal_delay_length_minus1 = reader.read_u8(5).unwrap();
        hdr_params.dpb_output_delay_length_minus1 = reader.read_u8(5).unwrap();
        hdr_params.time_offset_length = reader.read_u8(5).unwrap();
        hdr_params
    }

    pub fn parse_pps(&mut self, offset: usize) -> Result<H264NalUnitPPS, H264NalParseError> {
        let mut reader = H264NalReader::new(&self.data[offset+1..]);
        let mut pps = H264NalUnitPPS::new();

        pps.pic_parameter_set_id = reader.read_ue().unwrap();
        pps.seq_parameter_set_id = reader.read_ue().unwrap();
        pps.entropy_coding_mode_flag = reader.read_flag().unwrap();
        pps.bottom_field_pic_order_in_frame_present_flag = reader.read_flag().unwrap();
        pps.num_slice_groups_minus1 = reader.read_ue().unwrap();
        if pps.num_slice_groups_minus1 > 0 {
            pps.slice_group_map_type = reader.read_ue().unwrap();
            if pps.slice_group_map_type == 0 {
                let size = pps.num_slice_groups_minus1 as usize + 1;
                pps.run_length_minus1.reserve(size);
                for i in 0..size {
                    pps.run_length_minus1[i] = reader.read_ue().unwrap();
                }
            } else if pps.slice_group_map_type == 2 {
                let size = pps.num_slice_groups_minus1 as usize + 1;
                pps.top_left.reserve(size);
                pps.bottom_right.reserve(size);
                for i in 0..size {
                    pps.top_left[i] = reader.read_ue().unwrap();
                    pps.bottom_right[i] = reader.read_ue().unwrap();
                }
            } else if pps.slice_group_map_type == 3 ||
                        pps.slice_group_map_type == 4 ||
                        pps.slice_group_map_type == 5 {
                pps.slice_group_change_direction_flag = reader.read_u8(1).unwrap();
                pps.slice_group_change_rate_minus1 = reader.read_ue().unwrap();
            } else if pps.slice_group_map_type == 6 {
                pps.pic_size_in_map_units_minus1 = reader.read_ue().unwrap();
                let size = pps.pic_size_in_map_units_minus1 as usize + 1;
                pps.slice_group_id.reserve(size);
                for i in 0..size {
                    pps.slice_group_id[i] = reader.read_ue().unwrap();
                }
            }
        }
        pps.num_ref_idx_l0_default_active_minus1 = reader.read_ue().unwrap();
        pps.num_ref_idx_l1_default_active_minus1 = reader.read_ue().unwrap();
        pps.weighted_pred_flag = reader.read_flag().unwrap();
        pps.weighted_bipred_idc = reader.read_u8(2).unwrap();
        pps.pic_init_qp_minus26 = reader.read_se().unwrap();
        pps.pic_init_qs_minus26 = reader.read_se().unwrap();
        pps.chroma_qp_index_offset = reader.read_se().unwrap();
        pps.deblocking_filter_control_present_flag = reader.read_flag().unwrap();
        pps.constrained_intra_pred_flag = reader.read_u8(1).unwrap();
        pps.redundant_pic_cnt_present_flag = reader.read_flag().unwrap();

        // TODO: if more data
        //transform_8x8_mode_flag: 0,
        //pic_scaling_matrix_present_flag: 0,
        //pic_scaling_list_present_flag: Vec::new(),
        //scaling_list_4x4: vec![vec![0u8; 16]; 6],
        //scaling_list_8x8: vec![vec![0u8; 64]; 6],
        //second_chroma_qp_index_offset: 0

        self.pps.push(pps.clone());
        Ok(pps)
    }

    // Slice
    pub fn parse_slice(&self, offset: usize, nalu: &H264NalUnit) -> Result<H264NalUnitSlice, H264NalParseError> {
        let mut reader = H264NalReader::new(&self.data[offset+1..]);
        let mut slice = H264NalUnitSlice::new();
        // slice_header()
        slice.first_mb_in_slice = reader.read_ue().unwrap();
        slice.slice_type = reader.read_ue().unwrap();
        slice.pic_parameter_set_id = reader.read_ue().unwrap();
        let pps = &self.pps[slice.pic_parameter_set_id as usize];
        let sps = &self.sps[pps.seq_parameter_set_id as usize];
        if sps.separate_colour_plane_flag {
            slice.colour_plane_id = reader.read_u8(2).unwrap();
        }
        let frame_num_bits = sps.log2_max_frame_num_minus4 + 4;
        slice.frame_num = reader.read_u32(frame_num_bits).unwrap();

        if !sps.frame_mbs_only_flag {
            slice.field_pic_flag = reader.read_flag().unwrap();
            if slice.field_pic_flag {
                slice.bottom_field_flag = reader.read_flag().unwrap();
            }
        }
        // if slice pic flag
        if nalu.idr_pic_flag {
            slice.idr_pic_id = reader.read_ue().unwrap();
        }
        if sps.pic_order_cnt_type == 0 {
            slice.pic_order_cnt_lsb = reader.read_u16(sps.log2_max_pic_order_cnt_lsb_minus4 + 4).unwrap();
            if pps.bottom_field_pic_order_in_frame_present_flag &&
                !slice.field_pic_flag {
                    slice.delta_pic_order_cnt_bottom = reader.read_se().unwrap();
                }
        }
        if sps.pic_order_cnt_type == 1 && !sps.delta_pic_order_always_zero_flag {
            match slice.delta_pic_order_cnt.get_mut(0) {
                Some(elem) => *elem = reader.read_se().unwrap(),
                None => println!("WARNING: unable to get mut ref to delta_pic_order_cnt[0]")
            }
            if pps.bottom_field_pic_order_in_frame_present_flag && slice.field_pic_flag {
                match slice.delta_pic_order_cnt.get_mut(1) {
                    Some(elem) => *elem = reader.read_se().unwrap(),
                    None => println!("WARNING: unable to get mut ref to delta_pic_order_cnt[1]")
                }
            }
        }
        if pps.redundant_pic_cnt_present_flag {
            slice.redundant_pic_cnt = reader.read_ue().unwrap();
        }

        // B Slice
        if slice_type_is_b_slice(slice.slice_type) {
            slice.direct_spatial_mv_pred_flag = reader.read_flag().unwrap();
        }

        if slice_type_is_p_slice(slice.slice_type) || slice_type_is_b_slice(slice.slice_type)
            || slice_type_is_sp_slice(slice.slice_type) {
            slice.num_ref_idx_active_override_flag = reader.read_flag().unwrap();
            if slice.num_ref_idx_active_override_flag {
                slice.num_ref_idx_l0_active_minus1 = reader.read_ue().unwrap();
            }
            if slice_type_is_b_slice(slice.slice_type) {
                slice.num_ref_idx_l1_active_minus1 = reader.read_ue().unwrap();
            }
        }

        if nalu.nal_unit_type_num == 20 || nalu.nal_unit_type_num == 21 {
            // ref_pic_list_mvc_modification
            println!("ref_pic_list_mvc_modification");
        } else {
            // ref_pic_list_modification
            println!("ref_pic_list_modification");
        }
        if pps.weighted_pred_flag && (slice_type_is_p_slice(slice.slice_type) || slice_type_is_sp_slice(slice.slice_type)) ||
            (pps.weighted_bipred_idc == 1 && slice_type_is_b_slice(slice.slice_type)) {
            println!("pred_weight_table");
        }
        if nalu.nal_ref_idc != 0 {
            // dec_ref_pic_marking
            if slice_type_is_i_slice(slice.slice_type) {
                slice.no_output_of_prior_pics_flag = reader.read_flag().unwrap();
                slice.long_term_reference_flag = reader.read_flag().unwrap();
            } else {
                slice.adaptive_ref_pic_marking_mode_flag = reader.read_flag().unwrap();
                loop {
                    let mem_op = reader.read_ue().unwrap();
                    if mem_op > 6 || mem_op == 0 {
                        break;
                    }
                    if mem_op == 1 || mem_op == 3 {
                        slice.difference_of_pic_nums_minus1 = reader.read_ue().unwrap();
                    }
                    if mem_op == 2 {
                        slice.long_term_pic_num = reader.read_ue().unwrap();
                    }
                    if mem_op == 3 || mem_op == 6 {
                        slice.long_term_frame_idx = reader.read_ue().unwrap();
                    }
                    if mem_op == 4 {
                        slice.max_long_term_frame_idx_plus1 = reader.read_ue().unwrap();
                    }
                }
            }
        }

        if pps.entropy_coding_mode_flag && !slice_type_is_i_slice(slice.slice_type) && !slice_type_is_si_slice(slice.slice_type) {
            slice.cabac_init_idc = reader.read_ue().unwrap();
        }
        slice.slice_qp_delta = reader.read_se().unwrap();

        if slice_type_is_sp_slice(slice.slice_type) || slice_type_is_si_slice(slice.slice_type) {
            if slice_type_is_sp_slice(slice.slice_type) {
                slice.sp_for_switch_flag = reader.read_flag().unwrap();
            }
            slice.slice_qs_delta = reader.read_se().unwrap();
        }

        if pps.deblocking_filter_control_present_flag {
            slice.disable_deblocking_filter_idc = reader.read_ue().unwrap();
            if slice.disable_deblocking_filter_idc != 1 {
                slice.slice_alpha_c0_offset_div2 = reader.read_se().unwrap();
                slice.slice_beta_offset_div2 = reader.read_se().unwrap();
            }
        }

        if pps.num_slice_groups_minus1 > 0 && pps.slice_group_map_type >= 3 &&
            pps.slice_group_map_type <= 5 {
            let pic_width_in_mbs = sps.pic_width_in_mbs_minus1 + 1;
            let pic_height_in_map_units = sps.pic_height_in_map_units_minus1 + 1;
            let pic_size_in_map_units = pic_width_in_mbs * pic_height_in_map_units;
            let slice_group_change_rate = pps.slice_group_change_rate_minus1 + 1;
            let nbits = ceil_log2(pic_size_in_map_units / slice_group_change_rate + 1);
            slice.slice_group_change_cycle = reader.read_u32(nbits).unwrap();
        }

        // slice_data()

        Ok(slice)
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
