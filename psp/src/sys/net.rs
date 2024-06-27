use crate::eabi::{i5, i6};
use core::ffi::c_void;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetMallocStat {
    pub pool: i32,
    pub maximum: i32,
    pub free: i32,
}

psp_extern! {
    #![name = "sceNet"]
    #![flags = 0x0009]
    #![version = (0x00, 0x00)]

    #[psp(0x39AF39A6)]
    /// Initialise the networking library
    ///
    /// # Parameters
    ///
    /// - `poolsize`: Memory pool size (appears to be for the whole of the networking library).
    /// - `calloutprio`: Priority of the SceNetCallout thread.
    /// - `calloutstack`: Stack size of the SceNetCallout thread (defaults to 4096 on non 1.5 firmware regardless of what value is passed).
    /// - `netintrprio`: Priority of the SceNetNetintr thread.
    /// - `netintrstack`: Stack size of the SceNetNetintr thread (defaults to 4096 on non 1.5 firmware regardless of what value is passed).
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceNetInit(
        poolsize: i32,
        calloutprio: i32,
        calloutstack: i32,
        netintrprio: i32,
        netintrstack: i32,
    ) -> i32;

    #[psp(0x281928A9)]
    /// Terminate the networking library
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceNetTerm() -> i32;

    #[psp(0x50647530)]
    /// Free (delete) thread info/data
    ///
    /// # Parameters
    ///
    /// - `thid`: The thread id.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceNetFreeThreadinfo(thid: i32) -> i32;

    #[psp(0xAD6844C6)]
    /// Abort a thread
    ///
    /// # Parameters
    ///
    /// - `thid`: The thread id.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceNetThreadAbort(thid: i32) -> i32;

    #[psp(0xD27961C9)]
    /// Convert string to a Mac address
    ///
    /// # Parameters
    ///
    /// - `name`: The string to convert.
    /// - `mac`: Pointer to a buffer to store the result.
    pub fn sceNetEtherStrton(name: *mut u8, mac: *mut u8);

    #[psp(0x89360950)]
    /// Convert Mac address to a string
    ///
    /// # Parameters
    ///
    /// - `mac`: The Mac address to convert.
    /// - `name`: Pointer to a buffer to store the result.
    pub fn sceNetEtherNtostr(mac: *mut u8, name: *mut u8);

    #[psp(0x0BF0A3AE)]
    /// Retrieve the local Mac address
    ///
    /// # Parameters
    ///
    /// - `mac`: Pointer to a buffer to store the result.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceNetGetLocalEtherAddr(mac: *mut u8) -> i32;

    #[psp(0xCC393E48)]
    /// Retrieve the networking library memory usage
    ///
    /// # Parameters
    ///
    /// - `stat`: Pointer to a ::SceNetMallocStat type to store the result.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceNetGetMallocStat(stat: *mut SceNetMallocStat) -> i32;

}

/// Adhoc ID structure
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetAdhocctlAdhocId {
    /// Unknown, set to 0, other values used are 1 and 2. Not sure on what they represent
    pub unknown: i32,
    /// The adhoc ID string
    pub adhoc_id: [u8; 9usize],
    pub unk: [u8; 3usize],
}
/// Peer info structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceNetAdhocctlPeerInfo {
    pub next: *mut SceNetAdhocctlPeerInfo,
    /// Nickname
    pub nickname: [u8; 128usize],
    /// Mac address
    pub mac: [u8; 6usize],
    /// Unknown
    pub unknown: [u8; 6usize],
    /// Time stamp
    pub timestamp: u32,
}
/// Scan info structure
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetAdhocctlScanInfo {
    pub next: *mut SceNetAdhocctlScanInfo,
    /// Channel number
    pub channel: i32,
    /// Name of the connection (alphanumeric characters only)
    pub name: [u8; 8usize],
    /// The BSSID
    pub bssid: [u8; 6usize],
    /// Unknown
    pub unknown: [u8; 2usize],
    /// Unknown
    pub unknown2: i32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetAdhocctlGameModeInfo {
    /// Number of peers (including self)
    pub count: i32,
    /// MAC addresses of peers (including self)
    pub macs: [[u8; 6usize]; 16usize],
}
/// Params structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceNetAdhocctlParams {
    /// Channel number
    pub channel: i32,
    /// Name of the connection
    pub name: [u8; 8usize],
    /// The BSSID
    pub bssid: [u8; 6usize],
    /// Nickname
    pub nickname: [u8; 128usize],
}

pub type SceNetAdhocctlHandler =
    Option<unsafe extern "C" fn(flag: i32, error: i32, unknown: *mut c_void)>;

