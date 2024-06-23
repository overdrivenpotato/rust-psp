use core::ffi::c_void;
use num_enum::TryFromPrimitive;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UtilityDialogCommon {
    /// Size of the structure
    pub size: u32,
    /// Language
    pub language: SystemParamLanguage,
    /// Which button accepts the dialog
    pub button_accept: UtilityDialogButtonAccept,
    /// Graphics thread priority
    pub graphics_thread: i32,
    /// Access/fileio thread priority (SceJobThread)
    pub access_thread: i32,
    /// Font thread priority (ScePafThread)
    pub font_thread: i32,
    /// Sound thread priority
    pub sound_thread: i32,
    /// Result
    pub result: i32,
    pub reserved: [i32; 4usize],
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum UtilityMsgDialogMode {
    /// Error message
    Error,
    /// String message
    Text,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum UtilityMsgDialogPressed {
    Unknown1,
    Yes,
    No,
    Back,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum UtilityDialogButtonAccept {
    Circle,
    Cross,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, TryFromPrimitive)]
/// Return-values for the various `sceUtility***GetStatus()` functions, when they don't return an error.
///
/// # Example
///
/// ```no_run
///    let status: PspUtilityDialogState = unsafe { sceUtilityOskGetStatus().try_into().unwrap() };
/// ```
pub enum PspUtilityDialogState {
    None,
    Init,
    Visible,
    Quit,
    Finished,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum SceUtilityOskInputLanguage {
    Default,
    Japanese,
    English,
    French,
    Spanish,
    German,
    Italian,
    Dutch,
    Portugese,
    Russian,
    Korean,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum SceUtilityOskInputType {
    All,
    LatinDigit,
    LatinSymbol,
    LatinLowercase = 4,
    LatinUppercase = 8,
    JapaneseDigit = 0x100,
    JapaneseSymbol = 0x200,
    JapaneseLowercase = 0x400,
    JapaneseUppercase = 0x800,
    JapaneseHiragana = 0x1000,
    JapaneseHalfWidthKatakana = 0x2000,
    JapaneseKatakana = 0x4000,
    JapaneseKanji = 0x8000,
    RussianLowercase = 0x10000,
    RussianUppercase = 0x20000,
    Korean = 0x40000,
    Url = 0x80000,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum SceUtilityOskState {
    None,
    Initializing,
    Initialized,
    Visible,
    Quit,
    Finished,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum SceUtilityOskResult {
    Unchanged,
    Cancelled,
    Changed,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, TryFromPrimitive)]
pub enum SystemParamLanguage {
    Japanese,
    English,
    French,
    Spanish,
    German,
    Italian,
    Dutch,
    Portugese,
    Russian,
    Korean,
    ChineseTraditional,
    ChineseSimplified,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
/// #9 seems to be Region or maybe X/O button swap.
/// It doesn't exist on JAP v1.0
/// is 1 on NA v1.5s
/// is 0 on JAP v1.5s
/// is read-only
pub enum SystemParamId {
    StringNickname = 1,
    AdhocChannel,
    WlanPowerSave,
    DateFormat,
    TimeFormat,
    Timezone,
    DaylightSavings,
    Language,
    Unknown,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, TryFromPrimitive)]
pub enum SystemParamAdhocChannel {
    ChannelAutomatic = 0,
    Channel1 = 1,
    Channel6 = 6,
    Channel11 = 11,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, TryFromPrimitive)]
pub enum SystemParamWlanPowerSaveState {
    Off,
    On,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, TryFromPrimitive)]
pub enum SystemParamDateFormat {
    YYYYMMDD,
    MMDDYYYY,
    DDMMYYYY,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, TryFromPrimitive)]
pub enum SystemParamTimeFormat {
    Hour24,
    Hour12,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, TryFromPrimitive)]
pub enum SystemParamDaylightSavings {
    Std,
    Dst,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum AvModule {
    AvCodec,
    SasCore,
    /// Requires AvCodec loading first
    Atrac3Plus,
    /// Requires AvCodec loading first
    MpegBase,
    Mp3,
    Vaudio,
    Aac,
    G729,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum Module {
    NetCommon = 0x100,
    NetAdhoc,
    NetInet,
    NetParseUri,
    NetHttp,
    NetSsl,

    UsbPspCm = 0x200,
    UsbMic,
    UsbCam,
    UsbGps,

    AvCodec = 0x300,
    AvSascore,
    AvAtrac3Plus,
    AvMpegBase,
    AvMp3,
    AvVaudio,
    AvAac,
    AvG729,

    NpCommon = 0x400,
    NpService,
    NpMatching2,
    NpDrm = 0x500,

    Irda = 0x600,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum NetModule {
    NetCommon = 1,
    NetAdhoc,
    NetInet,
    NetParseUri,
    NetHttp,
    NetSsl,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum UsbModule {
    UsbPspCm = 1,
    UsbAcc,
    /// Requires UsbAcc loading first
    UsbMic,
    /// Requires UsbAcc loading first
    UsbCam,
    /// Requires UsbAcc loading first
    UsbGps,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum NetParam {
    /// c-style string
    Name,
    /// c-style string
    Ssid,
    /// i32
    Secure,
    /// c-style string
    WepKey,
    /// i32
    IsStaticIp,
    /// c-style string
    Ip,
    /// c-style string
    NetMask,
    /// c-style string
    Route,
    /// i32
    ManualDns,
    /// c-style string
    PrimaryDns,
    /// c-style string
    SecondaryDns,
    /// c-style string
    ProxyUser,
    /// c-style string
    ProxyPass,
    /// i32
    UseProxy,
    /// c-style string
    ProxyServer,
    /// i32
    ProxyPort,
    /// i32
    Unknown1,
    /// i32
    Unknown2,
}

#[derive(Debug, Copy, Clone)]
#[repr(i32)]
pub enum UtilityNetconfAction {
    ConnectAP,
    DisplayStatus,
    ConnectAdhoc,
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug)]
    pub struct UtilityMsgDialogOption: i32 {
        /// Error message (why two flags?)
        const ERROR = 0;
        /// Text message (why two flags?)
        const TEXT = 1;
        /// Yes/No buttons instead of cancel
        const YES_NO_BUTTONS = 0x10;
        /// Default position 'No', if not set will default to 'Yes'
        const DEFAULT_NO = 0x100;
    }
}

/// Structure to hold the parameters for a message dialog
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UtilityMsgDialogParams {
    pub base: UtilityDialogCommon,
    pub unknown: i32,
    pub mode: UtilityMsgDialogMode,
    pub error_value: u32,
    /// The message to display (may contain embedded linefeeds)
    pub message: [u8; 512usize],
    pub options: UtilityMsgDialogOption,
    pub button_pressed: UtilityMsgDialogPressed,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UtilityNetconfAdhoc {
    pub name: [u8; 8usize],
    pub timeout: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UtilityNetconfData {
    pub base: UtilityDialogCommon,
    /// One of NetconfActions
    pub action: UtilityNetconfAction,
    pub adhocparam: *mut UtilityNetconfAdhoc,
    /// Set to 1 to allow connections with the 'Internet Browser' option set to 'Start' (ie. hotspot connection)
    pub hotspot: i32,
    /// Will be set to 1 when connected to a hotspot style connection
    pub hotspot_connected: i32,
    /// Set to 1 to allow connections to Wifi service providers (WISP)
    pub wifisp: i32,
}

/// Datatype for sceUtilityGetNetParam
/// since it can return a u32 or a string
/// we use a union to avoid ugly casting
#[repr(C)]
#[derive(Copy, Clone)]
pub union UtilityNetData {
    pub as_uint: u32,
    pub as_string: [u8; 128usize],
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum UtilitySavedataMode {
    AutoLoad,
    AutoSave,
    Load,
    Save,
    ListLoad,
    ListSave,
    ListDelete,
    Delete,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum UtilitySavedataFocus {
    Unknown1,
    /// First in list
    FirstList,
    /// Last in list
    LastList,
    /// Most recent date
    Latest,
    Oldest,
    Unknown2,
    Unknown3,
    /// First empty slot
    FirstEmpty,
    /// Last empty slot
    LastEmpty,
}

/// title, savedataTitle, detail: parts of the unencrypted SFO
///data, it contains what the VSH and standard load screen shows
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UtilitySavedataSFOParam {
    pub title: [u8; 128usize],
    pub savedata_title: [u8; 128usize],
    pub detail: [u8; 1024usize],
    pub parental_level: u8,
    pub unknown: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UtilitySavedataFileData {
    pub buf: *mut c_void,
    pub buf_size: usize,
    // why are there two sizes?
    pub size: usize,
    pub unknown: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UtilitySavedataListSaveNewData {
    pub icon0: UtilitySavedataFileData,
    pub title: *mut u8,
}

/// Structure to hold the parameters for the `sceUtilitySavedataInitStart` function.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceUtilitySavedataParam {
    pub base: UtilityDialogCommon,
    pub mode: UtilitySavedataMode,
    pub unknown1: i32,
    pub overwrite: i32,
    /// gameName: name used from the game for saves, equal for all saves
    pub game_name: [u8; 13usize],
    pub reserved: [u8; 3usize],
    /// saveName: name of the particular save, normally a number
    pub save_name: [u8; 20usize],
    /// saveNameList: used by multiple modes
    pub save_name_list: *mut [u8; 20usize],
    /// fileName: name of the data file of the game for example DATA.BIN
    pub file_name: [u8; 13usize],
    pub reserved1: [u8; 3usize],
    /// pointer to a buffer that will contain data file unencrypted data
    pub data_buf: *mut c_void,
    /// size of allocated space to dataBuf
    pub data_buf_size: usize,
    pub data_size: usize,
    pub sfo_param: UtilitySavedataSFOParam,
    pub icon0_file_data: UtilitySavedataFileData,
    pub icon1_file_data: UtilitySavedataFileData,
    pub pic1_file_data: UtilitySavedataFileData,
    pub snd0_file_data: UtilitySavedataFileData,
    /// Pointer to an SavedataListSaveNewData structure
    pub new_data: *mut UtilitySavedataListSaveNewData,
    /// Initial focus for lists
    pub focus: UtilitySavedataFocus,
    /// unknown2: ?
    pub unknown2: [i32; 4usize],
    /// encrypt/decrypt key for save with firmware >= 2.00
    pub key: [u8; 16],
    /// ? firmware >= 2.00
    pub unknown3: [u8; 20],
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum UtilityGameSharingMode {
    /// Single send
    Single = 1,
    /// Up to 4 simulataneous sends
    Multiple,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum UtilityGameSharingDataType {
    /// EBOOT is a file
    File = 1,
    /// EBOOT is in memory
    Memory,
}

/// Structure to hold the parameters for Game Sharing
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UtilityGameSharingParams {
    pub base: UtilityDialogCommon,
    /// Set to 0
    pub unknown1: i32,
    /// Set to 0
    pub unknown2: i32,
    pub name: [u8; 8usize],
    /// Set to 0
    pub unknown3: i32,
    /// Set to 0
    pub unknown4: i32,
    /// Set to 0
    pub unknown5: i32,
    /// Return value
    pub result: i32,
    /// File path if `UtilityGameSharingDataType::File` specified
    pub filepath: *mut u8,
    /// Send mode. One of `UtilityGameSharingMode`
    pub mode: UtilityGameSharingMode,
    /// Data type. One of `UtilityGameSharingDataType`
    pub datatype: UtilityGameSharingDataType,
    /// Pointer to the EBOOT data in memory
    pub data: *mut c_void,
    /// Size of the EBOOT data in memory
    pub datasize: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UtilityHtmlViewerParam {
    pub base: UtilityDialogCommon,
    /// Pointer to the memory pool to be used
    pub memaddr: *mut c_void,
    /// Size of the memory pool
    pub memsize: u32,
    /// Unknown. Pass 0
    pub unknown1: i32,
    /// Unknown. Pass 0
    pub unknown2: i32,
    /// URL to be opened initially
    pub initialurl: *mut u8,
    /// Number of tabs (maximum of 3)
    pub numtabs: u32,
    /// One of ::UtilityHtmlViewerInterfaceModes
    pub interfacemode: UtilityHtmlViewerInterfaceMode,
    /// Values from ::UtilityHtmlViewerOption. Bitwise OR together
    pub options: UtilityHtmlViewerOption,
    /// Directory to be used for downloading
    pub dldirname: *mut u8,
    /// Filename to be used for downloading
    pub dlfilename: *mut u8,
    /// Directory to be used for uploading
    pub uldirname: *mut u8,
    /// Filename to be used for uploading
    pub ulfilename: *mut u8,
    /// One of ::UtilityHtmlViewerCookieMode
    pub cookiemode: UtilityHtmlViewerCookieMode,
    /// Unknown. Pass 0
    pub unknown3: u32,
    /// URL to set the home page to
    pub homeurl: *mut u8,
    /// One of ::UtilityHtmlViewerTextSize
    pub textsize: UtilityHtmlViewerTextSize,
    /// One of ::UtilityHtmlViewerDisplayMode
    pub displaymode: UtilityHtmlViewerDisplayMode,
    /// One of ::UtilityHtmlViewerConnectMode
    pub connectmode: UtilityHtmlViewerConnectMode,
    /// One of ::UtilityHtmlViewerDisconnectMode
    pub disconnectmode: UtilityHtmlViewerDisconnectMode,
    /// The maximum amount of memory the browser used
    pub memused: u32,
    /// Unknown. Pass 0
    pub unknown4: [i32; 10usize],
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum UtilityHtmlViewerInterfaceMode {
    /// Full user interface
    Full,
    /// Limited user interface
    Limited,
    /// No user interface
    None,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum UtilityHtmlViewerCookieMode {
    /// Disable accepting cookies
    Disabled = 0,
    /// Enable accepting cookies
    Enabled,
    /// Confirm accepting a cookie every time
    Confirm,
    /// Use the system default for accepting cookies
    Default,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum UtilityHtmlViewerTextSize {
    /// Large text size
    Large,
    /// Normal text size
    Normal,
    /// Small text size
    Small,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum UtilityHtmlViewerDisplayMode {
    /// Normal display
    Normal,
    /// Fit display
    Fit,
    /// Smart fit display
    SmartFit,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum UtilityHtmlViewerConnectMode {
    /// Auto connect to the last used connection
    Last,
    /// Manually select the connection (once)
    ManualOnce,
    /// Manually select the connection (every time)
    ManualAll,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum UtilityHtmlViewerDisconnectMode {
    /// Enable automatic disconnect
    Enable,
    /// Disable automatic disconnect
    Disable,
    /// Confirm disconnection
    Confirm,
}

bitflags::bitflags! {
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct UtilityHtmlViewerOption: u32 {
        /// Open SCE net start page
    const OPEN_SCE_START_PAGE  = 0x000001;
    /// Disable startup limitations
    const DISABLE_STARTUP_LIMITS = 0x000002;
    /// Disable exit confirmation dialog
    const DISABLE_EXIT_DIALOG = 0x000004;
    /// Disable cursor
    const DISABLE_CURSOR = 0x000008;
    /// Disable download completion confirmation dialog
    const DISABLE_DOWNLOAD_COMPLETE_DIALOG = 0x000010;
    /// Disable download confirmation dialog
    const DISABLE_DOWNLOAD_START_DIALOG = 0x000020;
    /// Disable save destination confirmation dialog
    const DISABLE_DOWNLOAD_DESTINATION_DIALOG = 0x000040;
    /// Disable modification of the download destination
    const LOCK_DOWNLOAD_DESTINATION_DIALOG= 0x000080;
    /// Disable tab display
    const DISABLE_TAB_DISPLAY = 0x000100;
    /// Hold analog controller when HOLD button is down
    const ENABLE_ANALOG_HOLD = 0x000200;
    /// Enable Flash Player
    const ENABLE_FLASH = 0x000400;
    /// Disable L/R triggers for back/forward
    const DISABLE_LRTRIGGER = 0x000800;
    }
}

/// OSK Field data
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUtilityOskData {
    /// Unknown. Pass 0.
    pub unk_00: i32,
    /// Unknown. Pass 0.
    pub unk_04: i32,
    /// One of `SceUtilityOskInputLanguage`
    pub language: SceUtilityOskInputLanguage,
    /// Unknown. Pass 0.
    pub unk_12: i32,
    /// One or more of `SceUtilityOskInputType` (types that are selectable by pressing SELECT)
    pub inputtype: SceUtilityOskInputType,
    /// Number of lines
    pub lines: i32,
    /// Unknown. Pass 0.
    pub unk_24: i32,
    /// Description text
    pub desc: *mut u16,
    /// Initial text
    pub intext: *mut u16,
    /// Length of output text
    pub outtextlength: i32,
    /// Pointer to the output text
    pub outtext: *mut u16,
    /// Result. One of `SceUtilityOskResult`
    pub result: SceUtilityOskResult,
    /// The max text that can be input
    pub outtextlimit: i32,
}

/// OSK parameters
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUtilityOskParams {
    pub base: UtilityDialogCommon,
    /// Number of input fields
    pub datacount: i32,
    /// Pointer to the start of the data for the input fields
    pub data: *mut SceUtilityOskData,
    /// The local OSK state, one of [`PspUtilityDialogState`]
    pub state: PspUtilityDialogState,
    /// Unknown. Pass 0
    pub unk_60: i32,
}

psp_extern! {
    #![name = "sceUtility"]
    #![flags = 0x4001]
    #![version = (0x00, 0x00)]

    #[psp(0x2AD8E239)]
    /// Create a message dialog
    ///
    /// # Parameters
    ///
    /// - `params`: dialog parameters
    /// # Return Value
    ///
    /// 0 on success
    pub fn sceUtilityMsgDialogInitStart(
        params: *mut UtilityMsgDialogParams,
    ) -> i32;

    #[psp(0x67AF3428)]
    /// Remove a message dialog currently active.  After calling this
    /// function you need to keep calling GetStatus and Update until
    /// you get a status of 4.
    pub fn sceUtilityMsgDialogShutdownStart();

    #[psp(0x9A1C91D7)]
    /// Get the current status of a message dialog currently active.
    ///
    /// # Return Value
    ///
    /// 2 if the GUI is visible (you need to call sceUtilityMsgDialogGetStatus).
    /// 3 if the user cancelled the dialog, and you need to call sceUtilityMsgDialogShutdownStart.
    /// 4 if the dialog has been successfully shut down.
    pub fn sceUtilityMsgDialogGetStatus() -> i32;

    #[psp(0x95FC253B)]
    /// Refresh the GUI for a message dialog currently active
    ///
    /// # Parameters
    ///
    /// - `n`: unknown, pass 1
    pub fn sceUtilityMsgDialogUpdate(n: i32);

    #[psp(0x4928BD96)]
    /// Abort a message dialog currently active
    pub fn sceUtilityMsgDialogAbort() -> i32;


    #[psp(0x4DB1E739)]
    /// Init the Network Configuration Dialog Utility
    ///
    /// # Parameters
    ///
    /// - `data`: pointer to NetconfData to be initialized
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceUtilityNetconfInitStart(data: *mut UtilityNetconfData) -> i32;

    #[psp(0xF88155F6)]
    /// Shutdown the Network Configuration Dialog Utility
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceUtilityNetconfShutdownStart() -> i32;

    #[psp(0x91E70E35)]
    /// Update the Network Configuration Dialog GUI
    ///
    /// # Parameters
    ///
    /// - `unknown`: unknown; set to 1
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceUtilityNetconfUpdate(unknown: i32) -> i32;

    #[psp(0x6332AA39)]
    /// Get the status of a running Network Configuration Dialog
    ///
    /// # Return Value
    ///
    /// one of [`PspUtilityDialogState`] on success, < 0 on error
    pub fn sceUtilityNetconfGetStatus() -> i32;

    #[psp(0x5EEE6548)]
    /// Check existance of a Net Configuration
    ///
    /// # Parameters
    ///
    /// - `id`: id of net Configuration (1 to n)
    /// # Return Value
    ///
    /// 0 on success,
    pub fn sceUtilityCheckNetParam(id: i32) -> i32;

    #[psp(0x434D4B3A)]
    /// Get Net Configuration Parameter
    ///
    /// # Parameters
    ///
    /// - `conf`: Net Configuration number (1 to n)
    /// (0 returns valid but seems to be a copy of the last config requested)
    /// # Parameters
    ///
    /// - `param`: which parameter to get
    /// - `data`: parameter data
    /// # Return Value
    ///
    /// 0 on success,
    pub fn sceUtilityGetNetParam(
        conf: i32,
        param: NetParam,
        data: *mut UtilityNetData,
    ) -> i32;

    #[psp(0x50C4CD57)]
    /// Saves or Load savedata to/from the passed structure
    /// After having called this continue calling sceUtilitySavedataGetStatus to
    /// check if the operation is completed
    ///
    /// # Parameters
    ///
    /// - `params`: savedata parameters
    /// # Return Value
    ///
    /// 0 on success
    pub fn sceUtilitySavedataInitStart(
        params: *mut SceUtilitySavedataParam,
    ) -> i32;

    #[psp(0x8874DBE0)]
    /// Check the current status of the saving/loading/shutdown process
    /// Continue calling this to check current status of the process
    /// before calling this call also sceUtilitySavedataUpdate
    /// # Return Value
    ///
    /// - 2 if the process is still being processed.
    /// - 3 on save/load success, then you can call sceUtilitySavedataShutdownStart.
    /// - 4 on complete shutdown.
    pub fn sceUtilitySavedataGetStatus() -> i32;

    #[psp(0x9790B33C)]
    /// Shutdown the savedata utility. after calling this continue calling
    /// sceUtilitySavedataGetStatus to check when it has shutdown
    ///
    /// # Return Value
    ///
    /// 0 on success
    ///
    pub fn sceUtilitySavedataShutdownStart() -> i32;

    #[psp(0xD4B95FFB)]
    /// Refresh status of the savedata function
    ///
    /// # Parameters
    ///
    /// - `unknown`: unknown, pass 1
    pub fn sceUtilitySavedataUpdate(unknown: i32);

    #[psp(0xC492F751)]
    /// Init the game sharing
    ///
    /// # Parameters
    ///
    /// - `params`: game sharing parameters
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceUtilityGameSharingInitStart(
        params: *mut UtilityGameSharingParams,
    ) -> i32;

    #[psp(0xEFC6F80F)]
    /// Shutdown game sharing.
    pub fn sceUtilityGameSharingShutdownStart();

    #[psp(0x946963F3)]
    /// Get the current status of game sharing.
    ///
    /// # Return Value
    ///
    /// 2 if the GUI is visible (you need to call sceUtilityGameSharingGetStatus).
    /// 3 if the user cancelled the dialog, and you need to call
    ///   sceUtilityGameSharingShutdownStart.
    /// 4 if the dialog has been successfully shut down.
    pub fn sceUtilityGameSharingGetStatus() -> i32;

    #[psp(0x7853182D)]
    /// Refresh the GUI for game sharing
    ///
    /// # Parameters
    ///
    /// - `n`: unknown, pass 1
    pub fn sceUtilityGameSharingUpdate(n: i32);

    #[psp(0xCDC3AA41)]
    /// Init the html viewer
    ///
    /// # Parameters
    ///
    /// - `params`: html viewer parameters
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceUtilityHtmlViewerInitStart(
        params: *mut UtilityHtmlViewerParam,
    ) -> i32;

    #[psp(0xF5CE1134)]
    /// Shutdown html viewer.
    pub fn sceUtilityHtmlViewerShutdownStart() -> i32;

    #[psp(0x05AFB9E4)]
    /// Refresh the GUI for html viewer
    ///
    /// # Parameters
    ///
    /// - `n`: unknown, pass 1
    pub fn sceUtilityHtmlViewerUpdate(n: i32) -> i32;

    #[psp(0xBDA7D894)]
    /// Get the current status of the html viewer.
    ///
    /// # Return Value
    ///
    /// 2 if the GUI is visible (you need to call sceUtilityHtmlViewerGetStatus).
    /// 3 if the user cancelled the dialog, and you need to call
    ///   sceUtilityHtmlViewerShutdownStart.
    /// 4 if the dialog has been successfully shut down.
    pub fn sceUtilityHtmlViewerGetStatus() -> i32;

    #[psp(0x45C18506)]
    /// Set Integer System Parameter
    ///
    /// # Parameters
    ///
    /// - `id`: which parameter to set
    /// - `value`: integer value to set
    /// # Return Value
    ///
    /// 0 on success, 0x80110103 on failure
    pub fn sceUtilitySetSystemParamInt(
        id: SystemParamId,
        value: i32,
    ) -> i32;

    #[psp(0x41E30674)]
    /// Set String System Parameter
    ///
    /// # Parameters
    ///
    /// - `id`: which parameter to set
    /// - `str`: char * value to set
    /// # Return Value
    ///
    /// 0 on success, 0x80110103 on failure
    pub fn sceUtilitySetSystemParamString(
        id: SystemParamId,
        str: *const u8,
    ) -> i32;

    #[psp(0xA5DA2406)]
    /// Get Integer System Parameter
    ///
    /// # Parameters
    ///
    /// - `id`: which parameter to get
    /// - `value`: pointer to integer value to place result in
    /// # Return Value
    ///
    /// 0 on success, 0x80110103 on failure
    pub fn sceUtilityGetSystemParamInt(
        id: SystemParamId,
        value: *mut i32,
    ) -> i32;

    #[psp(0x34B78343)]
    /// Get String System Parameter
    ///
    /// # Parameters
    ///
    /// - `id`: which parameter to get
    /// - `str`: char * buffer to place result in
    /// - `len`: length of str buffer
    /// # Return Value
    ///
    /// 0 on success, 0x80110103 on failure
    pub fn sceUtilityGetSystemParamString(
        id: SystemParamId,
        str: *mut u8,
        len: i32,
    ) -> i32;

    #[psp(0xF6269B82)]
    /// Create an on-screen keyboard
    ///
    /// # Parameters
    ///
    /// - `params`: OSK parameters.
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceUtilityOskInitStart(params: *mut SceUtilityOskParams) -> i32;

    #[psp(0x3DFAEBA9)]
    /// Remove a currently active keyboard. After calling this function you must
    ///
    /// poll sceUtilityOskGetStatus() until it returns None
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceUtilityOskShutdownStart() -> i32;

    #[psp(0x4B85C861)]
    /// Refresh the GUI for a keyboard currently active
    ///
    /// # Parameters
    ///
    /// - `n`: Unknown, pass 1.
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceUtilityOskUpdate(n: i32) -> i32;

    #[psp(0xF3F76017)]
    /// Get the status of a on-screen keyboard currently active.
    ///
    /// # Example
    ///
    /// ```
    /// unsafe {
    ///     let status: PspUtilityDialogState = sceUtilityOskGetStatus().try_into().unwrap();
    ///     match status {
    ///         PspUtilityDialogState::Visible => {
    ///             // do something
    ///        }
    ///        // ...
    ///    }
    /// }
    /// ```
    ///
    /// # Return Value
    ///
    /// the current status of the keyboard. See [`PspUtilityDialogState`] for details.
    pub fn sceUtilityOskGetStatus() -> i32;

    #[psp(0x1579a159)]
    /// Load a network module (PRX) from user mode.
    /// Load PSP_NET_MODULE_COMMON and PSP_NET_MODULE_INET
    /// to use infrastructure WifI (via an access point).
    /// Available on firmware 2.00 and higher only.
    ///
    /// # Parameters
    ///
    /// - `module`: module number to load (PSP_NET_MODULE_xxx)
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceUtilityLoadNetModule(module: NetModule) -> i32;

    #[psp(0x64d50c56)]
    /// Unload a network module (PRX) from user mode.
    /// Available on firmware 2.00 and higher only.
    ///
    /// # Parameters
    ///
    /// - `module`: module number be unloaded
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceUtilityUnloadNetModule(module: NetModule) -> i32;

    #[psp(0xC629AF26)]
    /// Load an audio/video module (PRX) from user mode.
    ///
    /// Available on firmware 2.00 and higher only.
    ///
    /// # Parameters
    ///
    /// - `module`: module number to load (PSP_AV_MODULE_xxx)
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceUtilityLoadAvModule(module: AvModule) -> i32;

    #[psp(0xF7D8D092)]
    /// Unload an audio/video module (PRX) from user mode.
    /// Available on firmware 2.00 and higher only.
    ///
    /// # Parameters
    ///
    /// - `module`: module number to be unloaded
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceUtilityUnloadAvModule(module: AvModule) -> i32;

    #[psp(0x0D5BC6D2)]
    /// Load a usb module (PRX) from user mode.
    /// Available on firmware 2.70 and higher only.
    ///
    /// # Parameters
    ///
    /// - `module`: module number to load (PSP_USB_MODULE_xxx)
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceUtilityLoadUsbModule(module: UsbModule) -> i32;

    #[psp(0xF64910F0)]
    /// Unload a usb module (PRX) from user mode.
    /// Available on firmware 2.70 and higher only.
    ///
    /// # Parameters
    ///
    /// - `module`: module number to be unloaded
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceUtilityUnloadUsbModule(module: UsbModule) -> i32;

    #[psp(0x2A2B3DE0)]
    /// Load a module (PRX) from user mode.
    ///
    /// # Parameters
    ///
    /// - `module`: module to load (PSP_MODULE_xxx)
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceUtilityLoadModule(module: Module) -> i32;

    #[psp(0xE49BFE92)]
    /// Unload a module (PRX) from user mode.
    ///
    /// # Parameters
    ///
    /// - `module`: module to unload (PSP_MODULE_xxx)
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceUtilityUnloadModule(module: Module) -> i32;

}

psp_extern! {
    #![name = "sceUtility_netparam_internal"]
    #![flags = 0x4001]
    #![version = (0x00, 0x00)]


    #[psp(0x072DEBF2)]
    /// Create a new Network Configuration
    /// @note This creates a new configuration at conf and clears 0
    ///
    /// # Parameters
    ///
    /// - `conf`: Net Configuration number (1 to n)
    ///
    /// # Return Value
    ///
    /// 0 on success
    pub fn sceUtilityCreateNetParam(conf: i32) -> i32;

    #[psp(0xFC4516F3)]
    /// Sets a network parameter
    ///
    /// # Note
    ///
    /// This sets only to configuration 0
    ///
    /// # Parameters
    ///
    /// - `param`: Which parameter to set
    /// - `val`: Pointer to the the data to set
    ///
    /// # Return Value
    ///
    /// 0 on success
    pub fn sceUtilitySetNetParam(
        param: NetParam,
        val: *const c_void,
    ) -> i32;

    #[psp(0xFB0C4840)]
    /// Copies a Network Configuration to another
    ///
    /// # Parameters
    ///
    /// - `src`: Source Net Configuration number (0 to n)
    /// - `dest`: Destination Net Configuration number (0 to n)
    ///
    /// # Return Value
    ///
    /// 0 on success
    pub fn sceUtilityCopyNetParam(
        src: i32,
        dest: i32,
    ) -> i32;

    #[psp(0x9CE50172)]
    /// Deletes a Network Configuration
    ///
    /// # Parameters
    ///
    /// - `conf`: Net Configuration number (1 to n)
    ///
    /// # Return Value
    ///
    /// 0 on success
    pub fn sceUtilityDeleteNetParam(conf: i32) -> i32;

}
