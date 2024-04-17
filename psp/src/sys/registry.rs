use crate::eabi::i5;
use core::ffi::c_void;

pub const SYSTEM_REGISTRY: [u8; 7] = *b"/system";
pub const REG_KEYNAME_SIZE: u32 = 27;

/// Typedef for a registry handle.
#[repr(transparent)]
pub struct RegistryHandle(u32);

/// Struct used to open a registry.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RegistryKey {
    pub key_type: KeyType,
    /// Seemingly never used, set to `SYSTEM_REGISTRY`.
    pub name: [u8; 256usize],
    /// Length of the name.
    pub name_len: u32,
    /// Unknown, set to 1.
    pub unk2: u32,
    /// Unknown, set to 1.
    pub unk3: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum KeyType {
    /// Key is a directory
    Directory = 1,
    /// Key is an integer (4 bytes)
    Integer = 2,
    /// Key is a string
    String = 3,
    /// Key is a binary string
    Bytes = 4,
}

psp_extern! {
    #![name = "sceReg"]
    #![flags = 0x4001]
    #![version = (0x00, 0x00)]

    #[psp(0x92E41280)]
    /// Open the registry
    ///
    /// # Parameters
    ///
    /// - `reg`: A filled in `Key` structure
    /// - `mode`: Open mode (set to 1)
    /// - `handle`: Pointer to a `Handle` to receive the registry handle
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceRegOpenRegistry(
        reg: *mut RegistryKey,
        mode: i32,
        handle: *mut RegistryHandle,
    ) -> i32;

    #[psp(0x39461B4D)]
    /// Flush the registry to disk
    ///
    /// # Parameters
    ///
    /// - `handle`: The open registry handle
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceRegFlushRegistry(handle: RegistryHandle) -> i32;

    #[psp(0xFA8A5739)]
    /// Close the registry
    ///
    /// # Parameters
    ///
    /// - `handle`: The open registry handle
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceRegCloseRegistry(handle: RegistryHandle) -> i32;

    #[psp(0x1D8A762E)]
    /// Open a registry directory
    ///
    /// # Parameters
    ///
    /// - `handle`: The open registry handle
    /// - `name`: The path to the dir to open (e.g. `/CONFIG/SYSTEM`)
    /// - `mode`: Open mode (can be 1 or 2, probably read or read/write)
    /// - `dir_handle`: Pointer to a `Handle` to receive the registry dir handle
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceRegOpenCategory(
        handle: RegistryHandle,
        name: *const u8,
        mode: i32,
        dir_handle: *mut RegistryHandle,
    ) -> i32;

    #[psp(0x4CA16893)]
    /// Remove a registry dir
    ///
    /// # Parameters
    ///
    /// - `handle`: The open registry dir handle
    /// - `name`: The name of the key
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceRegRemoveCategory(
        handle: RegistryHandle,
        name: *const u8,
    ) -> i32;

    #[psp(0x0CAE832B)]
    /// Close the registry directory
    ///
    /// # Parameters
    ///
    /// - `dir_handle`: The open registry dir handle
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceRegCloseCategory(dir_handle: RegistryHandle) -> i32;

    #[psp(0x0D69BF40)]
    /// Flush the registry directory to disk
    ///
    /// # Parameters
    ///
    /// - `dir_handle`: The open registry dir handle
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceRegFlushCategory(dir_handle: RegistryHandle) -> i32;

    #[psp(0xD4475AA8, i5)]
    /// Get a key's information
    ///
    /// # Parameters
    ///
    /// - `dir_handle`: The open registry dir handle
    /// - `name`: Name of the key
    /// - `key_handle`: Pointer to a `Handle` to get registry key handle (used in `sceRegGetKeyValue`)
    /// - `type_`: Type of the key
    /// - `size`: The size of the key's value in bytes
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceRegGetKeyInfo(
        dir_handle: RegistryHandle,
        name: *const u8,
        key_handle: *mut RegistryHandle,
        type_: *mut KeyType,
        size: *mut usize,
    ) -> i32;

    #[psp(0xC5768D02)]
    /// Get a key's information by name
    ///
    /// # Parameters
    ///
    /// - `dir_handle`: The open registry dir handle
    /// - `name`: Name of the key
    /// - `type_`: Type of the key
    /// - `size`: The size of the key's value in bytes
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceRegGetKeyInfoByName(
        dir_handle: RegistryHandle,
        name: *const u8,
        type_: *mut KeyType,
        size: *mut usize,
    ) -> i32;

    #[psp(0x28A8E98A)]
    /// Get a key's value
    ///
    /// # Parameters
    ///
    /// - `dir_handle`: The open registry dir handle
    /// - `key_handle`: The open registry key handler (from `sceRegGetKeyInfo`)
    /// - `buf`: Buffer to hold the value
    /// - `size`: The size of the buffer
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceRegGetKeyValue(
        dir_handle: RegistryHandle,
        key_handle: RegistryHandle,
        buf: *mut c_void,
        size: usize,
    ) -> i32;

    #[psp(0x30BE0259)]
    /// Get a key's value by name
    ///
    /// # Parameters
    ///
    /// - `dir_handle`: The open registry dir handle
    /// - `name`: The key name
    /// - `buf`: Buffer to hold the value
    /// - `size`: The size of the buffer
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceRegGetKeyValueByName(
        dir_handle: RegistryHandle,
        name: *const u8,
        buf: *mut c_void,
        size: usize,
    ) -> i32;

    #[psp(0x17768E14)]
    /// Set a key's value
    ///
    /// # Parameters
    ///
    /// - `dir_handle`: The open registry dir handle
    /// - `name`: The key name
    /// - `buf`: Buffer to hold the value
    /// - `size`: The size of the buffer
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceRegSetKeyValue(
        dir_handle: RegistryHandle,
        name: *const u8,
        buf: *const c_void,
        size: usize,
    ) -> i32;

    #[psp(0x2C0DB9DD)]
    /// Get number of subkeys in the current dir
    ///
    /// # Parameters
    ///
    /// - `dir_handle`: The open registry dir handle
    /// - `num`: Pointer to an integer to receive the number
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceRegGetKeysNum(
        dir_handle: RegistryHandle,
        num: *mut i32,
    ) -> i32;

    #[psp(0x2D211135)]
    /// Get the key names in the current directory
    ///
    /// # Parameters
    ///
    /// - `dir_handle`: The open registry dir handle
    /// - `buf`: Buffer to hold the NUL terminated strings, should be of size `num * KEYNAME_SIZE`
    /// - `num`: Number of elements in buf
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceRegGetKeys(
        dir_handle: RegistryHandle,
        buf: *mut u8,
        num: i32,
    ) -> i32;

    #[psp(0x57641A81)]
    /// Create a key
    ///
    /// # Parameters
    ///
    /// - `dir_handle`: The open registry dir handle
    /// - `name`: Name of the key to create
    /// - `type_`: Type of key (note cannot be a directory type)
    /// - `size`: Size of the allocated value space
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceRegCreateKey(
        dir_handle: RegistryHandle,
        name: *const u8,
        type_: i32,
        size: usize,
    ) -> i32;

    #[psp(0xDEDA92BF)]
    /// Remove a registry (HONESTLY, DO NOT USE)
    ///
    /// # Parameters
    ///
    /// - `key`: Filled out registry parameter
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub fn sceRegRemoveRegistry(key: *mut RegistryKey) -> i32;
}
