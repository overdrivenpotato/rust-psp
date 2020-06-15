/// Generate a zero-sized sentinel value.
///
/// Used to mark the top of a section, e.g. `.sceStub.text.ThreadManForUser`.
#[cfg(target_os = "psp")]
macro_rules! sentinel {
    ($name:ident, $section:expr) => {
        #[link_section = $section]
        #[no_mangle]
        static $name: () = ();
    }
}

#[cfg(target_os = "psp")]
macro_rules! count {
    ($single:ident) => { 1 };
    ($first:ident, $($rest:ident),*) => { 1 + count!($($rest),*) };
}

/// Generate a PSP function stub.
///
/// This generates an assembly stub with the given section and name.
#[cfg(target_os = "psp")]
macro_rules! stub {
    ($name:ident, $section:expr) => {
        global_asm!(concat!(
            "
                .section ", $section, ", \"ax\", @progbits
                .global ", stringify!($name), "
                ", stringify!($name), ":
                    jr $ra
            "
        ));
    }
}

/// Generate a PSP function NID.
///
/// This macro is split from `psp_extern!` to allow for `concat!`-based
/// generation. If you try to generate a NID like so...
///
/// ```ignore
/// #[link_section = concat!(".foo", ".bar")]
/// static FOO: u32 = 123;
/// ```
///
/// ... you will receive an error. However, calling this macro in place works
/// fine:
///
/// ```ignore
/// nid!(FOO, concat!(".foo", ".bar"), 123);
/// ```
#[cfg(target_os = "psp")]
macro_rules! nid {
    ($name:ident, $section:expr, $value:expr) => {
        #[no_mangle]
        #[link_section = $section]
        static $name: u32 = $value;
    }
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
    (__BODY $name:ident ($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?) => {
        expr! {
            extern {
                fn [< __ $name _stub >]($($arg : $arg_ty),*) $(-> $ret)?;
            }

            [< __ $name _stub >] ($($arg),*)
        }
    };

    // Generate body with an ABI mapper
    (__BODY $abi:ident $name:ident ($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?) => {{
        expr! {
            extern {
                fn [< __ $name _stub >]($($arg : $arg_ty),*) $(-> $ret)?;
            }

            // The transmutes here are for newtypes that fit into a single
            // register.
            core::mem::transmute($abi(
                $(core::mem::transmute($arg)),*,
                core::mem::transmute([< __ $name _stub >] as usize),
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
        item! {
            // Just passed to the linker, not used anywhere else.
            #[cfg(target_os = "psp")]
            #[allow(non_snake_case)]
            mod [< __ $lib_name _mod >] {
                #[link_section = ".rodata.sceResident"]
                #[no_mangle]
                static [< __ $lib_name _RESIDENT >] : [u8; $crate::sys::macros::lib_name_bytes_len($lib_name)] = $crate::sys::macros::lib_name_bytes($lib_name);

                #[link_section = ".lib.stub"]
                #[no_mangle]
                static [< __ $lib_name _STUB >] : $crate::sys::SceStubLibraryEntry = $crate::sys::SceStubLibraryEntry {
                    name: expr! { & [< __ $lib_name _RESIDENT >] [0] },
                    version: [$lib_minor_version, $lib_major_version],
                    flags: $lib_flags,
                    len: 5,
                    v_stub_count: 0,
                    stub_count: count!($($name),*),
                    nid_table: & [< __ $lib_name _NID_START >] as *const () as *const _,
                    stub_table: & [< __ $lib_name _STUB_START >] as *const () as *const _,
                };

                sentinel!(
                    [< __ $lib_name _NID_START >],
                    concat!(".rodata.sceNid.", $lib_name)
                );

                sentinel!(
                    [< __ $lib_name _STUB_START >],
                    concat!(".sceStub.text.", $lib_name)
                );

                $(
                    stub!(
                        [< __ $name _stub >],
                        concat!(
                            ".sceStub.text.", $lib_name,
                            ".", stringify!($name)
                        )
                    );

                    nid!(
                        [< __ $name _NID >],
                        concat!(
                            ".rodata.sceNid.", $lib_name,
                            ".", stringify!($name)
                        ),
                        $nid
                    );
                )*
            }
        }

        $(
            $(#[$attr])*
            #[allow(non_snake_case)]
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