psp_extern! {
    #![name = "sceNetAdhocctl"]
    #![flags = 0x0009]
    #![version = (0x00, 0x00)]

    #[psp(0xE26F226E)]
    /// Initialise the Adhoc control library
    ///
    /// # Parameters
    ///
    /// - `stacksize`: Stack size of the adhocctl thread. Set to 0x2000
    /// - `priority`: Priority of the adhocctl thread. Set to 0x30
    /// - `adhoc_id`: Pass a filled in ::SceNetAdhocctlAdhocId
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceNetAdhocctlInit(
        stacksize: i32,
        priority: i32,
        adhoc_id: *mut SceNetAdhocctlAdhocId,
    ) -> i32;

    #[psp(0x9D689E13)]
    /// Terminate the Adhoc control library
    ///
    /// # Return Value
    ///
    /// 0 on success, < on error.
    pub fn sceNetAdhocctlTerm() -> i32;

    #[psp(0x0AD043ED)]
    /// Connect to the Adhoc control
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the connection (maximum 8 alphanumeric characters).
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceNetAdhocctlConnect(name: *const u8) -> i32;

    #[psp(0x34401D65)]
    /// Disconnect from the Adhoc control
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceNetAdhocctlDisconnect() -> i32;

    #[psp(0x75ECD386)]
    /// Get the state of the Adhoc control
    ///
    /// # Parameters
    ///
    /// - `event`: Pointer to an integer to receive the status. Can continue when it becomes 1.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceNetAdhocctlGetState(event: *mut i32) -> i32;

    #[psp(0xEC0635C1)]
    /// Connect to the Adhoc control (as a host)
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the connection (maximum 8 alphanumeric characters).
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceNetAdhocctlCreate(name: *const u8) -> i32;

    #[psp(0x5E7F79C9)]
    /// Connect to the Adhoc control (as a client)
    ///
    /// # Parameters
    ///
    /// - `scaninfo`: A valid ::SceNetAdhocctlScanInfo struct that has been filled by sceNetAchocctlGetScanInfo
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceNetAdhocctlJoin(scaninfo: *mut SceNetAdhocctlScanInfo) -> i32;

    #[psp(0x362CBE8F)]
    /// Get the adhoc ID
    ///
    /// # Parameters
    ///
    /// - `id`: A pointer to a  ::SceNetAdhocctlAdhocId
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceNetAdhocctlGetAdhocId(id: *mut SceNetAdhocctlAdhocId) -> i32;

    #[psp(0xA5C055CE)]
    /// Connect to the Adhoc control game mode (as a host)
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the connection (maximum 8 alphanumeric characters).
    /// - `unknown`: Pass 1.
    /// - `num`: The total number of players (including the host).
    /// - `macs`: A pointer to a list of the participating mac addresses, host first, then clients.
    /// - `timeout`: Timeout in microseconds.
    /// - `unknown2`: pass 0.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceNetAdhocctlCreateEnterGameMode(
        name: *const u8,
        unknown: i32,
        num: i32,
        macs: *mut u8,
        timeout: u32,
        unknown2: i32,
    ) -> i32;

    #[psp(0x1FF89745)]
    /// Connect to the Adhoc control game mode (as a client)
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the connection (maximum 8 alphanumeric characters).
    /// - `hostmac`: The mac address of the host.
    /// - `timeout`: Timeout in microseconds.
    /// - `unknown`: pass 0.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceNetAdhocctlJoinEnterGameMode(
        name: *const u8,
        hostmac: *mut u8,
        timeout: u32,
        unknown: i32,
    ) -> i32;

    #[psp(0x5A014CE0)]
    /// Get game mode information
    ///
    /// # Parameters
    ///
    /// - `gamemodeinfo`: Pointer to store the info.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceNetAdhocctlGetGameModeInfo(
        gamemodeinfo: *mut SceNetAdhocctlGameModeInfo,
    ) -> i32;

    #[psp(0xCF8E084D)]
    /// Exit game mode.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceNetAdhocctlExitGameMode() -> i32;

    #[psp(0xE162CB14)]
    /// Get a list of peers
    ///
    /// # Parameters
    ///
    /// - `length`: The length of the list.
    /// - `buf`: An allocated area of size length.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceNetAdhocctlGetPeerList(
        length: *mut i32,
        buf: *mut c_void,
    ) -> i32;

    #[psp(0x8DB83FDC)]
    /// Get peer information
    ///
    /// # Parameters
    ///
    /// - `mac`: The mac address of the peer.
    /// - `size`: Size of peerinfo.
    /// - `peerinfo`: Pointer to store the information.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceNetAdhocctlGetPeerInfo(
        mac: *mut u8,
        size: i32,
        peerinfo: *mut SceNetAdhocctlPeerInfo,
    ) -> i32;

    #[psp(0x08FFF7A0)]
    /// Scan the adhoc channels
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceNetAdhocctlScan() -> i32;

    #[psp(0x81AEE1BE)]
    /// Get the results of a scan
    ///
    /// # Parameters
    ///
    /// - `length`: The length of the list.
    /// - `buf`: An allocated area of size length.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceNetAdhocctlGetScanInfo(
        length: *mut i32,
        buf: *mut c_void,
    ) -> i32;

    #[psp(0x20B317A0)]
    /// Register an adhoc event handler
    ///
    /// # Parameters
    ///
    /// - `handler`: The event handler.
    /// - `unknown`: Pass NULL.
    ///
    /// # Return Value
    ///
    /// Handler id on success, < 0 on error.
    pub fn sceNetAdhocctlAddHandler(
        handler: SceNetAdhocctlHandler,
        unknown: *mut c_void,
    ) -> i32;

    #[psp(0x6402490B)]
    /// Delete an adhoc event handler
    ///
    /// # Parameters
    ///
    /// - `id`: The handler id as returned by sceNetAdhocctlAddHandler.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceNetAdhocctlDelHandler(id: i32) -> i32;

    #[psp(0x8916C003)]
    /// Get nickname from a mac address
    ///
    /// # Parameters
    ///
    /// - `mac`: The mac address.
    /// - `nickname`: Pointer to a char buffer where the nickname will be stored.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceNetAdhocctlGetNameByAddr(
        mac: *mut u8,
        nickname: *mut u8,
    ) -> i32;

    #[psp(0x99560ABE)]
    /// Get mac address from nickname
    ///
    /// # Parameters
    ///
    /// - `nickname`: The nickname.
    /// - `length`: The length of the list.
    /// - `buf`: An allocated area of size length.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceNetAdhocctlGetAddrByName(
        nickname: *mut u8,
        length: *mut i32,
        buf: *mut c_void,
    ) -> i32;

    #[psp(0xDED9D28E)]
    /// Get Adhocctl parameter
    ///
    /// # Parameters
    ///
    /// - `params`: Pointer to a ::SceNetAdhocctlParams
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceNetAdhocctlGetParameter(params: *mut SceNetAdhocctlParams) -> i32;

}

