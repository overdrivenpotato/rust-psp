/// Playstation Movie Format
///
/// This module contains the imports for the Playstation Movie Format routines.
///
/// Source note:
///
/// NIDs and functions from PPSSPP,
/// sce datatypes pulled from Demo Disc for PSP Vol.1 (Japan)
use core::ffi::c_void;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScePsmf_GroupingPeriod {
    pub length_grouping_period: [u8; 4usize],
    pub msb_groupingperiod_start_time: [u8; 2usize],
    pub groupingperiod_start_time: [u8; 4usize],
    pub msb_groupingperiod_end_time: [u8; 2usize],
    pub groupingperiod_end_time: [u8; 4usize],
    pub reserved: u8,
    pub number_of_groups: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScePsmf_Group {
    pub length_group: [u8; 4usize],
    pub reserved: u8,
    pub number_of_streams: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScePsmf_SequenceInfo_Video {
    pub horizontal_size: i32,
    pub vertical_size: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScePsmf_GroupStream {
    pub stream_id: u8,
    pub private_stream_id: u8,
    pub p_std_buffer_scale_and_size: [u8; 2usize],
    pub ep_map_for_one_stream_id_start_address: [u8; 4usize],
    pub number_of_ep_entries: [u8; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScePsmf_EP {
    pub picture_copy_and_msb_pts_start: [u8; 2usize],
    pub pts_ep_start: [u8; 4usize],
    pub rpn_ep_start: [u8; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScePsmf_EntryPoint {
    pub pts_ep_start: u32,
    pub ep_start_offset: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScePsmf {
    pub type_: i32,
    pub grouping_period_id: i32,
    pub group_id: i32,
    pub psmf_stream_id: i32,
    pub header: *mut c_void,
    pub sequence_info: *mut c_void,
    pub current_grouping_period: *mut c_void,
    pub current_group: *mut c_void,
    pub current_stream: *mut c_void,
    pub current_ep_map: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScePsmf_SequenceInfo {
    pub length: [u8; 4usize],
    pub msb_presentation_start_time: [u8; 2usize],
    pub presentation_start_time: [u8; 4usize],
    pub msb_presentation_end_time: [u8; 2usize],
    pub presentation_end_time: [u8; 4usize],
    pub mux_rate_bound: [u8; 4usize],
    pub std_delay_bound: [u8; 4usize],
    pub number_of_total_stream: u8,
    pub number_of_grouping_period: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScePsmf_SequenceInfo_Audio {
    pub channel_configuration: i32,
    pub sampling_frequency: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ScePsmf_Header {
    pub type_indicator: [u8; 4usize],
    pub version_number: [u8; 4usize],
    pub stream_chunk_start_address: [u8; 4usize],
    pub stream_chunk_size: [u8; 4usize],
    pub reserved: [u8; 64usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PsmfData {
    pub version: u32,
    pub header_size: u32,
    pub header_offset: u32,
    pub stream_size: u32,
    pub stream_offset: u32,
    pub stream_num: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PsmfPlayerData {
    pub video_codec: i32,
    pub video_stream_num: i32,
    pub audio_codec: i32,
    pub audio_stream_num: i32,
    pub play_mode: i32,
    pub play_speed: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PsmfVideoData {
    pub frame_width: i32,
    pub display_buf: u32,
    pub display_pts: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PsmfInfo {
    pub last_frame_ts: u32,
    pub num_video_streams: i32,
    pub num_audio_steams: i32,
    pub num_pcm_streams: i32,
    pub player_version: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PsmfPlayer {
    pub file_handle: i32,
    pub file_offset: u32,
    pub read_size: i32,
    pub stream_size: i32,
    pub temp_buf: [u8; 0x10000],
    pub video_codec: i32,
    pub video_stream_num: i32,
    pub audio_codec: i32,
    pub audio_stream_num: i32,
    pub play_mode: i32,
    pub play_speed: i32,
    pub total_duration_timestamp: u64,
    pub display_buffer: i32,
    pub display_buffer_size: i32,
    pub playback_thread_priority: i32,
    pub total_video_streams: i32,
    pub total_audio_streams: i32,
    pub player_version: i32,
    pub video_step: i32,
    pub warm_up: i32,
    pub seek_dest_timestamp: i64,
    pub video_width: i32,
    pub video_height: i32,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum PsmfConfigMode {
    Loop,
    PixelType,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum PsmfPlayerMode {
    Play,
    SlowMotion,
    StepFrame,
    Pause,
    Forward,
    Rewind,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum PsmfPlayerStatus {
    None = 0,
    Init = 1,
    Standby = 2,
    Playing = 4,
    Error = 0x100,
    PlayingFinished = 0x200,
}

psp_extern! {
    #![name = "scePsmf"]
    #![flags = 0x4001]
    #![version = (0x00, 0x00)]

    #[psp(0xC22C8327)]
    pub fn scePsmfSetPsmf(psmf: &ScePsmf, psmf_data: &PsmfData) -> u32;

    #[psp(0xEAED89CD)]
    pub fn scePsmfGetNumberOfStreams(psmf: &ScePsmf) -> u32;

    #[psp(0x68D42328)]
    pub fn scePsmfGetNumberOfSpecificStreams(psmf: &ScePsmf, stream_type: i32) -> u32;

    #[psp(0x1E6D9013)]
    pub fn scePsmfSpecifyStreamWithStreamType(psmf: &ScePsmf, steam_type: u32, channel: u32) -> u32;

    #[psp(0x0C120E1D)]
    pub fn scePsmfSpecifyStreamWithStreamTypeNumber(psmf: &ScePsmf, stream_type: u32, type_num: u32) -> u32;

    #[psp(0x0BA514E5)]
    pub fn scePsmfGetVideoInfo(psmf_struct: &ScePsmf, video_info: &ScePsmf_SequenceInfo_Video) -> u32;

    #[psp(0xA83F7113)]
    pub fn scePsmfGetAudioInfo(psmf_struct: &ScePsmf, audio_info: &ScePsmf_SequenceInfo_Audio) -> u32;

    #[psp(0xC7DB3A5B)]
    pub fn scePsmfGetCurrentStreamType(psmf_struct: &ScePsmf, type_: &mut u32, channel_addr: u32) -> u32;

    #[psp(0xA5EBFE81)]
    pub fn scePsmfGetStreamSize(psmf_struct: &ScePsmf, size: &mut u32) -> u32;

    #[psp(0x5B70FCC1)]
    pub fn scePsmfQueryStreamOffset(buffer_addr: u32, offset_addr: u32) -> u32;

    #[psp(0x9553CC91)]
    pub fn scePsmfQueryStreamSize(buffer_addr: u32, size_addr: u32) -> u32;

    #[psp(0xB78EB9E9)]
    pub fn scePsmfGetHeaderSize(psmf_struct: &ScePsmf, size_addr: u32) -> u32;

    #[psp(0xE1283895)]
    pub fn scePsmfGetPsmfVersion(psmf_struct: &ScePsmf) -> u32;

    #[psp(0x2673646B)]
    pub fn scePsmfVerifyPsmf(psmf_addr: u32) -> u32;

    #[psp(0x7491C438)]
    pub fn scePsmfGetNumberOfEPentries(psmf_struct: &ScePsmf) -> u32;

    #[psp(0x76D3AEBA)]
    pub fn scePsmfGetPresentationStartTime(psmf_struct: &ScePsmf, start_time_addr: u32) -> u32;

    #[psp(0xBD8AE0D8)]
    pub fn scePsmfGetPresentationEndTime(psmf_struct: &ScePsmf, end_time_addr: u32) -> u32;

    #[psp(0x28240568)]
    pub fn scePsmfGetCurrentStreamNumber(psmf_struct: &ScePsmf) -> u32;

    #[psp(0x971A3A90)]
    pub fn scePsmfCheckEPMap(psmf_struct: &ScePsmf) -> u32;

    #[psp(0x4E624A34)]
    pub fn scePsmfGetEPWithId(psmf_struct: &ScePsmf, epid: i32, entry: &ScePsmf_EP) -> u32;

    #[psp(0x7C0E7AC3)]
    pub fn scePsmfGetEPWithTimestamp(psmf_struct: &ScePsmf, ts: u32, entry: &ScePsmf_EP) -> u32;
    #[psp(0x5F457515)]
    pub fn scePsmfGetEPidWithTimestamp(psmf_struct: &ScePsmf, ts: u32) -> u32;
}

psp_extern! {
    #![name = "scePsmfPlayer"]
    #![flags = 0x4001]
    #![version = (0x00, 0x00)]

    #[psp(0x235D8787)]
    pub fn scePsmfPlayerCreate(psmf_player: &mut PsmfPlayer, data_ptr: *const u32) -> i32;

    #[psp(0x1078C008)]
    pub fn scePsmfPlayerStop(psmf_player: &mut PsmfPlayer) -> i32;

    #[psp(0x2BEB1569)]
    pub fn scePsmfPlayerBreak(psmf_player: &mut PsmfPlayer) -> i32;

    #[psp(0x3D6D25A9)]
    pub fn scePsmfPlayerSetPsmf(psmf_player: &mut PsmfPlayer, filename: *const u8) -> i32;

    #[psp(0x58B83577)]
    pub fn scePsmfPlayerSetPsmfCB(psmf_player: &mut PsmfPlayer, filename: *const u8) -> i32;

    #[psp(0x76C0F4AE)]
    pub fn scePsmfPlayerSetPsmfOffset(psmf_player: &mut PsmfPlayer, filename: *const u8, offset: i32) -> i32;

    #[psp(0xA72DB4F9)]
    pub fn scePsmfPlayerSetPsmfOffsetCB(psmf_player: &mut PsmfPlayer, filename: *const u8, offset: i32) -> i32;

    #[psp(0x3EA82A4B)]
    pub fn scePsmfPlayerGetAudioOutSize(psmf_player: &mut PsmfPlayer) -> i32;

    #[psp(0x95A84EE5)]
    pub fn scePsmfPlayerStart(psmf_player: &mut PsmfPlayer, psmf_player_data: &PsmfPlayerData, init_pts: i32) -> i32;

    #[psp(0x9B71A274)]
    pub fn scePsmfPlayerDelete(psmf_player: &mut PsmfPlayer) -> i32;

    #[psp(0xA0B8CA55)]
    pub fn scePsmfPlayerUpdate(psmf_player: &mut PsmfPlayer) -> i32;

    #[psp(0xE792CD94)]
    pub fn scePsmfPlayerReleasePsmf(psmf_player: &mut PsmfPlayer) -> i32;

    #[psp(0x46F61F8B)]
    pub fn scePsmfPlayerGetVideoData(psmf_player: &mut PsmfPlayer, video_data: &mut PsmfVideoData) -> i32;

    #[psp(0xB9848A74)]
    pub fn scePsmfPlayerGetAudioData(psmf_player: &mut PsmfPlayer, audio_data_addr: u32) -> i32;

    #[psp(0xF8EF08A6)]
    pub fn scePsmfPlayerGetCurrentStatus(psmf_player: &mut PsmfPlayer) -> PsmfPlayerStatus;

    #[psp(0x3ED62233)]
    pub fn scePsmfPlayerGetCurrentPts(psmf_player: &mut PsmfPlayer, current_pts_addr: u32) -> u32;

    #[psp(0xDF089680)]
    pub fn scePsmfPlayerGetPsmfInfo(psmf_player: &mut PsmfPlayer, psmf_info: &mut PsmfInfo, width: &mut u32, height: &mut u32) -> u32;

    #[psp(0xF3EFAA91)]
    pub fn scePsmfPlayerGetCurrentPlayMode(psmf_player: &mut PsmfPlayer, play_mode: &PsmfPlayerMode, play_speed: &mut u32) -> u32;

    #[psp(0x9FF2B2E7)]
    pub fn scePsmfPlayerGetCurrentVideoStream(psmf_player: &mut PsmfPlayer, video_codec: &mut i32, video_stream_num: &mut  i32) -> u32;

    #[psp(0x68F07175)]
    pub fn scePsmfPlayerGetCurrentAudioStream(psmf_player: &mut PsmfPlayer, audio_codec: &mut i32, audio_stream_num: &mut i32) -> u32;

    #[psp(0x2D0E4E0A)]
    pub fn scePsmfPlayerSetTempBuf(psmf_player: &mut PsmfPlayer, temp_buf: *mut u8, temp_buf_size: u32) -> i32;

    #[psp(0xA3D81169)]
    pub fn scePsmfPlayerChangePlayMode(psmf_player: &mut PsmfPlayer, play_mode: PsmfPlayerMode, play_speed: i32) -> u32;

    #[psp(0xB8D10C56)]
    pub fn scePsmfPlayerSelectAudio(psmf_player: &mut PsmfPlayer) -> u32;

    #[psp(0x8A9EBDCD)]
    pub fn scePsmfPlayerSelectVideo(psmf_player: &mut PsmfPlayer) -> u32;

    #[psp(0x75F03FA2)]
    pub fn scePsmfPlayerSelectSpecificVideo(psmf_player: &mut PsmfPlayer, video_codec: i32, video_stream_num: i32) -> u32;

    #[psp(0x85461EFF)]
    pub fn scePsmfPlayerSelectSpecificAudio(psmf_player: &mut PsmfPlayer, audio_codec: i32, audio_stream_Num: i32) -> u32;

    #[psp(0x1E57A8E7)]
    pub fn scePsmfPlayerConfigPlayer(psmf_player: &mut PsmfPlayer, config_mode: PsmfConfigMode, config_attr: i32) -> u32;
}
