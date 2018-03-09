pub enum H264NalFormat {
    BYTESTREAM, AVC, UNKNOWN
}

#[derive(Debug, Clone, PartialEq)]
pub enum H264NalUnitType {
    SPS,
    PPS,
    IDR,
    P,
    UNKNOWN
}

pub const EXTENDED_SAR : u8 = 255;

#[derive(Debug, Clone)]
pub struct H264VUIParameters {
    pub aspect_ratio_info_present_flag: u8,
    pub aspect_ratio_idc: u8,
    pub sar_width: u16,
    pub sar_height: u16,

    pub overscan_info_present_flag: u8,
    pub overscan_appropriate_flag: u8,

    pub video_signal_type_present_flag: u8,
    pub video_format: u8,
    pub video_full_range_flag: u8,
    pub colour_description_present_flag: u8,
    pub colour_primaries: u8,
    pub transfer_characteristics: u8,
    pub matrix_coefficients: u8,

    pub chroma_loc_info_present_flag: u8,
    pub chroma_sample_loc_type_top_field: u32,
    pub chroma_sample_loc_type_bottom_field: u32,

    pub timing_info_present_flag: u8,
    pub num_units_in_tick: u32,
    pub time_scale: u32,
    pub fixed_frame_rate_flag: u8,

    pub nal_hrd_parameters_present_flag: u8,
    pub nal_hrd_parameters: Option<H264HDRParameters>,
    pub vcl_hrd_parameters_present_flag: u8,
    pub vcl_hrd_parameters: Option<H264HDRParameters>,
    pub low_delay_hrd_flag: u8,
    pub pic_struct_present_flag: u8,

    pub bitstream_restriction_flag: u8,
    pub motion_vectors_over_pic_boundaries_flag: u8,
    pub max_bytes_per_pic_denom: u32,
    pub max_bits_per_mb_denom: u32,
    pub log2_max_mv_length_horizontal: u32,
    pub log2_max_mv_length_vertical: u32,
    pub max_num_reorder_frames: u32,
    pub max_dec_frame_buffering: u32
}

impl H264VUIParameters {
    pub fn new() -> H264VUIParameters {
        H264VUIParameters {
            aspect_ratio_info_present_flag: 0,
            aspect_ratio_idc: 0,
            sar_width: 0,
            sar_height: 0,
            overscan_info_present_flag: 0,
            overscan_appropriate_flag: 0,
            video_signal_type_present_flag: 0,
            video_format: 0,
            video_full_range_flag: 0,
            colour_description_present_flag: 0,
            colour_primaries: 0,
            transfer_characteristics: 0,
            matrix_coefficients: 0,
            chroma_loc_info_present_flag: 0,
            chroma_sample_loc_type_top_field: 0,
            chroma_sample_loc_type_bottom_field: 0,
            timing_info_present_flag: 0,
            num_units_in_tick: 0,
            time_scale: 0,
            fixed_frame_rate_flag: 0,
            nal_hrd_parameters_present_flag: 0,
            nal_hrd_parameters: None,
            vcl_hrd_parameters_present_flag: 0,
            vcl_hrd_parameters: None,
            low_delay_hrd_flag: 0,
            pic_struct_present_flag: 0,
            bitstream_restriction_flag: 0,
            motion_vectors_over_pic_boundaries_flag: 0,
            max_bytes_per_pic_denom: 0,
            max_bits_per_mb_denom: 0,
            log2_max_mv_length_horizontal: 0,
            log2_max_mv_length_vertical: 0,
            max_num_reorder_frames: 0,
            max_dec_frame_buffering: 0
        }
    }
}

#[derive(Debug, Clone)]
pub struct H264HDRParameters {
    pub cpb_cnt_minus1: u32,
    pub bit_rate_scale: u8,
    pub cpb_size_scale: u8,
    pub bit_rate_value_minus1: Vec<u32>,
    pub cpb_size_value_minus1: Vec<u32>,
    pub cbr_flag: Vec<u8>,
    pub initial_cpb_removal_delay_length_minus1: u8,
    pub cpb_removal_delay_length_minus1: u8,
    pub dpb_output_delay_length_minus1: u8,
    pub time_offset_length: u8
}