/// PTP status structure
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetAdhocPtpStat {
    /// Pointer to next PTP structure in list
    pub next: *mut SceNetAdhocPtpStat,
    /// ptp ID
    pub ptp_id: i32,
    /// MAC address
    pub mac: [u8; 6usize],
    /// Peer MAC address
    pub peermac: [u8; 6usize],
    /// Port
    pub port: u16,
    /// Peer Port
    pub peerport: u16,
    /// Bytes sent
    pub sent_data: u32,
    /// Bytes received
    pub rcvd_data: u32,
    /// Unknown
    pub state: ScePspnetAdhocPtpState,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum ScePspnetAdhocPtpState {
    Closed,
    Listen,
    SynSent,
    SynReceived,
    Established,
}

/// PDP status structure
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetAdhocPdpStat {
    /// Pointer to next PDP structure in list
    pub next: *mut SceNetAdhocPdpStat,
    /// pdp ID
    pub pdp_id: i32,
    /// MAC address
    pub mac: [u8; 6usize],
    /// Port
    pub port: u16,
    /// Bytes received
    pub rcvd_data: u32,
}

psp_extern! {
    #![name = "sceNetAdhoc"]
    #![flags = 0x0009]
    #![version = (0x00, 0x00)]

    #[psp(0xE1D621D7)]
    /// Initialise the adhoc library.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceNetAdhocInit() -> i32;

    #[psp(0xA62C6F57)]
    /// Terminate the adhoc library
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceNetAdhocTerm() -> i32;

    #[psp(0x6F92741B)]
    /// Create a PDP object.
    ///
    /// # Parameters
    ///
    /// - `mac`: Your MAC address (from sceWlanGetEtherAddr)
    /// - `port`: Port to use, lumines uses 0x309
    /// - `buf_size`: Socket buffer size, lumines sets to 0x400
    /// - `unk1`: Unknown, lumines sets to 0
    ///
    /// # Return Value
    ///
    /// The ID of the PDP object (< 0 on error)
    pub fn sceNetAdhocPdpCreate(
        mac: *mut u8,
        port: u16,
        buf_size: u32,
        unk1: i32,
    ) -> i32;

    #[psp(0x7F27BB5E)]
    /// Delete a PDP object.
    ///
    /// # Parameters
    ///
    /// - `id`: The ID returned from ::sceNetAdhocPdpCreate
    /// - `unk1`: Unknown, set to 0
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceNetAdhocPdpDelete(
        id: i32,
        unk1: i32,
    ) -> i32;

    #[psp(0xABED3790)]
    /// Set a PDP packet to a destination
    ///
    /// # Parameters
    ///
    /// - `id`: The ID as returned by ::sceNetAdhocPdpCreate
    /// - `dest_mac_addr`: The destination MAC address, can be set to all 0xFF for broadcast
    /// - `port`: The port to send to
    /// - `data`: The data to send
    /// - `len`: The length of the data.
    /// - `timeout`: Timeout in microseconds.
    /// - `nonblock`: Set to 0 to block, 1 for non-blocking.
    ///
    /// # Return Value
    ///
    /// Bytes sent, < 0 on error
    pub fn sceNetAdhocPdpSend(
        id: i32,
        dest_mac_addr: *mut u8,
        port: u16,
        data: *mut c_void,
        len: u32,
        timeout: u32,
        nonblock: i32,
    ) -> i32;

    #[psp(0xDFE53E03)]
    /// Receive a PDP packet
    ///
    /// # Parameters
    ///
    /// - `id`: The ID of the PDP object, as returned by ::sceNetAdhocPdpCreate
    /// - `src_mac_addr`: Buffer to hold the source mac address of the sender
    /// - `port`: Buffer to hold the port number of the received data
    /// - `data`: Data buffer
    /// - `data_length`: The length of the data buffer
    /// - `timeout`: Timeout in microseconds.
    /// - `nonblock`: Set to 0 to block, 1 for non-blocking.
    ///
    /// # Return Value
    ///
    /// Number of bytes received, < 0 on error.
    pub fn sceNetAdhocPdpRecv(
        id: i32,
        src_mac_addr: *mut u8,
        port: *mut u16,
        data: *mut c_void,
        data_length: *mut c_void,
        timeout: u32,
        nonblock: i32,
    ) -> i32;

    #[psp(0xC7C1FC57)]
    /// Get the status of all PDP objects
    ///
    /// # Parameters
    ///
    /// - `size`: Pointer to the size of the stat array (e.g 20 for one structure)
    /// - `stat`: Pointer to a list of ::SceNetAdhocPdpStat structures.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceNetAdhocGetPdpStat(
        size: *mut i32,
        stat: *mut SceNetAdhocPdpStat,
    ) -> i32;

    #[psp(0x7F75C338)]
    /// Create own game object type data.
    ///
    /// # Parameters
    ///
    /// - `data`: A pointer to the game object data.
    /// - `size`: Size of the game data.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceNetAdhocGameModeCreateMaster(
        data: *mut c_void,
        size: i32,
    ) -> i32;

    #[psp(0x3278AB0C)]
    /// Create peer game object type data.
    ///
    /// # Parameters
    ///
    /// - `mac`: The mac address of the peer.
    /// - `data`: A pointer to the game object data.
    /// - `size`: Size of the game data.
    ///
    /// # Return Value
    ///
    /// The id of the replica on success, < 0 on error.
    pub fn sceNetAdhocGameModeCreateReplica(
        mac: *mut u8,
        data: *mut c_void,
        size: i32,
    ) -> i32;

    #[psp(0x98C204C8)]
    /// Update own game object type data.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceNetAdhocGameModeUpdateMaster() -> i32;

    #[psp(0xFA324B4E)]
    /// Update peer game object type data.
    ///
    /// # Parameters
    ///
    /// - `id`: The id of the replica returned by sceNetAdhocGameModeCreateReplica.
    /// - `unk1`: Pass 0.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceNetAdhocGameModeUpdateReplica(
        id: i32,
        unk1: i32,
    ) -> i32;

    #[psp(0xA0229362)]
    /// Delete own game object type data.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceNetAdhocGameModeDeleteMaster() -> i32;

    #[psp(0x0B2228E9)]
    /// Delete peer game object type data.
    ///
    /// # Parameters
    ///
    /// - `id`: The id of the replica.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceNetAdhocGameModeDeleteReplica(id: i32) -> i32;

    #[psp(0x877F6D66)]
    /// Open a PTP connection
    ///
    /// # Parameters
    ///
    /// - `srcmac`: Local mac address.
    /// - `srcport`: Local port.
    /// - `destmac`: Destination mac.
    /// - `destport`: Destination port
    /// - `buf_size`: Socket buffer size
    /// - `delay`: Interval between retrying (microseconds).
    /// - `count`: Number of retries.
    /// - `unk1`: Pass 0.
    ///
    /// # Return Value
    ///
    /// A socket ID on success, < 0 on error.
    pub fn sceNetAdhocPtpOpen(
        srcmac: *mut u8,
        srcport: u16,
        destmac: *mut u8,
        destport: u16,
        buf_size: u32,
        delay: u32,
        count: i32,
        unk1: i32,
    ) -> i32;

    #[psp(0xFC6FC07B)]
    /// Wait for connection created by sceNetAdhocPtpOpen()
    ///
    /// # Parameters
    ///
    /// - `id`: A socket ID.
    /// - `timeout`: Timeout in microseconds.
    /// - `nonblock`: Set to 0 to block, 1 for non-blocking.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceNetAdhocPtpConnect(
        id: i32,
        timeout: u32,
        nonblock: i32,
    ) -> i32;

    #[psp(0xE08BDAC1)]
    /// Wait for an incoming PTP connection
    ///
    /// # Parameters
    ///
    /// - `srcmac`: Local mac address.
    /// - `srcport`: Local port.
    /// - `buf_size`: Socket buffer size
    /// - `delay`: Interval between retrying (microseconds).
    /// - `count`: Number of retries.
    /// - `queue`: Connection queue length.
    /// - `unk1`: Pass 0.
    ///
    /// # Return Value
    ///
    /// A socket ID on success, < 0 on error.
    pub fn sceNetAdhocPtpListen(
        srcmac: *mut u8,
        srcport: u16,
        buf_size: u32,
        delay: u32,
        count: i32,
        queue: i32,
        unk1: i32,
    ) -> i32;

    #[psp(0x9DF81198)]
    /// Accept an incoming PTP connection
    ///
    /// # Parameters
    ///
    /// - `id`: A socket ID.
    /// - `mac`: Connecting peers mac.
    /// - `port`: Connecting peers port.
    /// - `timeout`: Timeout in microseconds.
    /// - `nonblock`: Set to 0 to block, 1 for non-blocking.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceNetAdhocPtpAccept(
        id: i32,
        mac: *mut u8,
        port: *mut u16,
        timeout: u32,
        nonblock: i32,
    ) -> i32;

    #[psp(0x4DA4C788)]
    /// Send data
    ///
    /// # Parameters
    ///
    /// - `id`: A socket ID.
    /// - `data`: Data to send.
    /// - `data_size`: Size of the data.
    /// - `timeout`: Timeout in microseconds.
    /// - `nonblock`: Set to 0 to block, 1 for non-blocking.
    ///
    /// # Return Value
    ///
    /// 0 success, < 0 on error.
    pub fn sceNetAdhocPtpSend(
        id: i32,
        data: *mut c_void,
        data_size: *mut i32,
        timeout: u32,
        nonblock: i32,
    ) -> i32;

    #[psp(0x8BEA2B3E)]
    /// Receive data
    ///
    /// # Parameters
    ///
    /// - `id`: A socket ID.
    /// - `data`: Buffer for the received data.
    /// - `data_size`: Size of the data received.
    /// - `timeout`: Timeout in microseconds.
    /// - `nonblock`: Set to 0 to block, 1 for non-blocking.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceNetAdhocPtpRecv(
        id: i32,
        data: *mut c_void,
        data_size: *mut i32,
        timeout: u32,
        nonblock: i32,
    ) -> i32;

    #[psp(0x9AC2EEAC)]
    /// Wait for data in the buffer to be sent
    ///
    /// # Parameters
    ///
    /// - `id`: A socket ID.
    /// - `timeout`: Timeout in microseconds.
    /// - `nonblock`: Set to 0 to block, 1 for non-blocking.
    ///
    /// # Return Value
    ///
    /// A socket ID on success, < 0 on error.
    pub fn sceNetAdhocPtpFlush(
        id: i32,
        timeout: u32,
        nonblock: i32,
    ) -> i32;

    #[psp(0x157E6225)]
    /// Close a socket
    ///
    /// # Parameters
    ///
    /// - `id`: A socket ID.
    /// - `unk1`: Pass 0.
    ///
    /// # Return Value
    ///
    /// A socket ID on success, < 0 on error.
    pub fn sceNetAdhocPtpClose(
        id: i32,
        unk1: i32,
    ) -> i32;

    #[psp(0xB9685118)]
    /// Get the status of all PTP objects
    ///
    /// # Parameters
    ///
    /// - `size`: Pointer to the size of the stat array (e.g 20 for one structure)
    /// - `stat`: Pointer to a list of ::SceNetAdhocPtpStat structures.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceNetAdhocGetPtpStat(
        size: *mut i32,
        stat: *mut SceNetAdhocPtpStat,
    ) -> i32;

}

