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
pub struct H264VUIParameters {
}

#[derive(Debug)]
pub struct H264NalUnitSPS {
    pub profile_idc: u8,
    pub constraint_0_flag: u8,
    pub constraint_1_flag: u8,
    pub constraint_2_flag: u8,
    pub constraint_3_flag: u8,
    pub constraint_4_flag: u8,
    pub constraint_5_flag: u8,
    pub level_idc: u8,
    pub seq_parameter_set_id: u32,

    pub chroma_format_idc: u32,
    pub separate_colour_plane_flag: u8,
    pub bit_depth_luma_minus8: u32,
    pub bit_depth_chroma_minus8: u32,
    pub qpprime_y_zero_transform_bypass_flag: u8,

    pub seq_scaling_matrix_present_flag: u8,
    pub seq_scaling_list_present_flag: Vec<u8>,
    pub scaling_list_4x4: Vec<u8>,
    pub scaling_list_8x8: Vec<u8>,

    pub log2_max_frame_num_minus4: u32,
    pub pic_order_cnt_type: u32,
    pub log2_max_pic_order_cnt_lsb_minus4: u32,
    pub delta_pic_order_always_zero_flag: u8,
    pub offset_for_non_ref_pic: i32,
    pub offset_for_top_to_bottom_field: i32,
    pub num_ref_frames_in_pic_order_cnt_cycle: u32,
    pub offset_for_ref_frame: Vec<i32>,

    pub max_num_ref_frames: u32,
    pub gaps_in_frame_num_value_allowed_flag: u8,
    pub pic_width_in_mbs_minus1: u32,
    pub pic_height_in_map_units_minus1: u32,

    pub frame_mbs_only_flag: u8,
    pub mb_adaptive_frame_field_flag: u8,

    pub direct_8x8_inference_flag: u8,

    pub frame_cropping_flag: u8,
    pub frame_crop_left_offset: u32,
    pub frame_crop_right_offset: u32,
    pub frame_crop_top_offset: u32,
    pub frame_crop_bottom_offset: u32,

    pub vui_parameters_present_flag: u8,
    pub vui_parameters: Option<H264VUIParameters>
}

impl H264NalUnitSPS {
    pub fn new() -> H264NalUnitSPS {
        H264NalUnitSPS {
            profile_idc: 0,
            constraint_0_flag: 0,
            constraint_1_flag: 0,
            constraint_2_flag: 0,
            constraint_3_flag: 0,
            constraint_4_flag: 0,
            constraint_5_flag: 0,
            level_idc: 0,
            seq_parameter_set_id: 0,
            chroma_format_idc: 0,
            separate_colour_plane_flag: 0,
            bit_depth_luma_minus8: 0,
            bit_depth_chroma_minus8: 0,
            qpprime_y_zero_transform_bypass_flag: 0,
            seq_scaling_matrix_present_flag: 0,
            seq_scaling_list_present_flag: Vec::new(),
            scaling_list_4x4: Vec::new(),
            scaling_list_8x8: Vec::new(),
            log2_max_frame_num_minus4: 0,
            pic_order_cnt_type: 0,
            log2_max_pic_order_cnt_lsb_minus4: 0,
            delta_pic_order_always_zero_flag: 0,
            offset_for_non_ref_pic: 0,
            offset_for_top_to_bottom_field: 0,
            num_ref_frames_in_pic_order_cnt_cycle: 0,
            offset_for_ref_frame: Vec::new(),
            max_num_ref_frames: 0,
            gaps_in_frame_num_value_allowed_flag: 0,
            pic_width_in_mbs_minus1: 0,
            pic_height_in_map_units_minus1: 0,
            frame_mbs_only_flag: 0,
            mb_adaptive_frame_field_flag: 0,
            direct_8x8_inference_flag: 0,
            frame_cropping_flag: 0,
            frame_crop_left_offset: 0,
            frame_crop_right_offset: 0,
            frame_crop_top_offset: 0,
            frame_crop_bottom_offset: 0,
            vui_parameters_present_flag: 0,
            vui_parameters: None
        }
    }
}

#[derive(Debug)]
pub struct H264NalUnitPPS {
}

#[derive(Debug)]
pub struct H264NalUnitIDR {
    pub first_mb_in_slice: u32
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