impl H264HDRParameters {
    pub fn new() -> H264HDRParameters {
        H264HDRParameters {
            cpb_cnt_minus1: 0,
            bit_rate_scale: 0,
            cpb_size_scale: 0,
            bit_rate_value_minus1: Vec::new(),
            cpb_size_value_minus1: Vec::new(),
            cbr_flag: Vec::new(),
            initial_cpb_removal_delay_length_minus1: 0,
            cpb_removal_delay_length_minus1: 0,
            dpb_output_delay_length_minus1: 0,
            time_offset_length: 0
        }
    }
}

#[derive(Debug, Clone)]
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
    pub separate_colour_plane_flag: bool,
    pub bit_depth_luma_minus8: u32,
    pub bit_depth_chroma_minus8: u32,
    pub qpprime_y_zero_transform_bypass_flag: u8,

    pub seq_scaling_matrix_present_flag: u8,
    pub seq_scaling_list_present_flag: Vec<u8>,
    pub scaling_list_4x4: Vec<Vec<u8>>,
    pub scaling_list_8x8: Vec<Vec<u8>>,

    pub log2_max_frame_num_minus4: u32,
    pub pic_order_cnt_type: u32,
    pub log2_max_pic_order_cnt_lsb_minus4: u32,
    pub delta_pic_order_always_zero_flag: bool,
    pub offset_for_non_ref_pic: i32,
    pub offset_for_top_to_bottom_field: i32,
    pub num_ref_frames_in_pic_order_cnt_cycle: u32,
    pub offset_for_ref_frame: Vec<i32>,

    pub max_num_ref_frames: u32,
    pub gaps_in_frame_num_value_allowed_flag: u8,
    pub pic_width_in_mbs_minus1: u32,
    pub pic_height_in_map_units_minus1: u32,

    pub frame_mbs_only_flag: bool,
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
            separate_colour_plane_flag: false,
            bit_depth_luma_minus8: 0,
            bit_depth_chroma_minus8: 0,
            qpprime_y_zero_transform_bypass_flag: 0,
            seq_scaling_matrix_present_flag: 0,
            seq_scaling_list_present_flag: Vec::new(),
            scaling_list_4x4: vec![vec![0u8; 16]; 6],
            scaling_list_8x8: vec![vec![0u8; 64]; 6],
            log2_max_frame_num_minus4: 0,
            pic_order_cnt_type: 0,
            log2_max_pic_order_cnt_lsb_minus4: 0,
            delta_pic_order_always_zero_flag: false,
            offset_for_non_ref_pic: 0,
            offset_for_top_to_bottom_field: 0,
            num_ref_frames_in_pic_order_cnt_cycle: 0,
            offset_for_ref_frame: Vec::new(),
            max_num_ref_frames: 0,
            gaps_in_frame_num_value_allowed_flag: 0,
            pic_width_in_mbs_minus1: 0,
            pic_height_in_map_units_minus1: 0,
            frame_mbs_only_flag: false,
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

