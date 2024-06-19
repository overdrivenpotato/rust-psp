use crate::eabi::{i6, i_ii_i_ri, i_ii_i_rii};
use crate::sys::ScePspDateTime;
use crate::sys::SceUid;
use core::ffi::c_void;

/// Describes a single directory entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceIoDirent {
    /// File status.
    pub d_stat: SceIoStat,
    /// File name.
    pub d_name: [u8; 256usize],
    /// Device-specific data.
    pub d_private: *mut c_void,
    pub dummy: i32,
}

/// Structure to hold the status information about a file
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceIoStat {
    pub st_mode: IoStatMode,
    pub st_attr: IoStatAttr,
    /// Size of the file in bytes.
    pub st_size: i64,
    /// Creation time.
    pub st_ctime: ScePspDateTime,
    /// Access time.
    pub st_atime: ScePspDateTime,
    /// Modification time.
    pub st_mtime: ScePspDateTime,
    /// Device-specific data.
    pub st_private: [u32; 6usize],
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Copy, Clone)]
    pub struct IoStatMode: i32 {
        /// Symbolic Link
        const IFLNK = 0x4000;
        /// Directory
        const IFDIR = 0x1000;
        /// Regular file
        const IFREG = 0x2000;
        /// Set UID
        const ISUID = 0x0800;
        /// Set GID
        const ISGID = 0x0400;
        /// Sticky
        const ISVTX = 0x0200;
        /// Read user permission
        const IRUSR = 0x0100;
        /// Write user permission
        const IWUSR = 0x0080;
        /// Execute user permission
        const IXUSR = 0x0040;
        /// Read group permission
        const IRGRP = 0x0020;
        /// Write group permission
        const IWGRP = 0x0010;
        /// Execute group permission
        const IXGRP = 0x0008;
        /// Read others permission
        const IROTH = 0x0004;
        /// Write others permission
        const IWOTH = 0x0002;
        /// Execute others permission
        const IXOTH = 0x0001;
    }
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Copy, Clone)]
    pub struct IoStatAttr: u32 {
        /// Symlink
        const IFLNK = 0x0008;
        /// Directory
        const IFDIR = 0x0010;
        /// Regular file
        const IFREG = 0x0020;
        /// Hidden read permisson
        const IROTH = 0x0004;
        /// Hidden write permission
        const IWOTH = 0x0002;
        /// Hidden execution permission
        const IXOTH = 0x0001;
    }
}

#[repr(u32)]
/// Permission value for the sceIoAssign function
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum IoAssignPerms {
    RdWr = 0,
    RdOnly = 1,
}

#[repr(u32)]
pub enum IoWhence {
    Set = 0,
    Cur = 1,
    End = 2,
}

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct IoOpenFlags: i32 {
        const RD_ONLY = 0x0001;
        const WR_ONLY = 0x0002;
        const RD_WR = 0x0003;
        const NBLOCK = 0x0004;
        const DIR = 0x0008;
        const APPEND = 0x0100;
        const CREAT = 0x0200;
        const TRUNC = 0x0400;
        const EXCL = 0x0800;
        const NO_WAIT = 0x8000;
    }
}

/// Octal unix permissions
pub type IoPermissions = i32;

