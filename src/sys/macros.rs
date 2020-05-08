
/// Generate a zero-sized sentinel value.
///
/// Used to mark the top of a section, e.g. `.sceStub.text.ThreadManForUser`.
macro_rules! sentinel {
    ($name:ident, $section:expr) => {
        #[link_section = $section]
        #[no_mangle]
        static $name: () = ();
    }
}

macro_rules! count {
    ($single:ident) => { 1 };
    ($first:ident, $($rest:ident),*) => { 1 + count!($($rest),*) };
}

/// Generate a PSP function stub.
///
/// This macro is split from `sys_lib!` to allow for `concat!`-based generation.
/// If you try to generate a function like so...
///
/// ```ignore
/// #[link_section = concat!(".foo", ".bar")]
/// fn quux() {}
/// ```
///
/// ... you will receive an error. However, calling this macro in place works
/// fine:
///
/// ```ignore
/// stub!(quux, concat!(".foo", ".bar"));
/// ```
macro_rules! stub {
    ($name:ident, $section:expr) => {
        #[link_section = $section]
        extern "C" fn $name() {}
    }
}

/// Generate a PSP function NID.
///
/// This is a macro for the same reason as `stub!`. See documentation there for
/// details.
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
pub const fn lib_name_bytes_len(name: &str) -> usize {
    let name_len = name.as_bytes().len();
    return name_len + (4 - name_len % 4);
}

/// Convert a library name to a byte array.
///
/// This is intended to be used with `lib_name_bytes_len`.
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

/// A black box function opaque to the optimizer.
pub fn black_box<T>(dummy: T) -> T {
    unsafe {
        let ret = core::ptr::read_volatile(&dummy);
        core::mem::forget(dummy);
        ret
    }
}

/// A complex macro used to define and link a PSP system library.
///
/// See `src/sys.rs` usage for examples.
macro_rules! sys_lib {
    // Generate body with default ABI
    (__BODY $name:ident ($($arg:ident : $arg_ty:ty)*) $(-> $ret:ty)?) => {
        {
            use core::mem;

            // Reconstruct the function signature for the stub.
            type Target = extern "C" fn($($arg_ty),*) $(-> $ret)?;

            // The black box here is necessary to make this opaque to the
            // optimizer. If it is removed, the optimizer will be confused as
            // it does not know that these functions are mutated at runtime.
            //
            // TODO: Replace this with assembly for better performance.
            let f = mem::transmute::<_, Target>(
                macros::black_box(expr! { [< __ $name _stub >] } as usize)
            );

            f($($arg),*)
        }
    };

    // Generate body with an ABI mapper
    (__BODY $abi:ident $name:ident ($($arg:ident : $arg_ty:ty)*) $(-> $ret:ty)?) => {
        {
            core::mem::transmute(macros::black_box($abi(
                $($arg as u32),*,
                macros::black_box(core::mem::transmute(expr! { [< __ $name _stub >] } as usize)),
            )))
        }
    };

    (
        #![name = $lib_name:expr]
        #![flags = $lib_flags:expr]
        #![version = ($lib_major_version:expr, $lib_minor_version:expr)]

        $(
            #[psp($nid:expr $(, $abi:ident)?)]
            $(#[$attr:meta])*
            pub unsafe fn $name:ident($($arg:ident : $arg_ty:ty),* $(,)?)
            $(-> $ret:ty)?;
        )*
    ) => {
        item! {
            #[link_section = ".rodata.sceResident"]
            #[no_mangle]
            static [< __ $lib_name _RESIDENT >] : [u8; macros::lib_name_bytes_len($lib_name)] = macros::lib_name_bytes($lib_name);

            #[link_section = ".lib.stub"]
            #[no_mangle]
            static [< __ $lib_name _STUB >] : SceStubLibraryEntry = SceStubLibraryEntry {
                name: expr! { & [< __ $lib_name _RESIDENT >] [0] },
                version: [$lib_major_version, $lib_minor_version],
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
        }

        $(
            item! {
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
            }

            pub unsafe fn $name($($arg : $arg_ty),*) $(-> $ret)? {
                sys_lib!(
                    __BODY $($abi)?
                    $name($($arg : $arg_ty)*) $(-> $ret)?
                )
            }
        )*
    }
}