/// Linked list for sceNetAdhocMatchingGetMembers
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AdhocPoolStat {
    /// Size of the pool
    pub size: i32,
    /// Maximum size of the pool
    pub maxsize: i32,
    /// Unused memory in the pool
    pub freesize: i32,
}

/// Matching callback
pub type AdhocMatchingCallback = Option<
    unsafe extern "C" fn(
        matching_id: i32,
        event: i32,
        mac: *mut u8,
        opt_len: i32,
        opt_data: *mut c_void,
    ),
>;

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum AdhocMatchingMode {
    Host = 1,
    Client,
    Ptp,
}

psp_extern! {
    #![name = "sceNetAdhocMatching"]
    #![flags = 0x0009]
    #![version = (0x00, 0x00)]

    #[psp(0x2A2A1E07)]
    /// Initialise the Adhoc matching library
    ///
    /// # Parameters
    ///
    /// - `memsize`: Internal memory pool size. Lumines uses 0x20000
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceNetAdhocMatchingInit(memsize: i32) -> i32;

    #[psp(0x7945ECDA)]
    /// Terminate the Adhoc matching library
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceNetAdhocMatchingTerm() -> i32;

    #[psp(0xCA5EDA6F)]
    /// Create an Adhoc matching object
    ///
    /// # Parameters
    ///
    /// - `mode`: One of ::AdhocMatchingMode
    /// - `max_peers`: Maximum number of peers to match (only used when mode is Host)
    /// - `port`: Port. Lumines uses 0x22B
    /// - `buf_size`: Receiving buffer size
    /// - `hello_delay`: Hello message send delay in microseconds (only used when mode is PHost or PTP)
    /// - `ping_delay`: Ping send delay in microseconds. Lumines uses 0x5B8D80 (only used when mode is Host or Ptp)
    /// - `init_count`: Initial count of the of the resend counter. Lumines uses 3
    /// - `msg_delay`: Message send delay in microseconds
    /// - `callback`: Callback to be called for matching
    ///
    /// # Return Value
    ///
    /// ID of object on success, < 0 on error.
    pub fn sceNetAdhocMatchingCreate(
        mode: AdhocMatchingMode,
        max_peers: i32,
        port: u16,
        buf_size: i32,
        hello_delay: u32,
        ping_delay: u32,
        init_count: i32,
        msg_delay: u32,
        callback: AdhocMatchingCallback,
    ) -> i32;

    #[psp(0xF16EAF4F)]
    /// Delete an Adhoc matching object
    ///
    /// # Parameters
    ///
    /// - `matching_id`: The ID returned from ::sceNetAdhocMatchingCreate
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceNetAdhocMatchingDelete(matching_id: i32) -> i32;

    #[psp(0x93EF3843)]
    /// Start a matching object
    ///
    /// # Parameters
    ///
    /// - `matching_id`: The ID returned from ::sceNetAdhocMatchingCreate
    /// - `evth_pri`: Priority of the event handler thread. Lumines uses 0x10
    /// - `evth_stack`: Stack size of the event handler thread. Lumines uses 0x2000
    /// - `inth_pri`: Priority of the input handler thread. Lumines uses 0x10
    /// - `inth_stack`: Stack size of the input handler thread. Lumines uses 0x2000
    /// - `opt_len`: Size of hellodata
    /// - `opt_data`: Pointer to block of data passed to callback
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceNetAdhocMatchingStart(
        matching_id: i32,
        evth_pri: i32,
        evth_stack: i32,
        inth_pri: i32,
        inth_stack: i32,
        opt_len: i32,
        opt_data: *mut c_void,
    ) -> i32;

    #[psp(0x32B156B3)]
    /// Stop a matching object
    ///
    /// # Parameters
    ///
    /// - `matching_id`: The ID returned from ::sceNetAdhocMatchingCreate
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceNetAdhocMatchingStop(matching_id: i32) -> i32;

    #[psp(0x5E3D4B79)]
    /// Select a matching target
    ///
    /// # Parameters
    ///
    /// - `matching_id`: The ID returned from ::sceNetAdhocMatchingCreate
    /// - `mac`: MAC address to select
    /// - `opt_len`: Optional data length
    /// - `opt_data`: Pointer to the optional data
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceNetAdhocMatchingSelectTarget(
        matching_id: i32,
        mac: *mut u8,
        opt_len: i32,
        opt_data: *mut c_void,
    ) -> i32;

    #[psp(0xEA3C6108)]
    /// Cancel a matching target
    ///
    /// # Parameters
    ///
    /// - `matching_id`: The ID returned from ::sceNetAdhocMatchingCreate
    /// - `mac`: The MAC address to cancel
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceNetAdhocMatchingCancelTarget(
        matching_id: i32,
        mac: *mut u8,
    ) -> i32;

    #[psp(0x8F58BEDF)]
    /// Cancel a matching target (with optional data)
    ///
    /// # Parameters
    ///
    /// - `matching_id`: The ID returned from ::sceNetAdhocMatchingCreate
    /// - `mac`: The MAC address to cancel
    /// - `opt_len`: Optional data length
    /// - `opt_data`: Pointer to the optional data
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceNetAdhocMatchingCancelTargetWithOpt(
        matching_id: i32,
        mac: *mut u8,
        opt_len: i32,
        opt_data: *mut c_void,
    ) -> i32;

    #[psp(0xF79472D7)]
    /// Send data to a matching target
    ///
    /// # Parameters
    ///
    /// - `matching_id`: The ID returned from ::sceNetAdhocMatchingCreate
    /// - `mac`: The MAC address to send the data to
    /// - `data_len`: Length of the data
    /// - `data`: Pointer to the data
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceNetAdhocMatchingSendData(
        matching_id: i32,
        mac: *mut u8,
        data_len: i32,
        data: *mut c_void,
    ) -> i32;

    #[psp(0xEC19337D)]
    /// Abort a data send to a matching target
    ///
    /// # Parameters
    ///
    /// - `matching_id`: The ID returned from ::sceNetAdhocMatchingCreate
    /// - `mac`: The MAC address to send the data to
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceNetAdhocMatchingAbortSendData(
        matching_id: i32,
        mac: *mut u8,
    ) -> i32;

    #[psp(0xB58E61B7)]
    /// Set the optional hello message
    ///
    /// # Parameters
    ///
    /// - `matching_id`: The ID returned from ::sceNetAdhocMatchingCreate
    /// - `opt_len`: Length of the hello data
    /// - `opt_data`: Pointer to the hello data
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceNetAdhocMatchingSetHelloOpt(
        matching_id: i32,
        opt_len: i32,
        opt_data: *mut c_void,
    ) -> i32;

    #[psp(0xB5D96C2A)]
    /// Get the optional hello message
    ///
    /// # Parameters
    ///
    /// - `matching_id`: The ID returned from ::sceNetAdhocMatchingCreate
    /// - `opt_len`: Length of the hello data
    /// - `opt_data`: Pointer to the hello data
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceNetAdhocMatchingGetHelloOpt(
        matching_id: i32,
        opt_len: *mut i32,
        opt_data: *mut c_void,
    ) -> i32;

    #[psp(0xC58BCD9E)]
    /// Get a list of matching members
    ///
    /// # Parameters
    ///
    /// - `matching_id`: The ID returned from ::sceNetAdhocMatchingCreate
    /// - `length`: The length of the list.
    /// - `buf`: An allocated area of size length.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceNetAdhocMatchingGetMembers(
        matching_id: i32,
        length: *mut i32,
        buf: *mut c_void,
    ) -> i32;

    #[psp(0x40F8F435)]
    /// Get the maximum memory usage by the matching library
    ///
    /// # Return Value
    ///
    /// The memory usage on success, < 0 on error.
    pub fn sceNetAdhocMatchingGetPoolMaxAlloc() -> i32;

    #[psp(0x9C5CFB7D)]
    /// Get the status of the memory pool used by the matching library
    ///
    /// # Parameters
    ///
    /// - `poolstat`: A ::AdhocPoolStat.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceNetAdhocMatchingGetPoolStat(poolstat: *mut AdhocPoolStat)
        -> i32;

}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum ApctlState {
    Disconnected,
    Scanning,
    Joining,
    GettingIp,
    GotIp,
    EapAuth,
    KeyExchange,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum ApctlEvent {
    ConnectRequest,
    ScanRequest,
    ScanComplete,
    Established,
    GetIp,
    DisconnectRequest,
    Error,
    Info,
    EapAuth,
    KeyExchange,
    Reconnect,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum ApctlInfo {
    ProfileName,
    Bssid,
    Ssid,
    SsidLength,
    SecurityType,
    Strength,
    Channel,
    PowerSave,
    Ip,
    SubnetMask,
    Gateway,
    PrimaryDns,
    SecondaryDns,
    UseProxy,
    ProxyUrl,
    ProxyPort,
    EapType,
    StartBrowser,
    Wifisp,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum ApctlInfoSecurityType {
    None,
    Wep,
    Wpa,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union SceNetApctlInfo {
    pub name: [u8; 64usize],
    pub bssid: [u8; 6usize],
    pub ssid: [u8; 32usize],
    pub ssid_length: u32,
    pub security_type: u32,
    pub strength: u8,
    pub channel: u8,
    pub power_save: u8,
    pub ip: [u8; 16usize],
    pub sub_net_mask: [u8; 16usize],
    pub gateway: [u8; 16usize],
    pub primary_dns: [u8; 16usize],
    pub secondary_dns: [u8; 16usize],
    pub use_proxy: u32,
    pub proxy_url: [u8; 128usize],
    pub proxy_port: u16,
    pub eap_type: u32,
    pub start_browser: u32,
    pub wifisp: u32,
}

pub type SceNetApctlHandler = Option<
    unsafe extern "C" fn(oldState: i32, newState: i32, event: i32, error: i32, pArg: *mut c_void),
>;

psp_extern! {
    #![name = "sceNetApctl"]
    #![flags = 0x0009]
    #![version = (0x00, 0x00)]

    #[psp(0xE2F91F9B)]
    /// Init the apctl.
    ///
    /// # Parameters
    ///
    /// - `stack_size`: The stack size of the internal thread.
    ///
    /// # Parameters
    ///
    /// - `init_priority`: The priority of the internal thread.
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceNetApctlInit(
        stack_size: i32,
        init_priority: i32,
    ) -> i32;

    #[psp(0xB3EDD0EC)]
    /// Terminate the apctl.
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceNetApctlTerm() -> i32;

    #[psp(0x2BEFDF23)]
    /// Get the apctl information.
    ///
    /// # Parameters
    ///
    /// - `code`: One of the ApctlInfo.
    ///
    /// # Parameters
    ///
    /// - `pinfo`: Pointer to a ::SceNetApctlInfo.
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceNetApctlGetInfo(
        code: ApctlInfo,
        pinfo: *mut SceNetApctlInfo,
    ) -> i32;

    #[psp(0x8ABADD51)]
    /// Add an apctl event handler.
    ///
    /// # Parameters
    ///
    /// - `handler`: Pointer to the event handler function.
    ///
    /// # Parameters
    ///
    /// - `parg`: Value to be passed to the pArg parameter of the handler function.
    ///
    /// # Return Value
    ///
    /// A handler id or < 0 on error.
    pub fn sceNetApctlAddHandler(
        handler: SceNetApctlHandler,
        parg: *mut c_void,
    ) -> i32;

    #[psp(0x5963991B)]
    /// Delete an apctl event handler.
    ///
    /// # Parameters
    ///
    /// - `handler_id`: A handler as created returned from sceNetApctlAddHandler.
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceNetApctlDelHandler(handler_id: i32) -> i32;

    #[psp(0xCFB957C6)]
    /// Connect to an access point.
    ///
    /// # Parameters
    ///
    /// - `conn_index`: The index of the connection.
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceNetApctlConnect(conn_index: i32) -> i32;

    #[psp(0x24FE91A1)]
    /// Disconnect from an access point.
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceNetApctlDisconnect() -> i32;

    #[psp(0x5DEAC81B)]
    /// Get the state of the access point connection.
    ///
    /// # Parameters
    ///
    /// - `pstate`: Pointer to receive the current state (one of ApctlState).
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub fn sceNetApctlGetState(pstate: *mut ApctlState) -> i32;

}

#[allow(non_camel_case_types)]
pub type socklen_t = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr {
    /// Total length
    pub sa_len: u8,
    /// Address family
    pub sa_family: u8,
    /// Actually longer; address value
    pub sa_data: [u8; 14],
}

psp_extern! {
    #![name = "sceNetInet"]
    #![flags = 0x0009]
    #![version = (0x00, 0x00)]

    #[psp(0x17943399)]
    pub fn sceNetInetInit() -> i32;

    #[psp(0xA9ED66B9)]
    pub fn sceNetInetTerm() -> i32;

    #[psp(0xDB094E1B)]
    pub fn sceNetInetAccept(
        s: i32,
        addr: *mut sockaddr,
        addr_len: *mut socklen_t,
    ) -> i32;

    #[psp(0x1A33F9AE)]
    pub fn sceNetInetBind(
        s: i32,
        my_addr: *const sockaddr,
        addr_len: socklen_t,
    ) -> i32;

    #[psp(0x410B34AA)]
    /// Connect a socket
    ///
    /// # Parameters
    ///
    /// - `s`: The socket.
    /// - `serv_addr`: The address to connect to.
    /// - `addr_len`: The length of the address (in bytes).
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    ///
    /// # Notes
    /// The parameter `s` is the socket's file descriptor, i.e. the
    /// value returned by [`sceNetInetSocket()`](crate::sys::net::sceNetInetSocket).
    pub fn sceNetInetConnect(
        s: i32,
        serv_addr: *const sockaddr,
        addr_len: socklen_t,
    ) -> i32;

    #[psp(0x4A114C7C, i5)]
    pub fn sceNetInetGetsockopt(
        s: i32,
        level: i32,
        opt_name: i32,
        opt_val: *mut c_void,
        optl_en: *mut socklen_t,
    ) -> i32;

    #[psp(0xD10A1A7A)]
    pub fn sceNetInetListen(
        s: i32,
        backlog: i32,
    ) -> i32;

    #[psp(0xCDA85C99)]
    /// Receive a message
    ///
    /// # Parameters
    ///
    /// - `s`: The socket.
    /// - `buf`: The buffer to receive the message.
    /// - `len`: The length of the buffer.
    /// - `flags`: Flags.
    ///
    /// # Return Value
    ///
    /// The number of bytes received, < 0 on error.
    pub fn sceNetInetRecv(
        s: i32,
        buf: *mut c_void,
        len: usize,
        flags: i32,
    ) -> isize;

    #[psp(0xC91142E4, i6)]
    pub fn sceNetInetRecvfrom(
        s: i32,
        buf: *mut c_void,
        len: usize,
        flags: i32,
        from: *mut sockaddr,
        from_len: *mut socklen_t,
    ) -> isize;

    #[psp(0x7AA671BC)]
    pub fn sceNetInetSend(
        s: i32,
        buf: *const c_void,
        len: usize,
        flags: i32,
    ) -> isize;

    #[psp(0x05038FC7, i6)]
    pub fn sceNetInetSendto(
        s: i32,
        buf: *const c_void,
        len: usize,
        flags: i32,
        to: *const sockaddr,
        to_len: socklen_t,
    ) -> isize;

    #[psp(0x2FE71FE7, i5)]
    pub fn sceNetInetSetsockopt(
        s: i32,
        level: i32,
        opt_name: i32,
        opt_val: *const c_void,
        opt_len: socklen_t,
    ) -> i32;


    #[psp(0x4CFE4E56)]
    pub fn sceNetInetShutdown(
        s: i32,
        how: i32,
    ) -> i32;

    #[psp(0x8B7B220F)]
    /// Create a socket
    ///
    /// # Parameters
    ///
    /// - `domain`: The socket's domain (`2` IPv4).
    /// - `type_`: The socket's type (`1` = TCP, `2` = UDP).
    /// - `protocol`: The socket's protocol (`0` = default).
    ///
    /// # Return Value
    /// - the socket's file descriptor on success, `-1` on error.
    pub fn sceNetInetSocket(
        domain: i32,
        type_: i32,
        protocol: i32,
    ) -> i32;

    #[psp(0x8D7284EA)]
    pub fn sceNetInetClose(s: i32) -> i32;

    #[psp(0xFBABE411)]
    pub fn sceNetInetGetErrno() -> i32;

    #[psp(0x162E6FD5)]
    pub fn sceNetInetGetsockname(
        s: i32,
        addr: *mut sockaddr,
        addr_len: *mut socklen_t,
    ) -> i32;

    #[psp(0xE247B6D6)]
    pub fn sceNetInetGetpeername(
        s: i32,
        addr: *mut sockaddr,
        addr_len: *mut socklen_t,
    ) -> i32;
}

psp_extern! {
    #![name = "sceSsl"]
    #![flags = 0x0009]
    #![version = (0x00, 0x11)]

    #[psp(0x957ECBE2)]
    /// Init the ssl library.
    ///
    /// # Parameters
    ///
    /// - `unknown1`: Memory size? Pass 0x28000
    ///
    /// # Return Value
    ///
    /// 0 on success
    pub fn sceSslInit(unknown1: i32) -> i32;

    #[psp(0x191CDEFF)]
    /// Terminate the ssl library.
    ///
    /// # Return Value
    ///
    /// 0 on success
    pub fn sceSslEnd() -> i32;

    #[psp(0xB99EDE6A)]
    /// Get the maximum memory size used by ssl.
    ///
    /// # Parameters
    ///
    /// - `memory`: Pointer where the maximum memory used value will be stored.
    ///
    /// # Return Value
    ///
    /// 0 on success
    pub fn sceSslGetUsedMemoryMax(memory: *mut u32) -> i32;

    #[psp(0x0EB43B06)]
    /// Get the current memory size used by ssl.
    ///
    /// # Parameters
    ///
    /// - `memory`: Pointer where the current memory used value will be stored.
    ///
    /// # Return Value
    ///
    /// 0 on success
    pub fn sceSslGetUsedMemoryCurrent(memory: *mut u32)
        -> i32;

}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum HttpMethod {
    Get,
    Post,
    Head,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum HttpAuthType {
    Basic,
    Digest,
}

pub type HttpMallocFunction = Option<unsafe extern "C" fn(size: usize) -> *mut c_void>;
pub type HttpReallocFunction =
    Option<unsafe extern "C" fn(p: *mut c_void, size: usize) -> *mut c_void>;
pub type HttpFreeFunction = Option<unsafe extern "C" fn(p: *mut c_void)>;
pub type HttpPasswordCB = Option<
    unsafe extern "C" fn(
        request: i32,
        auth_type: HttpAuthType,
        realm: *const u8,
        username: *mut u8,
        password: *mut u8,
        need_entity: i32,
        entity_body: *mut *mut u8,
        entity_size: *mut usize,
        save: *mut i32,
    ) -> i32,
>;

psp_extern! {
    #![name = "sceHttp"]
    #![flags = 0x0009]
    #![version = (0x00, 0x11)]

    #[psp(0xAB1ABE07)]
    /// Init the http library.
    ///
    /// # Parameters
    ///
    /// - `unknown1`: Memory pool size? Pass 20000
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceHttpInit(unknown1: u32) -> i32;

    #[psp(0xD1C8945E)]
    /// Terminate the http library.
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceHttpEnd() -> i32;

    #[psp(0x9B1F1F36)]
    /// Create a http template.
    ///
    /// # Parameters
    ///
    /// - `agent`: User agent
    /// - `unknown1`: Pass 1
    /// - `unknown2`: Pass 0
    /// # Return Value
    ///
    /// A template ID on success, < 0 on error.
    pub fn sceHttpCreateTemplate(
        agent: *mut u8,
        unknown1: i32,
        unknown2: i32,
    ) -> i32;

    #[psp(0xFCF8C055)]
    /// Delete a http template.
    ///
    /// # Parameters
    ///
    /// - `templateid`: ID of the template created by sceHttpCreateTemplate
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceHttpDeleteTemplate(templateid: i32) -> i32;

    #[psp(0x8EEFD953)]
    /// Create a http connection.
    ///
    /// # Parameters
    ///
    /// - `templateid`: ID of the template created by sceHttpCreateTemplate
    /// - `host`: Host to connect to
    /// - `unknown1`: Pass "http"]
    /// # Parameters
    ///
    /// - `port`: Port to connect on
    /// - `unknown2`: Pass 0
    /// # Return Value
    ///
    /// A connection ID on success, < 0 on error.
    pub fn sceHttpCreateConnection(
        templateid: i32,
        host: *mut u8,
        unknown1: *mut u8,
        port: u16,
        unknown2: i32,
    ) -> i32;

    #[psp(0xCDF8ECB9)]
    /// Create a http connection to a url.
    ///
    /// # Parameters
    ///
    /// - `templateid`: ID of the template created by sceHttpCreateTemplate
    /// - `url`: url to connect to
    /// - `unknown1`: Pass 0
    /// # Return Value
    ///
    /// A connection ID on success, < 0 on error.
    pub fn sceHttpCreateConnectionWithURL(
        templateid: i32,
        url: *const u8,
        unknown1: i32,
    ) -> i32;

    #[psp(0x5152773B)]
    /// Delete a http connection.
    ///
    /// # Parameters
    ///
    /// - `connection_id`: ID of the connection created by sceHttpCreateConnection or sceHttpCreateConnectionWithURL
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceHttpDeleteConnection(connection_id: i32) -> i32;

    #[psp(0x47347B50)]
    /// Create a http request.
    ///
    /// # Parameters
    ///
    /// - `connection_id`: ID of the connection created by sceHttpCreateConnection or sceHttpCreateConnectionWithURL
    /// - `method`: One of ::HttpMethod
    /// - `path`: Path to access
    /// - `content_length`: Length of the content (POST method only)
    /// # Return Value
    ///
    /// A request ID on success, < 0 on error.
    pub fn sceHttpCreateRequest(
        connection_id: i32,
        method: HttpMethod,
        path: *mut u8,
        content_length: u64,
    ) -> i32;

    #[psp(0xB509B09E)]
    /// Create a http request with url.
    ///
    /// # Parameters
    ///
    /// - `connection_id`: ID of the connection created by sceHttpCreateConnection or sceHttpCreateConnectionWithURL
    /// - `method`: One of ::HttpMethod
    /// - `url`: url to access
    /// - `content_length`: Length of the content (POST method only)
    /// # Return Value
    ///
    /// A request ID on success, < 0 on error.
    pub fn sceHttpCreateRequestWithURL(
        connection_id: i32,
        method: HttpMethod,
        url: *mut u8,
        content_length: u64,
    ) -> i32;

    #[psp(0xA5512E01)]
    /// Delete a http request.
    ///
    /// # Parameters
    ///
    /// - `request_id`: ID of the request created by sceHttpCreateRequest or sceHttpCreateRequestWithURL
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceHttpDeleteRequest(request_id: i32) -> i32;

    #[psp(0xBB70706F)]
    /// Send a http request.
    ///
    /// # Parameters
    ///
    /// - `request_id`: ID of the request created by sceHttpCreateRequest or sceHttpCreateRequestWithURL
    /// - `data`: For POST methods specify a pointer to the post data, otherwise pass NULL
    /// - `data_size`: For POST methods specify the size of the post data, otherwise pass 0
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceHttpSendRequest(
        request_id: i32,
        data: *mut c_void,
        data_size: u32,
    ) -> i32;

    #[psp(0xC10B6BD9)]
    /// Abort a http request.
    ///
    /// # Parameters
    ///
    /// - `request_id`: ID of the request created by sceHttpCreateRequest or sceHttpCreateRequestWithURL
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceHttpAbortRequest(request_id: i32) -> i32;

    #[psp(0xEDEEB999)]
    /// Read a http request response.
    ///
    /// # Parameters
    ///
    /// - `request_id`: ID of the request created by sceHttpCreateRequest or sceHttpCreateRequestWithURL
    /// - `data`: Buffer for the response data to be stored
    /// - `data_size`: Size of the buffer
    /// # Return Value
    ///
    /// The size read into the data buffer, 0 if there is no more data, < 0 on error.
    pub fn sceHttpReadData(
        request_id: i32,
        data: *mut c_void,
        data_size: u32,
    ) -> i32;

    #[psp(0x0282A3BD)]
    /// Get http request response length.
    ///
    /// # Parameters
    ///
    /// - `request_id`: ID of the request created by sceHttpCreateRequest or sceHttpCreateRequestWithURL
    /// - `content_length`: The size of the content
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceHttpGetContentLength(
        request_id: i32,
        content_length: *mut u64,
    ) -> i32;

    #[psp(0x4CC7D78F)]
    /// Get http request status code.
    ///
    /// # Parameters
    ///
    /// - `request_id`: ID of the request created by sceHttpCreateRequest or sceHttpCreateRequestWithURL
    /// - `status_code`: The status code from the host (200 is ok, 404 is not found etc)
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceHttpGetStatusCode(
        request_id: i32,
        status_code: *mut i32,
    ) -> i32;

    #[psp(0x47940436)]
    /// Set resolver timeout
    ///
    /// # Parameters
    ///
    /// - `id`: ID of the template or connection
    /// - `timeout`: Timeout value in microseconds
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceHttpSetResolveTimeOut(
        id: i32,
        timeout: u32,
    ) -> i32;

    #[psp(0x03D9526F)]
    /// Set resolver retry
    ///
    /// # Parameters
    ///
    /// - `id`: ID of the template or connection
    /// - `count`: Number of retries
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceHttpSetResolveRetry(
        id: i32,
        count: i32,
    ) -> i32;

    #[psp(0x8ACD1F73)]
    /// Set connect timeout
    ///
    /// # Parameters
    ///
    /// - `id`: ID of the template, connection or request
    /// - `timeout`: Timeout value in microseconds
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceHttpSetConnectTimeOut(
        id: i32,
        timeout: u32,
    ) -> i32;

    #[psp(0x9988172D)]
    /// Set send timeout
    ///
    /// # Parameters
    ///
    /// - `id`: ID of the template, connection or request
    /// - `timeout`: Timeout value in microseconds
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceHttpSetSendTimeOut(
        id: i32,
        timeout: u32,
    ) -> i32;

    #[psp(0x1F0FC3E3)]
    /// Set receive timeout
    ///
    /// # Parameters
    ///
    /// - `id`: ID of the template or connection
    /// - `timeout`: Timeout value in microseconds
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceHttpSetRecvTimeOut(
        id: i32,
        timeout: u32,
    ) -> i32;

    #[psp(0x78A0D3EC)]
    /// Enable keep alive
    ///
    /// # Parameters
    ///
    /// - `id`: ID of the template or connection
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceHttpEnableKeepAlive(id: i32) -> i32;

    #[psp(0xC7EF2559)]
    /// Disable keep alive
    ///
    /// # Parameters
    ///
    /// - `id`: ID of the template or connection
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceHttpDisableKeepAlive(id: i32) -> i32;

    #[psp(0x0809C831)]
    /// Enable redirect
    ///
    /// # Parameters
    ///
    /// - `id`: ID of the template or connection
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceHttpEnableRedirect(id: i32) -> i32;

    #[psp(0x1A0EBB69)]
    /// Disable redirect
    ///
    /// # Parameters
    ///
    /// - `id`: ID of the template or connection
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceHttpDisableRedirect(id: i32) -> i32;

    #[psp(0x0DAFA58F)]
    /// Enable cookie
    ///
    /// # Parameters
    ///
    /// - `id`: ID of the template or connection
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceHttpEnableCookie(id: i32) -> i32;

    #[psp(0x0B12ABFB)]
    /// Disable cookie
    ///
    /// # Parameters
    ///
    /// - `id`: ID of the template or connection
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceHttpDisableCookie(id: i32) -> i32;

    #[psp(0x76D1363B)]
    /// Save cookie
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceHttpSaveSystemCookie() -> i32;

    #[psp(0xF1657B22)]
    /// Load cookie
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceHttpLoadSystemCookie() -> i32;

    #[psp(0x3EABA285)]
    /// Add content header
    ///
    /// # Parameters
    ///
    /// - `id`: ID of the template, connection or request
    /// - `name`: Name of the content
    /// - `value`: Value of the content
    /// - `unknown1`: Pass 0
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceHttpAddExtraHeader(
        id: i32,
        name: *mut u8,
        value: *mut u8,
        unknown1: i32,
    ) -> i32;

    #[psp(0x15540184)]
    /// Delete content header
    ///
    /// # Parameters
    ///
    /// - `id`: ID of the template, connection or request
    /// - `name`: Name of the content
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceHttpDeleteHeader(
        id: i32,
        name: *const u8,
    ) -> i32;

    #[psp(0xE4D21302)]
    /// Init the https library.
    ///
    /// # Parameters
    ///
    /// - `unknown1`: Pass 0
    /// - `unknown2`: Pass 0
    /// - `unknown3`: Pass 0
    /// - `unknown4`: Pass 0
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceHttpsInit(
        unknown1: i32,
        unknown2: i32,
        unknown3: i32,
        unknown4: i32,
    ) -> i32;

    #[psp(0xF9D8EB63)]
    /// Terminate the https library
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceHttpsEnd() -> i32;

    #[psp(0x87797BDD)]
    /// Load default certificate
    ///
    /// # Parameters
    ///
    /// - `unknown1`: Pass 0
    /// - `unknown2`: Pass 0
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub fn sceHttpsLoadDefaultCert(
        unknown1: i32,
        unknown2: i32,
    ) -> i32;

    #[psp(0xAE948FEE)]
    pub fn sceHttpDisableAuth(id: i32) -> i32;

    #[psp(0xCCBD167A)]
    pub fn sceHttpDisableCache(id: i32) -> i32;

    #[psp(0x9FC5F10D)]
    pub fn sceHttpEnableAuth(id: i32) -> i32;

    #[psp(0x59E6D16F)]
    pub fn sceHttpEnableCache(id: i32) -> i32;

    #[psp(0x78B54C09)]
    pub fn sceHttpEndCache() -> i32;

    #[psp(0xDB266CCF)]
    pub fn sceHttpGetAllHeader(
        request: i32,
        header: *mut *mut u8,
        header_size: *mut u32,
    ) -> i32;

    #[psp(0xD081EC8F)]
    pub fn sceHttpGetNetworkErrno(
        request: i32,
        err_num: *mut i32,
    ) -> i32;

    #[psp(0xD70D4847)]
    pub fn sceHttpGetProxy(
        id: i32,
        activate_flag: *mut i32,
        mode: *mut i32,
        proxy_host: *mut u8,
        len: usize,
        proxy_port: *mut u16,
    ) -> i32;

    #[psp(0xA6800C34)]
    pub fn sceHttpInitCache(max_size: usize) -> i32;

    #[psp(0x2A6C3296)]
    pub fn sceHttpSetAuthInfoCB(
        id: i32,
        cbfunc: HttpPasswordCB,
    ) -> i32;

    #[psp(0xF0F46C62)]
    pub fn sceHttpSetProxy(
        id: i32,
        activate_flag: i32,
        mode: i32,
        new_proxy_host: *const u8,
        new_proxy_port: u16,
    ) -> i32;

    #[psp(0xC98CBBA7)]
    pub fn sceHttpSetResHeaderMaxSize(
        id: i32,
        header_size: u32,
    ) -> i32;

    #[psp(0xF49934F6)]
    pub fn sceHttpSetMallocFunction(
        malloc_func: HttpMallocFunction,
        free_func: HttpFreeFunction,
        realloc_func: HttpReallocFunction,
    ) -> i32;

}

#[repr(C)]
pub struct in_addr(pub u32);

psp_extern! {
    #![name = "sceNetResolver"]
    #![flags = 0x0009]
    #![version = (0x00, 0x00)]

    #[psp(0xF3370E61)]
    /// Inititalise the resolver library
    ///
    /// # Return Value
    ///
    /// 0 on sucess, < 0 on error.
    pub fn sceNetResolverInit() -> i32;

    #[psp(0x244172AF)]
    /// Create a resolver object
    ///
    /// # Parameters
    ///
    /// - `rid`: Pointer to receive the resolver id
    /// - `buf`: Temporary buffer
    /// - `buf_length`: Length of the temporary buffer
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceNetResolverCreate(
        rid: *mut i32,
        buf: *mut c_void,
        buf_length: u32,
    ) -> i32;

    #[psp(0x94523E09)]
    /// Delete a resolver
    ///
    /// # Parameters
    ///
    /// - `rid`: The resolver to delete
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceNetResolverDelete(rid: i32) -> i32;

    #[psp(0x224C5F44)]
    /// Begin a name to address lookup
    ///
    /// # Parameters
    ///
    /// - `rid`: Resolver id
    /// - `hostname`: Name to resolve
    /// - `addr`: Pointer to in_addr structure to receive the address
    /// - `timeout`: Number of seconds before timeout
    /// - `retry`: Number of retires
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceNetResolverStartNtoA(
        rid: i32,
        hostname: *const u8,
        addr: *mut in_addr,
        timeout: u32,
        retry: i32,
    ) -> i32;

    #[psp(0x629E2FB7)]
    /// Begin a address to name lookup
    ///
    /// # Parameters
    ///
    /// - `rid`: Resolver id
    /// - `addr`: Pointer to the address to resolve
    /// - `hostname`: Buffer to receive the name
    /// - `hostname_len`: Length of the buffer
    /// - `timeout`: Number of seconds before timeout
    /// - `retry`: Number of retries
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceNetResolverStartAtoN(
        rid: i32,
        addr: *const in_addr,
        hostname: *mut u8,
        hostname_len: u32,
        timeout: u32,
        retry: i32,
    ) -> i32;

    #[psp(0x808F6063)]
    /// Stop a resolver operation
    ///
    /// # Parameters
    ///
    /// - `rid`: Resolver id
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceNetResolverStop(rid: i32) -> i32;

    #[psp(0x6138194A)]
    /// Terminate the resolver library
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceNetResolverTerm() -> i32;

}