psp_extern! {
    #![name = "IoFileMgrForUser"]
    #![flags = 0x4001]
    #![version = (0, 0)]

    #[psp(0x109F50BC)]
    /// Open or create a file for reading or writing
    ///
    /// # Parameters
    ///
    /// - `file`: Pointer to a string holding the name of the file to open
    /// - `flags`: Libc styled flags that are or'ed together
    /// - `permissions`: Octal unix permissions.
    ///
    /// # Return value
    ///
    /// A non-negative integer is a valid fd, anything else an error
    pub fn sceIoOpen(file: *const u8, flags: IoOpenFlags, permissions: IoPermissions) -> SceUid;

    #[psp(0x89AA9906)]
    /// Open or create a file for reading or writing (asynchronous)
    ///
    /// # Parameters
    ///
    /// - `file`: Pointer to a string holding the name of the file to open
    /// - `flags`: Libc styled flags that are or'ed together
    /// - `permissions`: Octal unix permissions.
    ///
    /// # Return value
    ///
    /// A non-negative integer is a valid fd, anything else an error
    pub fn sceIoOpenAsync(
        file: *const u8,
        flags: IoOpenFlags,
        permissions: IoPermissions
    ) -> SceUid;

    #[psp(0x810C4BC3)]
    /// Delete a descriptor
    ///
    /// # Parameters
    ///
    /// - `fd`: File descriptor to close
    ///
    /// # Return value
    ///
    /// < 0 on error
    pub fn sceIoClose(fd: SceUid) -> i32;

    #[psp(0xFF5940B6)]
    /// Delete a descriptor (asynchronous)
    ///
    /// # Parameters
    ///
    /// - `fd`: File descriptor to close
    ///
    /// # Return value
    ///
    /// < 0 on error
    pub fn sceIoCloseAsync(fd: SceUid) -> i32;

    #[psp(0x6A638D83)]
    /// Read input
    ///
    /// # Parameters
    ///
    /// - `fd`: Opened file descriptor to read from
    /// - `data`: Pointer to the buffer where the read data will be placed
    /// - `size`: Size of the read in bytes
    ///
    /// # Return value
    ///
    /// The number of bytes read
    pub fn sceIoRead(fd: SceUid, data: *mut c_void, size: u32)
     -> i32;

    #[psp(0xA0B5A7C2)]
    /// Read input (asynchronous)
    ///
    /// # Parameters
    ///
    /// - `fd`: Opened file descriptor to read from
    /// - `data`: Pointer to the buffer where the read data will be placed
    /// - `size`: Size of the read in bytes
    ///
    /// # Return value
    ///
    /// < 0 on error.
    pub fn sceIoReadAsync(fd: SceUid, data: *mut c_void, size: u32)
     -> i32;


    #[psp(0x42EC03AC)]
    /// Write output
    ///
    /// # Parameters
    ///
    /// - `fd`: Opened file descriptor to write to
    /// - `data`: Pointer to the data to write
    /// - `size`: Size of data to write
    ///
    /// # Return value
    ///
    /// The number of bytes written
    pub fn sceIoWrite(fd: SceUid, data: *const c_void, size: usize) -> i32;

    #[psp(0x0FACAB19)]
    /// Write output (asynchronous)
    ///
    /// # Parameters
    ///
    /// - `fd`: Opened file descriptor to write to
    /// - `data`: Pointer to the data to write
    /// - `size`: Size of data to write
    ///
    /// # Return value
    ///
    /// < 0 on error.
    pub fn sceIoWriteAsync(fd: SceUid, data: *const c_void, size: u32)
     -> i32;

    #[psp(0x27EB27B8, i_ii_i_rii)]
    /// Reposition read/write file descriptor offset
    ///
    /// # Parameters
    ///
    /// - `fd`: Opened file descriptor with which to seek
    /// - `offset`: Relative offset from the start position given by whence
    /// - `whence`: Set to `IoWhence::Set` to seek from the start of the file,
    ///   `IoWhence::Cur` seek from the current position and `IoWhence::End` to seek
    ///   from the end.
    ///
    /// # Return value
    ///
    /// The position in the file after the seek.
    pub fn sceIoLseek(fd: SceUid, offset: i64, whence: IoWhence) -> i64;

    #[psp(0x71B19E77, i_ii_i_ri)]
    /// Reposition read/write file descriptor offset (asynchronous)
    ///
    /// # Parameters
    ///
    /// - `fd`: Opened file descriptor with which to seek
    /// - `offset`: Relative offset from the start position given by whence
    /// - `whence`: Set to `IoWhence::Set` to seek from the start of the file,
    ///   `IoWhence::Cur` seek from the current position and IoWhence::End to seek
    ///   from the end.
    ///
    /// # Return value
    ///
    /// < 0 on error. Actual value should be passed returned by the ::sceIoWaitAsync call.
    pub fn sceIoLseekAsync(fd: SceUid, offset: i64, whence: IoWhence) -> i32;

    #[psp(0x68963324)]
    /// Reposition read/write file descriptor offset (32bit mode)
    ///
    /// # Parameters
    ///
    /// - `fd`: Opened file descriptor with which to seek
    /// - `offset`: Relative offset from the start position given by whence
    /// - `whence`: Set to `IoWhence::Set` to seek from the start of the file,
    ///   `IoWhence::Cur` seek from the current position and IoWhence::End to seek
    ///   from the end.
    ///
    /// # Return value
    ///
    /// The position in the file after the seek.
    pub fn sceIoLseek32(fd: SceUid, offset: i32, whence: IoWhence) -> i32;

    #[psp(0x1B385D8F)]
    /// Reposition read/write file descriptor offset (32bit mode, asynchronous)
    ///
    /// # Parameters
    ///
    /// - `fd`: Opened file descriptor with which to seek
    /// - `offset`: Relative offset from the start position given by whence
    /// - `whence`: Set to IoWhence::Set to seek from the start of the file, IoWhence::Cur
    ///   seek from the current position and IoWhence::End to seek from the end.
    ///
    /// # Return value
    ///
    /// < 0 on error.
    pub fn sceIoLseek32Async(fd: SceUid, offset: i32, whence: IoWhence) -> i32;

    #[psp(0xF27A9C51)]
    /// Remove directory entry
    ///
    /// # Parameters
    ///
    /// - `file`: Path to the file to remove
    ///
    /// # Return value
    ///
    /// < 0 on error
    pub fn sceIoRemove(file: *const u8) -> i32;

    #[psp(0x06A70004)]
    /// Make a directory file
    ///
    /// # Parameters
    ///
    /// `dir`
    /// - `mode`: Access mode.
    ///
    /// # Return value
    ///
    /// Returns the value 0 if its succesful otherwise -1
    pub fn sceIoMkdir(dir: *const u8, mode: IoPermissions) -> i32;

    #[psp(0x1117C65F)]
    /// Remove a directory file
    /// # Parameters
    ///
    /// - `path`: Removes a directory file pointed by the string path
    ///
    /// # Return value
    ///
    /// Returns the value 0 if its succesful otherwise -1
    pub fn sceIoRmdir(path: *const u8) -> i32;

    #[psp(0x55F4717D)]
    /// Change the current directory.
    /// # Parameters
    ///
    /// - `path`: The path to change to.
    ///
    /// # Return value
    ///
    /// < 0 on error.
    pub fn sceIoChdir(path: *const u8) -> i32;

    #[psp(0x779103A0)]
    /// Change the name of a file
    ///
    /// # Parameters
    ///
    /// - `oldname`: The old filename
    /// - `newname`: The new filename
    ///
    /// # Return value
    ///
    /// < 0 on error.
    pub fn sceIoRename(oldname: *const u8, newname: *const u8) -> i32;

    #[psp(0xB29DDF9C)]
    /// Open a directory
    ///
    /// # Parameters
    ///
    /// - `dirname`: The directory to open for reading.
    ///
    /// # Return value
    ///
    /// If >= 0 then a valid file descriptor, otherwise a Sony error code.
    pub fn sceIoDopen(dirname: *const u8) -> SceUid;

    #[psp(0xE3EB004C)]
    /// Reads an entry from an opened file descriptor.
    ///
    /// # Parameters
    ///
    /// - `fd`: Already opened file descriptor (using sceIoDopen)
    /// - `dir`: Pointer to an `SceIoDirent` structure to hold the file information
    ///
    /// # Return value
    ///
    /// Read status
    /// -   0 - No more directory entries left
    /// - > 0 - More directory entired to go
    /// - < 0 - Error
    pub fn sceIoDread(fd: SceUid, dir: *mut SceIoDirent) -> i32;

    #[psp(0xEB092469)]
    /// Close an opened directory file descriptor
    ///
    /// # Parameters
    ///
    /// - `fd`: Already opened file descriptor (using sceIoDopen)
    ///
    /// # Return value
    ///
    /// < 0 on error
    pub fn sceIoDclose(fd: SceUid) -> i32;

    #[psp(0x54F5FB11, i6)]
    /// Send a devctl command to a device.
    ///
    ///  # Parameters
    ///
    /// - `dev`: String for the device to send the devctl to (e.g. "ms0:")
    /// - `cmd`: The command to send to the device
    /// - `indata`: A data block to send to the device, if NULL sends no data
    /// - `inlen`: Length of indata, if 0 sends no data
    /// - `outdata`: A data block to receive the result of a command, if NULL receives no data
    /// - `outlen`: Length of outdata, if 0 receives no data
    ///
    /// # Return value
    ///
    /// 0 on success, < 0 on error
    pub fn sceIoDevctl(
        dev: *const u8,
        cmd: u32,
        indata: *mut c_void,
        inlen: i32,
        outdata: *mut c_void,
        outlen: i32
    ) -> i32;

    #[psp(0xB2A628C1,i6)]
    /// Assigns one IO device to another (I guess)
    ///
    /// # Parameters
    ///
    /// - `dev1`: The device name to assign.
    /// - `dev2`: The block device to assign from.
    /// - `dev3`: The filesystem device to mape the block device to dev1
    /// - `mode`: Read/Write mode. One of `IoAssignPerms`.
    /// - `unk1`: Unknown, set to NULL.
    /// - `unk2`: Unknown, set to 0.
    ///
    /// # Return value
    ///
    /// < 0 on error.
    pub fn sceIoAssign(
        dev1: *const u8,
        dev2: *const u8,
        dev3: *const u8,
        mode: IoAssignPerms,
        unk1: *mut c_void,
        unk2: i32
    ) -> i32;

    #[psp(0x6D08A871)]
    /// Unassign an IO device.
    ///
    /// # Parameters
    ///
    /// - `dev`: The device to unassign.
    ///
    /// # Return value
    ///
    /// < 0 on error
    pub fn sceIoUnassign(dev: *const u8) -> i32;

    #[psp(0xACE946E8)]
    /// Get the status of a file.
    ///
    /// # Parameters
    ///
    /// - `file`: The path to the file.
    /// - `stat`: A pointer to an `SceIoStat` structure.
    ///
    /// # Return value
    ///
    /// < 0 on error.
    pub fn sceIoGetstat(file: *const u8, stat: *mut SceIoStat) -> i32;

    #[psp(0xB8A740F4)]
    /// Change the status of a file.
    ///
    /// # Parameters
    ///
    /// - `file`: The path to the file.
    /// - `stat`: A pointer to an `SceIoStat` structure.
    /// - `bits`: Bitmask defining which bits to change.
    ///
    /// # Return value
    ///
    /// < 0 on error.
    pub fn sceIoChstat(file: *const u8, stat: *mut SceIoStat, bits: i32) -> i32;

    #[psp(0x63632449, i6)]
    /// Perform an ioctl on a device.
    ///
    /// # Parameters
    ///
    /// - `fd`: Opened file descriptor to ioctl to
    /// - `cmd`: The command to send to the device
    /// - `indata`: A data block to send to the device, if NULL sends no data
    /// - `inlen`: Length of indata, if 0 sends no data
    /// - `outdata`: A data block to receive the result of a command, if NULL receives no data
    /// - `outlen`: Length of outdata, if 0 receives no data
    ///
    /// # Return value
    ///
    /// 0 on success, < 0 on error
    pub fn sceIoIoctl(
        fd: SceUid,
        cmd: u32,
        indata: *mut c_void,
        inlen: i32,
        outdata: *mut c_void,
        outlen: i32
    ) -> i32;

    #[psp(0xE95A012B, i6)]
    /// Perform an ioctl on a device. (asynchronous)
    ///
    /// # Parameters
    ///
    /// - `fd`: Opened file descriptor to ioctl to
    /// - `cmd`: The command to send to the device
    /// - `indata`: A data block to send to the device, if NULL sends no data
    /// - `inlen`: Length of indata, if 0 sends no data
    /// - `outdata`: A data block to receive the result of a command, if NULL receives no data
    /// - `outlen`: Length of outdata, if 0 receives no data
    ///
    /// # Return value
    ///
    /// 0 on success, < 0 on error
    pub fn sceIoIoctlAsync(
        fd: SceUid,
        cmd: u32,
        indata: *mut c_void,
        inlen: i32,
        outdata: *mut c_void,
        outlen: i32
    ) -> i32;

    #[psp(0xAB96437F)]
    /// Synchronise the file data on the device.
    ///
    /// # Parameters
    ///
    /// - `device`: The device to synchronise (e.g. msfat0:)
    /// - `unk`: Unknown
    ///
    /// # Return value
    ///
    /// ???
    pub fn sceIoSync(device: *const u8, unk: u32) -> i32;

    #[psp(0xE23EEC33)]
    /// Wait for asyncronous completion.
    ///
    /// # Parameters
    ///
    /// - `fd`: The file descriptor which is current performing an asynchronous action.
    /// - `res`: The result of the async action.
    ///
    /// # Return value
    ///
    /// < 0 on error.
    pub fn sceIoWaitAsync(fd: SceUid, res: *mut i64) -> i32;

    #[psp(0x35DBD746)]
    /// Wait for asyncronous completion (with callbacks).
    ///
    /// # Parameters
    ///
    /// - `fd`: The file descriptor which is current performing an asynchronous action.
    /// - `res`: The result of the async action.
    ///
    /// # Return value
    ///
    /// < 0 on error.
    pub fn sceIoWaitAsyncCB(fd: SceUid, res: *mut i64) -> i32;

    #[psp(0x3251EA56)]
    /// Poll for asyncronous completion.
    ///
    /// # Parameters
    ///
    /// - `fd`: The file descriptor which is current performing an asynchronous action.
    /// - `res`: The result of the async action.
    ///
    /// # Return value
    ///
    /// < 0 on error.
    pub fn sceIoPollAsync(fd: SceUid, res: *mut i64) -> i32;

    #[psp(0xCB05F8D6)]
    /// Get the asyncronous completion status.
    ///
    /// # Parameters
    ///
    /// - `fd`: The file descriptor which is current performing an asynchronous action.
    /// - `poll`: If 0 then waits for the status, otherwise it polls the fd.
    /// - `res`: The result of the async action.
    ///
    /// # Return value
    ///
    /// < 0 on error.
    pub fn sceIoGetAsyncStat(fd: SceUid, poll: i32, res: *mut i64)
     -> i32;

    #[psp(0xE8BC6571)]
    /// Cancel an asynchronous operation on a file descriptor.
    ///
    /// # Parameters
    ///
    /// - `fd`: The file descriptor to perform cancel on.
    ///
    /// # Return value
    ///
    /// < 0 on error.
    pub fn sceIoCancel(fd: SceUid) -> i32;

    #[psp(0x08BD7374)]
    /// Get the device type of the currently opened file descriptor.
    ///
    /// # Parameters
    ///
    /// - `fd`: The opened file descriptor.
    ///
    /// # Return value
    ///
    /// < 0 on error. Otherwise the device type?
    pub fn sceIoGetDevType(fd: SceUid) -> i32;

    #[psp(0xB293727F)]
    /// Change the priority of the asynchronous thread.
    ///
    /// # Parameters
    ///
    /// - `fd`: The opened fd on which the priority should be changed.
    /// - `pri`: The priority of the thread.
    ///
    /// # Return value
    ///
    /// < 0 on error.
    pub fn sceIoChangeAsyncPriority(fd: SceUid, pri: i32) -> i32;

    #[psp(0xA12A0514)]
    /// Sets a callback for the asynchronous action.
    ///
    /// # Parameters
    ///
    /// - `fd`: The filedescriptor currently performing an asynchronous action.
    /// - `cb`: The UID of the callback created with `sceKernelCreateCallback`
    /// - `argp`: Pointer to an argument to pass to the callback.
    ///
    /// # Return value
    ///
    /// < 0 on error.
    pub fn sceIoSetAsyncCallback(
        fd: SceUid,
        cb: SceUid,
        argp: *mut c_void
    ) -> i32;
}