#[derive(Debug, Clone)]
pub struct H264NalUnitPPS {
    pub pic_parameter_set_id: u32,
    pub seq_parameter_set_id: u32,
    pub entropy_coding_mode_flag: bool,
    pub bottom_field_pic_order_in_frame_present_flag: bool,
    pub num_slice_groups_minus1: u32,
    pub slice_group_map_type: u32,
    pub run_length_minus1: Vec<u32>,
    pub top_left: Vec<u32>,
    pub bottom_right: Vec<u32>,
    pub slice_group_change_direction_flag: u8,
    pub slice_group_change_rate_minus1: u32,
    pub pic_size_in_map_units_minus1: u32,
    pub slice_group_id: Vec<u32>,
    pub num_ref_idx_l0_default_active_minus1: u32,
    pub num_ref_idx_l1_default_active_minus1: u32,
    pub weighted_pred_flag: bool,
    pub weighted_bipred_idc: u8,
    pub pic_init_qp_minus26: i32,
    pub pic_init_qs_minus26: i32,
    pub chroma_qp_index_offset: i32,
    pub deblocking_filter_control_present_flag: bool,
    pub constrained_intra_pred_flag: u8,
    pub redundant_pic_cnt_present_flag: bool,
    pub transform_8x8_mode_flag: u8,
    pub pic_scaling_matrix_present_flag: u8,
    pub pic_scaling_list_present_flag: Vec<u8>,
    pub scaling_list_4x4: Vec<Vec<u8>>,
    pub scaling_list_8x8: Vec<Vec<u8>>,
    pub second_chroma_qp_index_offset: i32
}

impl H264NalUnitPPS {
    pub fn new() -> H264NalUnitPPS {
        H264NalUnitPPS {
            pic_parameter_set_id: 0,
            seq_parameter_set_id: 0,
            entropy_coding_mode_flag: false,
            bottom_field_pic_order_in_frame_present_flag: false,
            num_slice_groups_minus1: 0,
            slice_group_map_type: 0,
            run_length_minus1: Vec::new(),
            top_left: Vec::new(),
            bottom_right: Vec::new(),
            slice_group_change_direction_flag: 0,
            slice_group_change_rate_minus1: 0,
            pic_size_in_map_units_minus1: 0,
            slice_group_id: Vec::new(),
            num_ref_idx_l0_default_active_minus1: 0,
            num_ref_idx_l1_default_active_minus1: 0,
            weighted_pred_flag: false,
            weighted_bipred_idc: 0,
            pic_init_qp_minus26: 0,
            pic_init_qs_minus26: 0,
            chroma_qp_index_offset: 0,
            deblocking_filter_control_present_flag: false,
            constrained_intra_pred_flag: 0,
            redundant_pic_cnt_present_flag: false,
            transform_8x8_mode_flag: 0,
            pic_scaling_matrix_present_flag: 0,
            pic_scaling_list_present_flag: Vec::new(),
            scaling_list_4x4: vec![vec![0u8; 16]; 6],
            scaling_list_8x8: vec![vec![0u8; 64]; 6],
            second_chroma_qp_index_offset: 0
        }
    }
}

pub fn ceil_log2(val: u32) -> u32 {
    let t = vec![0xFFFFFFFF00000000,
                 0x00000000FFFF0000,
                 0x000000000000FF00,
                 0x00000000000000F0,
                 0x000000000000000C,
                 0x0000000000000002];
    let mut x = val;
    let mut y = if val & (val - 1) == 0 {
        0
    } else {
        1
    };
    let mut j = 32;
    for i in 0..6 {
        let k = if x & t[i] == 0 {
            0
        } else {
            j
        };
        y += k;
        x >>= k;
        j >>= 1;
    }

    y
}

const P_SLICE : u32 = 0;
const B_SLICE : u32 = 1;
const I_SLICE : u32 = 2;
const SP_SLICE : u32 = 3;
const SI_SLICE : u32 = 4;
const S_P_SLICE : u32 = 5;
const S_B_SLICE : u32 = 6;
const S_I_SLICE : u32 = 7;
const S_SP_SLICE : u32 = 8;
const S_SI_SLICE : u32 = 9;

pub fn slice_type_is_p_slice(slice_type: u32) -> bool {
    (slice_type % 5) == P_SLICE
}
pub fn slice_type_is_b_slice(slice_type: u32) -> bool {
    (slice_type % 5) == B_SLICE
}
pub fn slice_type_is_i_slice(slice_type: u32) -> bool {
    (slice_type % 5) == I_SLICE
}
pub fn slice_type_is_sp_slice(slice_type: u32) -> bool {
    (slice_type % 5) == SP_SLICE
}
pub fn slice_type_is_si_slice(slice_type: u32) -> bool {
    (slice_type % 5) == SI_SLICE
}

