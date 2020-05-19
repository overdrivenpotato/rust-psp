/// Call a function accepting 6 arguments via the MIPS-EABI ABI.
///
/// This is not safe to call with a function that expects any other ABI.
#[inline(always)]
pub unsafe fn i6(
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    e: u32,
    f: u32,
    ptr: fn(u32, u32, u32, u32, u32, u32) -> u32,
) -> u32 {
    #[naked]
    #[inline(never)]
    unsafe fn inner() -> u32 {
        // Potential resource:
        // (scroll down to table) https://www.linux-mips.org/wiki/P32_Linux_ABI
        //
        // Page 3-18: http://web.archive.org/web/20040930224745/http://www.caldera.com/developers/devspecs/mipsabi.pdf
        // Copied from PDF:
        // Despite the fact that some or all of the arguments to a function are
        // passed in registers, always allocate space on the stack for all
        // arguments. This stack space should be a structure large enough to
        // contain all the arguments, aligned according to normal structure
        // rules (after promotion and structure return pointer insertion). The
        // locations within the stack frame used for arguments are called the
        // home locations.
        llvm_asm!(r#"
            // Store the return register as we are calling a function manually.
            addiu $$sp, -32
            sw $$ra, 8($$sp)

            // Load argument 5 and 6 into the appropriate registers. It doesn't
            // matter that they remain on the stack, they are effectively local
            // variables now.
            lw $$t0, 48($$sp)
            lw $$t1, 52($$sp)

            // Load and call the bridged function.
            lw $$t2, 56($$sp)
            jalr $$t2

            // Restore the stack and return.
            lw $$ra, 8($$sp)
            addiu $$sp, 32
            jr $$ra
        "#);

        core::intrinsics::unreachable()
    }

    type Target = fn(
        u32, u32, u32, u32, u32, u32,
        fn(u32, u32, u32, u32, u32, u32) -> u32
    ) -> u32;

    core::mem::transmute::<_, Target>(inner as usize)(a, b, c, d, e, f, ptr)
}

/// Call a function accepting 5 arguments via the MIPS-EABI ABI.
///
/// See documentation for `i6` for details.
// TODO: Naked functions should not take arguments. This should be implemented
// like `i6` instead.
#[naked]
#[inline(never)]
#[no_mangle]
pub unsafe extern "C" fn i5(
    _a: u32,
    _b: u32,
    _c: u32,
    _d: u32,
    _e: u32,
    _ptr: fn(u32, u32, u32, u32, u32) -> u32,
) -> u32 {
    llvm_asm!(
        "
            addiu $$sp, -32
            sw $$ra, 8($$sp)

            lw $$t0, 48($$sp)

            // Load and call the bridged function.
            lw $$t9, 52($$sp)
            jalr $$t9

            // Restore the stack and return.
            lw $$ra, 8($$sp)
            addiu $$sp, 32
            jr $$ra
        "
    );

    core::intrinsics::unreachable()
}

/// Call a function accepting 7 arguments via the MIPS-EABI ABI.
///
/// See documentation for `i6` for details.
#[inline(always)]
pub unsafe fn i7(
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    e: u32,
    f: u32,
    g: u32,
    ptr: fn(u32, u32, u32, u32, u32, u32, u32) -> u32,
) -> u32 {
    #[naked]
    #[inline(never)]
    unsafe fn inner() -> u32 {
        llvm_asm!(r#"
            addiu $$sp, -32
            sw $$ra, 8($$sp)

            lw $$t0, 48($$sp)
            lw $$t1, 52($$sp)
            lw $$t2, 56($$sp)

            lw $$t3, 60($$sp)
            jalr $$t3

            lw $$ra, 8($$sp)
            addiu $$sp, 32
            jr $$ra
        "#);

        core::intrinsics::unreachable()
    }

    type Target = fn(
        u32, u32, u32, u32, u32, u32, u32,
        fn(u32, u32, u32, u32, u32, u32, u32) -> u32
    ) -> u32;

    core::mem::transmute::<_, Target>(inner as usize)(a, b, c, d, e, f, g, ptr)
}
