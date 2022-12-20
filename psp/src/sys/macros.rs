#[cfg(target_os = "psp")]
use crate::sys::SceStubLibraryEntry;

/// A macro that enables the use of `concat!` inside the `#[link_section = ...]`
/// attribute.
#[cfg(target_os = "psp")]
macro_rules! link_section_concat {
    ($(#[link_section = $section:expr] $item:item)*) => {
        $(
            #[link_section = $section]
            $item
        )*
    }
}

/// A "function" stub.
///
/// This is a very dirty trick for LTO. Essentially, the PSP OS takes the
/// address of a stub and inserts 2 instructions (8 bytes) which are `jr $ra`
/// and `syscall xxx`. Traditionally, the C/C++ toolchain stores the initial
/// data here as just an empty function that immediately returns (8 bytes,
/// `jr $ra; nop`). However, we use it to store 2 references: to the NID and to
/// the library stub.
///
/// This results in LTO builds compiling in the NID and library stubs whenever a
/// function is called, as this struct references them. Thus, this struct
/// definition creates a dependency between the function call, and the NID + lib
/// stub. As mentioned earlier, these two addresses (8 bytes) are replaced with
/// two instructions (also 8 bytes) at runtime, so they are not actually called
/// as a function. With this method, nothing has to be marked `#[used]`, so LLVM
/// can automatically remove unreferenced NIDs and library stubs during LTO.
/// Compiling with LTO then only links the functions that are called, and no
/// more.
#[cfg(target_os = "psp")]
#[derive(Copy, Clone)]
pub(crate) struct Stub {
    // These are never read, but need to be written into as static items.
    #[allow(dead_code)]
    pub(crate) lib_addr: &'static SceStubLibraryEntry,
    #[allow(dead_code)]
    pub(crate) nid_addr: &'static u32,
}

/// Calculate the padded length for a library name.
///
/// The name is padded on the end with zeroes. Must be at least one and a
/// multiple of 4.
#[cfg(target_os = "psp")]
pub const fn lib_name_bytes_len(name: &str) -> usize {
    let name_len = name.as_bytes().len();
    return name_len + (4 - name_len % 4);
}

/// Convert a library name to a byte array.
///
/// This is intended to be used with `lib_name_bytes_len`.
#[cfg(target_os = "psp")]
pub const fn lib_name_bytes<const T: usize>(name: &str) -> [u8; T] {
    let mut buf = [0; T];

    let name_bytes = name.as_bytes();
    let mut i = 0;

    while i < name_bytes.len() {
        buf[i] = name_bytes[i];
        i += 1;
    }

    buf
}

/// A complex macro used to define and link a PSP system library.
macro_rules! psp_extern {
    // Generate body with default ABI.
    (__BODY $name:ident ($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?) => {{
        paste! {
            extern "C" {
                pub fn [< __ $name _stub >]($($arg : $arg_ty),*) $(-> $ret)?;
            }
            let func = [< __ $name _stub >];
            func($($arg),*)
        }
    }};

    // Generate body with an ABI mapper
    (__BODY $abi:ident $name:ident ($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?) => {{
        type Func = unsafe extern "C" fn($($arg : $arg_ty),*) $(-> $ret)?;

        paste! {
            extern "C" {
                pub fn [< __ $name _stub >]($($arg : $arg_ty),*) $(-> $ret)?;
            }
            let func = [< __ $name _stub >] as Func;

            // The transmutes here are for newtypes that fit into a single
            // register.
            core::mem::transmute($abi(
                $(core::mem::transmute($arg)),*,
                core::mem::transmute(func),
            ))
        }
    }};

    (
        #![name = $lib_name:expr]
        #![flags = $lib_flags:expr]
        #![version = ($lib_major_version:expr, $lib_minor_version:expr)]

        $(
            #[psp($nid:expr $(, $abi:ident)?)]
            $(#[$attr:meta])*
            pub fn $name:ident($($arg:ident : $arg_ty:ty),* $(,)?)
            $(-> $ret:ty)?;
        )*
    ) => {
        paste! {
            #[allow(non_snake_case)]
            mod [< __ $lib_name _mod >] {
                #[allow(unused)]
                use super::*;

                #[cfg(target_os = "psp")]
                link_section_concat! {
                    #[link_section = concat!(".rodata.sceResident.", $lib_name)]
                    #[allow(non_upper_case_globals)]
                    static [< __ $lib_name _RESIDENT >] : [u8; $crate::sys::macros::lib_name_bytes_len($lib_name)] = $crate::sys::macros::lib_name_bytes($lib_name);

                    #[link_section = concat!(".rodata.sceNid.", $lib_name)]
                    #[allow(non_upper_case_globals)]
                    static [< __ $lib_name _NID_START >] : () = ();

                    #[link_section = concat!(".sceStub.text.", $lib_name)]
                    #[allow(non_upper_case_globals)]
                    static [< __ $lib_name _STUB_START >] : () = ();

                    #[link_section = concat!(".lib.stub.entry.", $lib_name)]
                    #[allow(non_upper_case_globals)]
                    static [< __ $lib_name _STUB >] : $crate::sys::SceStubLibraryEntry = $crate::sys::SceStubLibraryEntry {
                        name: paste! { & [< __ $lib_name _RESIDENT >] [0] },
                        version: [$lib_minor_version, $lib_major_version],
                        flags: $lib_flags,
                        len: 5,
                        v_stub_count: 0,

                        // This is to be fixed up later by patching the ELF file,
                        // in cargo-psp. The reason we cannot determine this
                        // statically is that the real number of imported stubs
                        // is found during link time.
                        //
                        // For example, in LTO builds, imports can be removed at link
                        // time if they are not ever called. There may be (?) a way
                        // to get the linker to handle this, but I have not found a
                        // working method.
                        stub_count: 0,

                        nid_table: & [< __ $lib_name _NID_START >] as *const () as *const _,
                        stub_table: & [< __ $lib_name _STUB_START >] as *const () as *const _,
                    };
                }

                $(
                    #[allow(unused)]
                    use super::*;

                    #[cfg(target_os = "psp")]
                    link_section_concat! {
                        #[link_section = concat!(
                            ".rodata.sceNid.", $lib_name,
                            ".", stringify!($name)
                        )]
                        #[allow(non_upper_case_globals)]
                        static [< __ $name _NID >]: u32 = $nid;

                        #[link_section = concat!(
                            ".sceStub.text.", $lib_name,
                            ".", stringify!($name)
                        )]
                        #[no_mangle]
                        #[allow(non_upper_case_globals)]
                        static [< __ $name _stub >]: $crate::sys::macros::Stub = $crate::sys::macros::Stub {
                            lib_addr: &[< __ $lib_name _STUB >],
                            nid_addr: &[< __ $name _NID >],
                        };
                    }

                    $(#[$attr])*
                    #[allow(non_snake_case, clippy::missing_safety_doc)]
                    #[no_mangle]
                    pub unsafe extern fn $name($($arg : $arg_ty),*) $(-> $ret)? {
                        #[cfg(target_os = "psp")]
                        {
                            psp_extern!(
                                __BODY $($abi)?
                                $name($($arg : $arg_ty),*) $(-> $ret)?
                            )
                        }

                        #[cfg(not(target_os = "psp"))]
                        {
                            // Get rid of warnings
                            $(let _arg = $arg;)*
                            $(let _abi = $abi;)?

                            panic!("tried to call PSP system function on non-PSP target");
                        }
                    }
                )*
            }
        }

        paste! {
            $(
                pub use self :: [< __ $lib_name _mod >] :: $name;
            )*
        }
    }
}