#[derive(Debug, Clone)]
pub struct H264NalUnitSlice {
    pub first_mb_in_slice: u32,
    pub slice_type: u32,
    pub pic_parameter_set_id: u32,
    pub colour_plane_id: u8,
    pub frame_num: u32,
    pub field_pic_flag: bool,
    pub bottom_field_flag: bool,
    pub idr_pic_id: u32,
    pub pic_order_cnt_lsb: u16,
    pub delta_pic_order_cnt_bottom: i32,
    pub delta_pic_order_cnt: Vec<i32>,
    pub redundant_pic_cnt: u32,
    pub direct_spatial_mv_pred_flag: bool,
    pub num_ref_idx_active_override_flag: bool,
    pub num_ref_idx_l0_active_minus1: u32,
    pub num_ref_idx_l1_active_minus1: u32,

    // TODO: When I have a B slice to parse
    // ref_pic_list_mvc_modification
    // ref_pic_list_modification

    // pred_weight_table
    // TODO: When I have a P slice to parse

    // dec_ref_pic_marking
    pub no_output_of_prior_pics_flag: bool,
    pub long_term_reference_flag: bool,
    pub adaptive_ref_pic_marking_mode_flag: bool,
    pub difference_of_pic_nums_minus1: u32,
    pub long_term_pic_num: u32,
    pub long_term_frame_idx: u32,
    pub max_long_term_frame_idx_plus1: u32,

    pub cabac_init_idc: u32,
    pub slice_qp_delta: i32,

    pub sp_for_switch_flag: bool,
    pub slice_qs_delta: i32,

    pub disable_deblocking_filter_idc: u32,
    pub slice_alpha_c0_offset_div2: i32,
    pub slice_beta_offset_div2: i32,

    pub slice_group_change_cycle: u32,
}

impl H264NalUnitSlice {
    pub fn new() -> H264NalUnitSlice {
        H264NalUnitSlice {
            first_mb_in_slice: 0,
            slice_type: 0,
            pic_parameter_set_id: 0,
            colour_plane_id: 0,
            frame_num: 0,
            field_pic_flag: false,
            bottom_field_flag: false,
            idr_pic_id: 0,
            pic_order_cnt_lsb: 0,
            delta_pic_order_cnt_bottom: 0,
            delta_pic_order_cnt: vec![0; 2],
            redundant_pic_cnt: 0,
            direct_spatial_mv_pred_flag: false,
            num_ref_idx_active_override_flag: false,
            num_ref_idx_l0_active_minus1: 0,
            num_ref_idx_l1_active_minus1: 0,

            // ref_pic_list_mvc_modification
            // ref_pic_list_modification
            // pred_weight_table
            // dec_ref_pic_marking
            no_output_of_prior_pics_flag: false,
            long_term_reference_flag: false,
            adaptive_ref_pic_marking_mode_flag: false,
            difference_of_pic_nums_minus1: 0,
            long_term_pic_num: 0,
            long_term_frame_idx: 0,
            max_long_term_frame_idx_plus1: 0,

            cabac_init_idc: 0,
            slice_qp_delta: 0,
            sp_for_switch_flag: false,
            slice_qs_delta: 0,
            disable_deblocking_filter_idc: 0,
            slice_alpha_c0_offset_div2: 0,
            slice_beta_offset_div2: 0,
            slice_group_change_cycle: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct H264NalUnit {
    pub name: String,
    pub sc_offset: usize,
    pub data_offset: usize,
    pub size: usize,

    /* H264 Nal Unit Fields */
    pub idr_pic_flag: bool,
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
            idr_pic_flag: nal_unit_type == H264NalUnitType::IDR,
            nal_ref_idc: ref_idc,
            nal_unit_type_num: unit_type,
            nal_unit_type: nal_unit_type,
        }
    }
}

