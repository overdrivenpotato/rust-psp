//! VFPU support.

/// A macro-based VFPU assembler.
///
/// This follows the standard Rust `asm!` macro syntax, with support for VFPU
/// instructions.
///
/// Limitations:
///
/// - Currently, registers cannot be specified by name in operands. For example
///   this is invalid: `out("t0") _`, instead you must use `out("$8")`.
///
/// # A note on transposed matrices
///
/// While some documentation online suggests that the `M___` registers represent
/// matrices, and the adjacent `E___` registers represent their inverse, this is
/// somewhat wrong.
///
/// It can be better said that `M___` interprets the registers as being stored
/// in row-major format, and `E___` interprets the registers as being stored in
/// column-major format. While many instructions (`vmmov`, `vmidt`, `vmzero`)
/// aren't sensitive to matrix transpositions, this matters for `vtfm_` and
/// `vhtfm_`.
///
/// `vmmul` is an exception to this rule. See [this comment] for more details.
///
/// [this comment]: https://github.com/overdrivenpotato/rust-psp/issues/112#issuecomment-1043535976
#[macro_export]
macro_rules! vfpu_asm {
    // Kickstart the parser.
    ($($t:tt)*) => {{
        $crate::vfpu_asm_next!((asm:) () $($t)*)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! vfpu_asm_next {
    // Extract an assembly literal.
    ((asm: $($a:tt)* ) () $code2:literal $(, $($t:tt)*)?) => {
        $crate::unstringify!(let $tokens = unstringify!($code2) in {
            $crate::vfpu_asm_next!((asm: $($a)* ($crate::instruction!($tokens))) () $($($t)*)?)
        })
    };

    // Extract a stringified assembly literal.
    ((asm: $($a:tt)* ) () stringify!( $($token:tt)* ) $(, $($t:tt)*)?) => {
        $crate::vfpu_asm_next!((asm: $($a)* ($crate::instruction!($($token)*))) () $($($t)*)?)
    };

    // If the next token isn't a directive, start parsing operands.
    ((asm: $($a:tt)* ) () $($t:tt)*) => {
        $crate::vfpu_asm_next!((asm: $($a)* ) (ops: ) $($t)*)
    };

    // Extract an option directive.
    ((asm: $($a:tt)* ) (ops: $($b:tt)* ) options($($option:ident),+) $(, $($t:tt)*)?) => {
        $crate::vfpu_asm_next!((asm: $($a)* ) (ops: $($b)* (options ($($option),+);) ) $($($t),*)? )
    };

    // Extract an unaliased register operand.
    ((asm: $($a:tt)* ) (ops: $($b:tt)* ) $op:ident($reg:tt) $place1:tt $(=> $place2:tt)? $(, $($t:tt)*)?) => {
        $crate::vfpu_asm_next!((asm: $($a)* ) (ops: $($b)* (regop $op($reg) $place1 $(=> $place2)? ;) ) $($($t)*)? )
    };

    // Extract an aliased register operand.
    ((asm: $($a:tt)* ) (ops: $($b:tt)* ) $alias:ident = $op:ident($reg:tt) $place1:tt $(=> $place2:tt)? $(, $($t:tt)*)?) => {
        $crate::vfpu_asm_next!((asm: $($a)* ) (ops: $($b)* (regop [$alias =] $op($reg) $place1 $(=> $place2)? ;) ) $($($t)*)? )
    };

    // Extract an unaliased const operand.
    ((asm: $($a:tt)* ) (ops: $($b:tt)* ) const $val:expr $(, $($t:tt)*)?) => {
        $crate::vfpu_asm_next!((asm: $($a)* ) (ops: $($b)* (const_ $val ;) ) $($($t)*)? )
    };

    // Extract an aliased const operand.
    ((asm: $($a:tt)* ) (ops: $($b:tt)* ) $alias:ident = const $ex:expr $(, $($t:tt)*)?) => {
        $crate::vfpu_asm_next!((asm: $($a)* ) (ops: $($b)* (const_ [$alias =] $ex ;) ) $($($t)*)? )
    };

    // Extract an unaliased sym operand.
    ((asm: $($a:tt)* ) (ops: $($b:tt)* ) sym $pa:path $(, $($t:tt)*)?) => {
        $crate::vfpu_asm_next!((asm: $($a)* ) (ops: $($b)* (sym $pa ;) ) $($($t)*)? )
    };

    // Extract an aliased sym operand.
    ((asm: $($a:tt)* ) (ops: $($b:tt)* ) $alias:ident = sym $pa:path $(, $($t:tt)*)?) => {
        $crate::vfpu_asm_next!((asm: $($a)* ) (ops: $($b)* (sym [$alias =] $pa ;) ) $($($t)*)? )
    };

    // No more tokens to parse. Build the assembly.
    (
        (asm: $( ( $($asm:tt)* ) )+)
        $((ops:
            $((
                $(regop  $([$io_name:tt =])? $op:ident($io_reg:tt) $io_p1:tt $(=> $io_p2:tt)? ;)?
                $(const_ $([$c_name:tt =])? $c_expr:expr ;)?
                $(sym    $([$s_name:tt =])? $s_path:path ;)?
                $(options ($($option:ident),+) ;)?
            ))*
        ))?
    ) => {{
        #[cfg(target_os = "psp")]
        {
            core::arch::asm!(
                ".set push",
                ".set noreorder",
                ".set noat",

                ".set __psp_regnum_$0, 0",
                ".set __psp_regnum_$1, 1",
                ".set __psp_regnum_$2, 2",
                ".set __psp_regnum_$3, 3",
                ".set __psp_regnum_$4, 4",
                ".set __psp_regnum_$5, 5",
                ".set __psp_regnum_$6, 6",
                ".set __psp_regnum_$7, 7",
                ".set __psp_regnum_$8, 8",
                ".set __psp_regnum_$9, 9",
                ".set __psp_regnum_$10, 10",
                ".set __psp_regnum_$11, 11",
                ".set __psp_regnum_$12, 12",
                ".set __psp_regnum_$13, 13",
                ".set __psp_regnum_$14, 14",
                ".set __psp_regnum_$15, 15",
                ".set __psp_regnum_$16, 16",
                ".set __psp_regnum_$17, 17",
                ".set __psp_regnum_$18, 18",
                ".set __psp_regnum_$19, 19",
                ".set __psp_regnum_$20, 20",
                ".set __psp_regnum_$21, 21",
                ".set __psp_regnum_$22, 22",
                ".set __psp_regnum_$23, 23",
                ".set __psp_regnum_$24, 24",
                ".set __psp_regnum_$25, 25",
                ".set __psp_regnum_$26, 26",
                ".set __psp_regnum_$27, 27",
                ".set __psp_regnum_$28, 28",
                ".set __psp_regnum_$29, 29",
                ".set __psp_regnum_$30, 30",
                ".set __psp_regnum_$31, 31",

                // TODO: Can VFPU instructions ever even access coproc1 registers?
                // If not, this part can be removed.
                //
                ".set __psp_regnum_$f0, 0",
                ".set __psp_regnum_$f1, 1",
                ".set __psp_regnum_$f2, 2",
                ".set __psp_regnum_$f3, 3",
                ".set __psp_regnum_$f4, 4",
                ".set __psp_regnum_$f5, 5",
                ".set __psp_regnum_$f6, 6",
                ".set __psp_regnum_$f7, 7",
                ".set __psp_regnum_$f8, 8",
                ".set __psp_regnum_$f9, 9",
                ".set __psp_regnum_$f10, 10",
                ".set __psp_regnum_$f11, 11",
                ".set __psp_regnum_$f12, 12",
                ".set __psp_regnum_$f13, 13",
                ".set __psp_regnum_$f14, 14",
                ".set __psp_regnum_$f15, 15",
                ".set __psp_regnum_$f16, 16",
                ".set __psp_regnum_$f17, 17",
                ".set __psp_regnum_$f18, 18",
                ".set __psp_regnum_$f19, 19",
                ".set __psp_regnum_$f20, 20",
                ".set __psp_regnum_$f21, 21",
                ".set __psp_regnum_$f22, 22",
                ".set __psp_regnum_$f23, 23",
                ".set __psp_regnum_$f24, 24",
                ".set __psp_regnum_$f25, 25",
                ".set __psp_regnum_$f26, 26",
                ".set __psp_regnum_$f27, 27",
                ".set __psp_regnum_$f28, 28",
                ".set __psp_regnum_$f29, 29",
                ".set __psp_regnum_$f30, 30",
                ".set __psp_regnum_$f31, 31",

                // This block defines a macro, and a symbol to guard that macro so
                // that it is only defined once. The macro is equivalent to:
                //
                // __psp_reg_or(register, left shift, orval)
                //
                // Arguments:
                // - register: The register name, as spit out by the rust assembler.
                //   E.g. $1, $5, or $f0, $f13, etc..
                // - left shift: The mount to shift the register number left by
                // - orval: The value to or the shifted register with
                //
                // Once invoked, the macro defines a .word value, so it can be used
                // with an instruction template to create VFPU instructions.
                //
                // The registers are converted to numbers with the use of the above
                // variables, following the `__psp_regnum_$N` pattern, where N is
                // the register number, (0, 5, f0, f13).
                ".ifndef __psp_reg_or_defined",
                ".set __psp_reg_or_defined, 1",
                ".macro __psp_reg_or reg lshift orval",
                ".word ((__psp_regnum_\\()\\reg\\() << \\lshift)|\\orval)",
                ".endm",
                ".endif",

                ".align 2",
                $( $($asm)* ),*,
                ".set pop",
                $(
                    $(
                        $($($io_name =)? $op($io_reg) $io_p1 $(=> $io_p2)?)?
                        $($($c_name =)? const $c_expr)?
                        $($($s_name =)? sym $s_path)?
                        $(options ($($option),*))?
                    ),*
                )?
            )
        }

        #[cfg(not(target_os = "psp"))]
        {
            // The type signature here lets you obtain any value, which avoids
            // dead code warnings.
            //
            // Adding `unsafe` ensures that the macro caller always calls PSP
            // assembly in an unsafe context.
            #[inline(always)]
            unsafe fn die<T>() -> T {
                panic!("tried running vfpu_asm on a non-PSP platform");
            }

            die::<()>();

            // Fix errors for output variables which are never assigned). The
            // type can be anything due to the signature of `die`.

            $(
                $(
                    {
                        $($crate::psp_asm_discard!(regop $op $io_p1 $(=> $io_p2)?);)?
                        $($crate::psp_asm_discard!(const_ $c_expr);)?
                    }
                )*
            )?
        }
    }};
}

/// Like `stringify!`, but with several extra features:
///
/// # Preserves assembly string templates
///
/// Assembly strings break when spaces are added, i.e.:
///
/// ```rust,compile_fail
/// asm!("xor { }, { }", out("eax") _);
/// ```
///
/// This macro preserves format strings, and also understands double braces:
///
/// ```
/// let s = format!(stringify_asm!(op {  }, { 2 : ? }, {{{1}}}), 123, 456, "test");
/// assert_eq!(s, r#"op 123 , "test" , { 456 }"#));
/// ```
///
/// # Register names
///
/// This macro automatically converts register names (`a0`, `t4`, `s2`, etc) to
/// corresponding numbers (`$4`, `$12`, etc).
#[macro_export]
#[doc(hidden)]
macro_rules! stringify_asm {
    // Catch double braces.
    ({{ $($t1:tt)* }} $($($t2:tt)+)?) => {
        concat!("{{ ", concat!($crate::stringify_asm!($($t1)*)), " }}" $(, " ", $crate::stringify_asm!($($t2)+))?)
    };

    // Catch fmt directives.
    ({ $($t1:tt)* } $($($t2:tt)+)?) => {
        concat!("{", concat!($(stringify!($t1)),*), "}" $(, " ", $crate::stringify_asm!($($t2)+))?)
    };

    // Catch preprocessor directives
    (. $a:ident $($($t2:tt)+)?) => {
        concat!(
            ".", stringify!($a)
            $(, " ", $crate::stringify_asm!($($t2)+))?
        )
    };

    // Catch mnemonics with 3 dots
    ($a:ident . $b:ident . $c:ident $($($t2:tt)+)?) => {
        concat!(
            stringify!($a), ".", stringify!($b), ".", stringify!($c)
            $(, " ", $crate::stringify_asm!($($t2)+))?
        )
    };

    // We can't make a rule for dollar signs due to macro limitations. This is
    // a workaround that attempts to parse a dollar sign anyways.
    ($t1:tt $t2:tt $($t3:tt)*) => {
        $crate::try_stringify_reg!(($t1) $t2 $($t3)*)
    };



    // Base case.
    ($token:tt) => { $crate::try_stringify_mips_reg!($token) };
}

#[macro_export]
#[doc(hidden)]
macro_rules! psp_asm_discard {
    (regop inout $io_p1:tt => _) => {
        let _ = $io_p1;
    };

    (regop inout $io_p1:tt => $io_p2:tt) => {
        let _ = $io_p1;
        $io_p2 = die();
    };

    (regop inlateout $io_p1:tt => _) => {
        let _ = $io_p1;
    };

    (regop inlateout $io_p1:tt => $io_p2:tt) => {
        let _ = $io_p1;
        $io_p2 = die();
    };

    (regop out _) => {};

    (regop out $io_p1:tt) => {
        $io_p1 = die();
    };

    (regop lateout $io_p1:tt) => {
        $io_p1 = die();
    };

    (regop in $io_p1:tt) => {
        let _ = $io_p1;
    };

    (regop const_ $c_expr:expr) => {
        let _ = $c_expr;
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! try_stringify_reg {
    (($) $b:tt $($($t:tt)+)?) => {
        concat!("$", stringify!($b) $(, " ", $crate::stringify_asm!($($t)+))? )
    };

    // If nothing else, try to interpret this as a register mnemonic.
    (($a:tt) $b:tt $($t:tt)*) => {
        concat!($crate::try_stringify_mips_reg!($a), " ", $crate::stringify_asm!($b $($t)*) )
    };
}

// The instruction encodings here were mainly obtained from the following link:
// http://hitmen.c02.at/files/yapspd/psp_doc/chap4.html#sec4.9
//
// Missing instructions were discovered via PSP toolchain GCC patches.
//
// Almost all instructions here were generated using vim macros operating on the
// table linked above.
#[macro_export]
#[doc(hidden)]
macro_rules! instruction {
    // No offset
    (lv.s $t:ident, $s:tt) => { $crate::instruction!(lv.s $t, 0($s)) };

    // lv.s 110110ss sssttttt oooooooo oooooo0t
    (lv.s $t:ident, $offset:literal ( $s:tt )) => {
        concat!(
            "__psp_reg_or ", $crate::stringify_asm!($s), " (16+5) ", " (",
                "(0b11001000 << 24) | ",
                "((", $crate::register_single!($t), "& 0b11111) << 16) | ",
                "((((", stringify!($offset), " / 4) >> 6) & 0xff) << 8) | ",
                "(((", stringify!($offset), " / 4) << 2) & 0xff) | ((", $crate::register_single!($t), " >> 5) & 1)",
            ")",
        )
    };

    // No offset
    (lv.q $t:ident, $s:tt) => { $crate::instruction!(lv.q $t, 0($s)) };

    // lv.q 110110ss sssttttt oooooooo oooooo0t
    (lv.q $t:ident, $offset:literal ( $s:tt )) => {
        concat!(
            "__psp_reg_or ", $crate::stringify_asm!($s), " (16+5) ", " (",
                "(0b11011000 << 24) | ",
                "((", $crate::register_quad!($t), "& 0b11111) << 16) | ",
                "((((", stringify!($offset), " / 4) >> 6) & 0xff) << 8) | ",
                "(((", stringify!($offset), " / 4) << 2) & 0xff) | ((", $crate::register_quad!($t), " >> 5) & 1)",
            ")",
        )
    };

    // No offset, no writeback
    (sv.s $t:ident, $s:tt) => {
        $crate::instruction!(sv.s $t, 0($s), wb:0)
    };

    // No offset, has writeback
    (sv.s $t:ident, $s:tt, wb) => {
        $crate::instruction!(sv.s $t, 0($s), wb:1)
    };

    // Has offset, no writeback
    (sv.s $t:ident, $offset:literal ( $s:tt )) => {
        $crate::instruction!(sv.s $t, $offset ($s), wb:0)
    };

    // Has offset, has writeback
    (sv.s $t:ident, $offset:literal ( $s:tt ), wb) => {
        $crate::instruction!(sv.s $t, $offset ($s), wb:1)
    };

    // sv.s 111110ss sssttttt oooooooo oooooowt
    (sv.s $t:ident, $offset:literal ( $s:tt ), wb:$wb:literal) => {
        concat!(
            "__psp_reg_or ", $crate::stringify_asm!($s), " (16+5) (",
                "(0b11101000 << 24) | ",
                "((", $crate::register_single!($t), " & 0b11111) << 16) | ",
                "((((", stringify!($offset), " / 4) >> 6) & 0xff) << 8) | ",
                "(((", stringify!($offset), " / 4) << 2) & 0xff) | ",
                    "((", $crate::register_single!($t), " >> 5) & 1) | ",
                    "(", stringify!($wb), " << 1)",
            ")",
        )
    };

    // No offset, no writeback
    (sv.q $t:ident, $s:tt) => {
        $crate::instruction!(sv.q $t, 0($s), wb:0)
    };

    // No offset, has writeback
    (sv.q $t:ident, $s:tt, wb) => {
        $crate::instruction!(sv.q $t, 0($s), wb:1)
    };

    // Has offset, no writeback
    (sv.q $t:ident, $offset:literal ( $s:tt )) => {
        $crate::instruction!(sv.q $t, $offset ($s), wb:0)
    };

    // Has offset, has writeback
    (sv.q $t:ident, $offset:literal ( $s:tt ), wb) => {
        $crate::instruction!(sv.q $t, $offset ($s), wb:1)
    };

    // sv.q 111110ss sssttttt oooooooo oooooowt
    (sv.q $t:ident, $offset:literal ( $s:tt ), wb:$wb:literal) => {
        concat!(
            "__psp_reg_or ", $crate::stringify_asm!($s), " (16+5) (",
                "(0b11111000 << 24) | ",
                "((", $crate::register_quad!($t), " & 0b11111) << 16) | ",
                "((((", stringify!($offset), " / 4) >> 6) & 0xff) << 8) | ",
                "(((", stringify!($offset), " / 4) << 2) & 0xff) | ",
                    "((", $crate::register_quad!($t), " >> 5) & 1) | ",
                    "(", stringify!($wb), " << 1)",
            ")",
        )
    };

    // mtv 0100 1000 111 sssss 0000 0000 0 ddddddd
    (mtvc $t:ident, $s:ident) => { $crate::instruction!(mtv $t, $s) };

    (mtv $s:tt, $d:ident) => {
        concat!(
            "__psp_reg_or ", $crate::stringify_asm!($s), " 16 (",
                "(0b01001000 << 24) | ",
                "(0b11100000 << 16) | ",
                "(0 << 8) |",
                $crate::register_single!($d),
            ")",
        )
    };


    // mfv 0100 1000 011 ddddd 000000000 sssssss
    (mfvc $t:ident, $s:ident) => { $crate::instruction!(mfv $t, $s) };

    (mfv $d:tt, $s:ident) => {
        concat!(
            "__psp_reg_or ", $crate::stringify_asm!($d), " 16 (",
                "(0b01001000 << 24) | ",
                "(0b01100000 << 16) | ",
                "(0 << 8) |",
                $crate::register_single!($s),
            ")",
        )
    };

    (vpfxd $($t:tt)*) => {
        $crate::vpfx_instr!(_vpfxd_internal (stack:) (cur:) $($t)*);
    };

    (_vpfxd_internal [$($x:tt)*]) => {
        $crate::instruction!(_vpfxd_internal [$($x)*], [])
    };

    (_vpfxd_internal [$($x:tt)*], [$($y:tt)*]) => {
        $crate::instruction!(_vpfxd_internal [$($x)*], [$($y)*], [])
    };

    (_vpfxd_internal [$($x:tt)*], [$($y:tt)*], [$($z:tt)*]) => {
        $crate::instruction!(_vpfxd_internal [$($x)*], [$($y)*], [$($z)*], [])
    };

    // vpfxd 1101 1110 iiiiiiii iiiiiiii iiiiiiii
    (_vpfxd_internal [$($x:tt)*], [$($y:tt)*], [$($z:tt)*], [$($w:tt)*]) => {
        concat!(
            ".word (0b11011110 << 24)",

            // First the lowest byte
            "| (", $crate::instruction_prefix_d!($($x)*), " & 0x3)",
            "| ((", $crate::instruction_prefix_d!($($y)*), " & 0x3) << 2)",
            "| ((", $crate::instruction_prefix_d!($($z)*), " & 0x3) << 4)",
            "| ((", $crate::instruction_prefix_d!($($w)*), " & 0x3) << 6)",

            // Then the rest
            "| (", $crate::instruction_prefix_d!($($x)*), " & 0xffff00)",
            "| ((", $crate::instruction_prefix_d!($($y)*), " & 0xffff00) << 1)",
            "| ((", $crate::instruction_prefix_d!($($z)*), " & 0xffff00) << 2)",
            "| ((", $crate::instruction_prefix_d!($($w)*), " & 0xffff00) << 3)",
        )
    };



    // Internal pre-parse variants of this prefix instruction. Arguments are
    // surrounded by square brackets for easy use.

    (vpfxs $($x:tt)+) => {
        $crate::vpfx_instr!(_vpfxs_internal (stack:) (cur:) $($x)+)
    };

    // vpfxs 1101 1100 iiiiiiii iiiiiiii iiiiiiii
    (_vpfxs_internal [$($x:tt)+], [$($y:tt)+], [$($z:tt)+], [$($w:tt)+]) => {
        concat!(
            ".word (0b11011100 << 24)",

            // First the lowest byte
            "| (", $crate::instruction_prefix!($($x)+), " & 0x3)",
            "| ((", $crate::instruction_prefix!($($y)+), " & 0x3) << 2)",
            "| ((", $crate::instruction_prefix!($($z)+), " & 0x3) << 4)",
            "| ((", $crate::instruction_prefix!($($w)+), " & 0x3) << 6)",

            // Then the rest
            "| (", $crate::instruction_prefix!($($x)+), " & 0xffff00)",
            "| ((", $crate::instruction_prefix!($($y)+), " & 0xffff00) << 1)",
            "| ((", $crate::instruction_prefix!($($z)+), " & 0xffff00) << 2)",
            "| ((", $crate::instruction_prefix!($($w)+), " & 0xffff00) << 3)",
        )
    };

    (_vpfxs_internal [$($x:tt)+]) => {
        $crate::instruction!(_vpfxs_internal [$($x)+], [Y])
    };

    (_vpfxs_internal [$($x:tt)+], [$($y:tt)+]) => {
        $crate::instruction!(_vpfxs_internal [$($x)+], [$($y)+], [Z])
    };

    (_vpfxs_internal [$($x:tt)+], [$($y:tt)+], [$($z:tt)+]) => {
        $crate::instruction!(_vpfxs_internal [$($x)+], [$($y)+], [$($z)+], [W])
    };



    (vpfxt $($x:tt)+) => {
        $crate::vpfx_instr!(_vpfxt_internal (stack:) (cur:) $($x)+)
    };

    (_vpfxt_internal [$($x:tt)+]) => {
        $crate::instruction!(_vpfxt_internal [$($x)+], [Y])
    };

    (_vpfxt_internal [$($x:tt)+], [$($y:tt)+]) => {
        $crate::instruction!(_vpfxt_internal [$($x)+], [$($y)+], [Z])
    };

    (_vpfxt_internal [$($x:tt)+], [$($y:tt)+], [$($z:tt)+]) => {
        $crate::instruction!(_vpfxt_internal [$($x)+], [$($y)+], [$($z)+], [W])
    };

    // vpfxs 1101 1101 iiiiiiii iiiiiiii iiiiiiii
    (_vpfxt_internal [$($x:tt)+], [$($y:tt)+], [$($z:tt)+], [$($w:tt)+]) => {
        concat!(
            ".word (0b11011101 << 24)",

            // First the lowest byte
            "| (", $crate::instruction_prefix!($($x)+), " & 0x3)",
            "| ((", $crate::instruction_prefix!($($y)+), " & 0x3) << 2)",
            "| ((", $crate::instruction_prefix!($($z)+), " & 0x3) << 4)",
            "| ((", $crate::instruction_prefix!($($w)+), " & 0x3) << 6)",

            // Then the rest
            "| (", $crate::instruction_prefix!($($x)+), " & 0xffff00)",
            "| ((", $crate::instruction_prefix!($($y)+), " & 0xffff00) << 1)",
            "| ((", $crate::instruction_prefix!($($z)+), " & 0xffff00) << 2)",
            "| ((", $crate::instruction_prefix!($($w)+), " & 0xffff00) << 3)",
        )
    };

    // Performs element-wise floating point absolute value

    (vabs.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (0b0000001 << 16)",
        )
    };

    (vabs.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (0b0000001 << 16)",
        )
    };

    (vabs.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (0b0000001 << 16)",
        )
    };

    (vabs.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (0b0000001 << 16)",
        )
    };

    // Performs element-wise floating point addition

    (vadd.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01100000000000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (", $crate::register_single!($rt), " << 16)",
        )
    };

    (vadd.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01100000000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (", $crate::register_pair!($rt), " << 16)",
        )
    };

    (vadd.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01100000000000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (", $crate::register_triple!($rt), " << 16)",
        )
    };

    (vadd.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01100000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (", $crate::register_quad!($rt), " << 16)",
        )
    };

    // Performs element-wise floating point asin(rs)⋅2/π operation

    (vasin.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (0b0010111 << 16)",
        )
    };

    (vasin.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (0b0010111 << 16)",
        )
    };

    (vasin.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (0b0010111 << 16)",
        )
    };

    (vasin.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (0b0010111 << 16)",
        )
    };

    // Calculates the average value of the vector elements

    (vavg.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (0b1000111 << 16)",
        )
    };

    (vavg.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (0b1000111 << 16)",
        )
    };

    (vavg.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (0b1000111 << 16)",
        )
    };

    // Performs a `butterfly` operation between the input elements.

    (vbfy1.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (0b1000010 << 16)",
        )
    };

    (vbfy1.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (0b1000010 << 16)",
        )
    };

    // Performs a `butterfly` operation between the input elements.

    (vbfy2.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (0b1000011 << 16)",
        )
    };

    // Converts the input packed chars into full 32 bit integers in the output register. The input is placed on the most significant bits of the output integer, while the least significant bits are filled with zeros.

    (vc2i.s $rd:ident $([$($rdp:tt)+])?, $rs:ident) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (0b0111001 << 16)",
        )
    };

    // Performs element-wise floating point cos(π/2⋅rs) operation

    (vcos.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (0b0010011 << 16)",
        )
    };

    (vcos.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (0b0010011 << 16)",
        )
    };

    (vcos.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (0b0010011 << 16)",
        )
    };

    (vcos.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (0b0010011 << 16)",
        )
    };

    // Performs a partial cross-product operation

    (vcrs.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b01100110100000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (", $crate::register_triple!($rt), " << 16)",
        )
    };

    // Performs a full cross-product operation

    (vcrsp.t $rd:ident, $rs:ident, $rt:ident) => {
        concat!(
            ".word 0b11110010100000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (", $crate::register_triple!($rt), " << 16)",
        )
    };

    // Loads a predefined indexed floating point constant specified by the immediate field

    (vcst.s $rd:ident $([$($rdp:tt)+])?, $imm5:ident) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000011000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::vfpu_const!($imm5), " << 16)",
        )
    };

    (vcst.p $rd:ident $([$($rdp:tt)+])?, $imm5:ident) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000011000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::vfpu_const!($imm5), " << 16)",
        )
    };

    (vcst.t $rd:ident $([$($rdp:tt)+])?, $imm5:ident) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000011000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::vfpu_const!($imm5), " << 16)",
        )
    };

    (vcst.q $rd:ident $([$($rdp:tt)+])?, $imm5:ident) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000011000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::vfpu_const!($imm5), " << 16)",
        )
    };

    // Performs a 2x2 matrix determinant between two matrix rows

    (vdet.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b01100111000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (", $crate::register_pair!($rt), " << 16)",
        )
    };

    // Performs element-wise floating point division

    (vdiv.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01100011100000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (", $crate::register_single!($rt), " << 16)",
        )
    };

    (vdiv.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01100011100000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (", $crate::register_pair!($rt), " << 16)",
        )
    };

    (vdiv.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01100011100000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (", $crate::register_triple!($rt), " << 16)",
        )
    };

    (vdiv.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01100011100000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (", $crate::register_quad!($rt), " << 16)",
        )
    };

    // Performs vector floating point dot product

    (vdot.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01100100100000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (", $crate::register_pair!($rt), " << 16)",
        )
    };

    (vdot.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01100100100000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (", $crate::register_triple!($rt), " << 16)",
        )
    };

    (vdot.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01100100100000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (", $crate::register_quad!($rt), " << 16)",
        )
    };

    // Performs element-wise floating point exp2(rs) operation

    (vexp2.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (0b0010100 << 16)",
        )
    };

    (vexp2.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (0b0010100 << 16)",
        )
    };

    (vexp2.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (0b0010100 << 16)",
        )
    };

    (vexp2.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (0b0010100 << 16)",
        )
    };

    // Converts the float inputs to float16 (half-float) and packs them in pairs in the output register. The conversion process may naturally result in precision loss.

    (vf2h.p $rd:ident $([$($rdp:tt)+])?, $rs:ident) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (0b0110010 << 16)",
        )
    };

    (vf2h.q $rd:ident $([$($rdp:tt)+])?, $rs:ident) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (0b0110010 << 16)",
        )
    };

    // Performs element-wise float to integer conversion with optional scaling factor, rounding down (that is, towards the previous, equal or smaller, integer value)

    (vf2id.s $rd:ident $([$($rdp:tt)+])?, $rs:ident, $scale:expr) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010010011000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (", stringify!($scale), " << 16)",
        )
    };

    (vf2id.p $rd:ident $([$($rdp:tt)+])?, $rs:ident, $scale:expr) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010010011000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (", stringify!($scale), " << 16)",
        )
    };

    (vf2id.t $rd:ident $([$($rdp:tt)+])?, $rs:ident, $scale:expr) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010010011000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (", stringify!($scale), " << 16)",
        )
    };

    (vf2id.q $rd:ident $([$($rdp:tt)+])?, $rs:ident, $scale:expr) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010010011000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (", stringify!($scale), " << 16)",
        )
    };

    // Performs element-wise float to integer conversion with optional scaling factor, rounding to the nearest integer

    (vf2in.s $rd:ident $([$($rdp:tt)+])?, $rs:ident, $scale:expr) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010010000000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (", stringify!($scale), " << 16)",
        )
    };

    (vf2in.p $rd:ident $([$($rdp:tt)+])?, $rs:ident, $scale:expr) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010010000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (", stringify!($scale), " << 16)",
        )
    };

    (vf2in.t $rd:ident $([$($rdp:tt)+])?, $rs:ident, $scale:expr) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010010000000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (", stringify!($scale), " << 16)",
        )
    };

    (vf2in.q $rd:ident $([$($rdp:tt)+])?, $rs:ident, $scale:expr) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010010000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (", stringify!($scale), " << 16)",
        )
    };

    // Performs element-wise float to integer conversion with optional scaling factor, rounding up (that is, towards the next, equal or greater, integer value)

    (vf2iu.s $rd:ident $([$($rdp:tt)+])?, $rs:ident, $scale:expr) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010010010000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (", stringify!($scale), " << 16)",
        )
    };

    (vf2iu.p $rd:ident $([$($rdp:tt)+])?, $rs:ident, $scale:expr) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010010010000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (", stringify!($scale), " << 16)",
        )
    };

    (vf2iu.t $rd:ident $([$($rdp:tt)+])?, $rs:ident, $scale:expr) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010010010000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (", stringify!($scale), " << 16)",
        )
    };

    (vf2iu.q $rd:ident $([$($rdp:tt)+])?, $rs:ident, $scale:expr) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010010010000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (", stringify!($scale), " << 16)",
        )
    };

    // Performs element-wise float to integer conversion with optional scaling factor, truncating the decimal argument (that is, rounding towards zero)

    (vf2iz.s $rd:ident $([$($rdp:tt)+])?, $rs:ident, $scale:expr) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010010001000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (", stringify!($scale), " << 16)",
        )
    };

    (vf2iz.p $rd:ident $([$($rdp:tt)+])?, $rs:ident, $scale:expr) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010010001000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (", stringify!($scale), " << 16)",
        )
    };

    (vf2iz.t $rd:ident $([$($rdp:tt)+])?, $rs:ident, $scale:expr) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010010001000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (", stringify!($scale), " << 16)",
        )
    };

    (vf2iz.q $rd:ident $([$($rdp:tt)+])?, $rs:ident, $scale:expr) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010010001000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (", stringify!($scale), " << 16)",
        )
    };

    // Adds all vector elements toghether producing a single result

    (vfad.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (0b1000110 << 16)",
        )
    };

    (vfad.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (0b1000110 << 16)",
        )
    };

    (vfad.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (0b1000110 << 16)",
        )
    };

    // Loads a float16 immediate value in a register

    (vfim.s $rd:ident $([$($rdp:tt)+])?, $imm16:expr) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11011111000000000000000000000000",
            "| 0b0000000010000000",
            "| ((", stringify!($imm16), " & 0xFFFF) << 0)",
            "| (", $crate::register_single!($rd), " << 16)",
        )
    };

    // Waits until the write buffer has been flushed

    (vflush) => {
        ".word 0b11111111111111110000010000001101"
    };

    // Converts the input packed float16 into full 32 bit floating point numbers.

    (vh2f.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (0b0110011 << 16)",
        )
    };

    (vh2f.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (0b0110011 << 16)",
        )
    };

    // Performs vector floating point homegeneous dot product

    (vhdp.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01100110000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (", $crate::register_pair!($rt), " << 16)",
        )
    };

    (vhdp.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01100110000000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (", $crate::register_triple!($rt), " << 16)",
        )
    };

    (vhdp.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01100110000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (", $crate::register_quad!($rt), " << 16)",
        )
    };

    // Performs a vector-matrix homogeneous transform (matrix-vector product), with a vector result

    (vhtfm2.p $rd:ident, $rs:ident, $rt:ident) => {
        concat!(
            ".word 0b11110000100000000000000000000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_mpair!($rs), " << 8)",
            "| (", $crate::register_pair!($rt), " << 16)",
        )
    };

    // Performs a vector-matrix homogeneous transform (matrix-vector product), with a vector result

    (vhtfm3.t $rd:ident, $rs:ident, $rt:ident) => {
        concat!(
            ".word 0b11110001000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_mtriple!($rs), " << 8)",
            "| (", $crate::register_triple!($rt), " << 16)",
        )
    };

    // Performs a vector-matrix homogeneous transform (matrix-vector product), with a vector result

    (vhtfm4.q $rd:ident, $rs:ident, $rt:ident) => {
        concat!(
            ".word 0b11110001100000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_mquad!($rs), " << 8)",
            "| (", $crate::register_quad!($rt), " << 16)",
        )
    };

    // Converts the four integer inputs to char and packs them as a single element word. The conversion process takes the 8 most significant bits of each integer.

    (vi2c.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (0b0111101 << 16)",
        )
    };

    // Performs element-wise integer to float conversion with optional scaling factor. The integer is divided by 2^scale after the conversion.

    (vi2f.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $scale:expr) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010010100000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (", stringify!($scale), " << 16)",
        )
    };

    (vi2f.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $scale:expr) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010010100000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (", stringify!($scale), " << 16)",
        )
    };

    (vi2f.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $scale:expr) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010010100000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (", stringify!($scale), " << 16)",
        )
    };

    (vi2f.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $scale:expr) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010010100000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (", stringify!($scale), " << 16)",
        )
    };

    // Converts the integer inputs to short and packs them in pairs in the output register. The conversion process takes the 16 most significant bits of each integer.

    (vi2s.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (0b0111111 << 16)",
        )
    };

    (vi2s.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (0b0111111 << 16)",
        )
    };

    // Converts the four integer inputs to char and packs them as a single element word. The conversion process takes the 8 most significant bits of each integer and clamps any negative input values to zero.

    (vi2uc.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (0b0111100 << 16)",
        )
    };

    // Converts the integer inputs to short and packs them in pairs in the output register. The conversion process takes the 16 most significant bits of each integer and clamps any negative input values to zero.

    (vi2us.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (0b0111110 << 16)",
        )
    };

    (vi2us.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (0b0111110 << 16)",
        )
    };

    // Initializes destination register as an identity matrix row (all zeros but one). The behaviour depends on the destination register number.

    (vidt.p $rd:ident $([$($rdp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (0b0000011 << 16)",
        )
    };

    (vidt.q $rd:ident $([$($rdp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (0b0000011 << 16)",
        )
    };

    // Loads a signed 16 bit immediate value (converted to floating point) in a register

    (viim.s $rd:ident $([$($rdp:tt)+])?, $imm16:expr) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11011111000000000000000000000000",
            "| ((", stringify!($imm16), " & 0xFFFF) << 0)",
            "| (", $crate::register_single!($rd), " << 16)",
        )
    };

    // Performs element-wise logB() calculation

    (vlgb.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (0b0110111 << 16)",
        )
    };

    // Performs element-wise floating point log2(rs) operation

    (vlog2.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (0b0010101 << 16)",
        )
    };

    (vlog2.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (0b0010101 << 16)",
        )
    };

    (vlog2.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (0b0010101 << 16)",
        )
    };

    (vlog2.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (0b0010101 << 16)",
        )
    };

    // Performs element-wise floating point max(rs, rt) operation

    (vmax.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01101101100000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (", $crate::register_single!($rt), " << 16)",
        )
    };

    (vmax.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01101101100000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (", $crate::register_pair!($rt), " << 16)",
        )
    };

    (vmax.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01101101100000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (", $crate::register_triple!($rt), " << 16)",
        )
    };

    (vmax.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01101101100000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (", $crate::register_quad!($rt), " << 16)",
        )
    };

    // Writes the identity matrix into the destination register

    (vmidt.p $rd:ident) => {
        concat!(
            ".word 0b11110011100000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_mpair!($rd), " << 0)",
            "| (0b0000011 << 16)",
        )
    };

    (vmidt.t $rd:ident) => {
        concat!(
            ".word 0b11110011100000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_mtriple!($rd), " << 0)",
            "| (0b0000011 << 16)",
        )
    };

    (vmidt.q $rd:ident) => {
        concat!(
            ".word 0b11110011100000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_mquad!($rd), " << 0)",
            "| (0b0000011 << 16)",
        )
    };

    // Performs element-wise floating point min(rs, rt) operation

    (vmin.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01101101000000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (", $crate::register_single!($rt), " << 16)",
        )
    };

    (vmin.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01101101000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (", $crate::register_pair!($rt), " << 16)",
        )
    };

    (vmin.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01101101000000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (", $crate::register_triple!($rt), " << 16)",
        )
    };

    (vmin.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01101101000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (", $crate::register_quad!($rt), " << 16)",
        )
    };

    // Element-wise data copy

    (vmmov.p $rd:ident, $rs:ident) => {
        concat!(
            ".word 0b11110011100000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_mpair!($rd), " << 0)",
            "| (", $crate::register_mpair!($rs), " << 8)",
        )
    };

    (vmmov.t $rd:ident, $rs:ident) => {
        concat!(
            ".word 0b11110011100000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_mtriple!($rd), " << 0)",
            "| (", $crate::register_mtriple!($rs), " << 8)",
        )
    };

    (vmmov.q $rd:ident, $rs:ident) => {
        concat!(
            ".word 0b11110011100000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_mquad!($rd), " << 0)",
            "| (", $crate::register_mquad!($rs), " << 8)",
        )
    };

    // Performs a matrix multiplication

    (vmmul.p $rd:ident, $rs:ident, $rt:ident) => {
        concat!(
            ".word 0b11110000000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_mpair!($rd), " << 0)",
            "| ((", $crate::register_mpair!($rs), " ^ 0b0100000) << 8)",
            "| (", $crate::register_mpair!($rt), " << 16)",
        )
    };

    (vmmul.t $rd:ident, $rs:ident, $rt:ident) => {
        concat!(
            ".word 0b11110000000000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_mtriple!($rd), " << 0)",
            "| ((", $crate::register_mtriple!($rs), " ^ 0b0100000) << 8)",
            "| (", $crate::register_mtriple!($rt), " << 16)",
        )
    };

    (vmmul.q $rd:ident, $rs:ident, $rt:ident) => {
        concat!(
            ".word 0b11110000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_mquad!($rd), " << 0)",
            "| ((", $crate::register_mquad!($rs), " ^ 0b0100000) << 8)",
            "| (", $crate::register_mquad!($rt), " << 16)",
        )
    };

    // Overwrites all elements in a matrix with ones (1.0f)

    (vmone.p $rd:ident) => {
        concat!(
            ".word 0b11110011100000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_mpair!($rd), " << 0)",
            "| (0b0000111 << 16)",
        )
    };

    (vmone.t $rd:ident) => {
        concat!(
            ".word 0b11110011100000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_mtriple!($rd), " << 0)",
            "| (0b0000111 << 16)",
        )
    };

    (vmone.q $rd:ident) => {
        concat!(
            ".word 0b11110011100000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_mquad!($rd), " << 0)",
            "| (0b0000111 << 16)",
        )
    };

    // Element-wise data copy

    (vmov.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
        )
    };

    (vmov.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
        )
    };

    (vmov.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
        )
    };

    (vmov.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
        )
    };

    // Performs a matrix scaling by a single factor

    (vmscl.p $rd:ident, $rs:ident, $rt:ident) => {
        concat!(
            ".word 0b11110010000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_mpair!($rd), " << 0)",
            "| (", $crate::register_mpair!($rs), " << 8)",
            "| (", $crate::register_single!($rt), " << 16)",
        )
    };

    (vmscl.t $rd:ident, $rs:ident, $rt:ident) => {
        concat!(
            ".word 0b11110010000000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_mtriple!($rd), " << 0)",
            "| (", $crate::register_mtriple!($rs), " << 8)",
            "| (", $crate::register_single!($rt), " << 16)",
        )
    };

    (vmscl.q $rd:ident, $rs:ident, $rt:ident) => {
        concat!(
            ".word 0b11110010000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_mquad!($rd), " << 0)",
            "| (", $crate::register_mquad!($rs), " << 8)",
            "| (", $crate::register_single!($rt), " << 16)",
        )
    };

    // Performs element-wise floating point multiplication

    (vmul.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01100100000000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (", $crate::register_single!($rt), " << 16)",
        )
    };

    (vmul.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01100100000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (", $crate::register_pair!($rt), " << 16)",
        )
    };

    (vmul.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01100100000000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (", $crate::register_triple!($rt), " << 16)",
        )
    };

    (vmul.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01100100000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (", $crate::register_quad!($rt), " << 16)",
        )
    };

    // Writes a zero matrix into the destination register

    (vmzero.p $rd:ident) => {
        concat!(
            ".word 0b11110011100000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_mpair!($rd), " << 0)",
            "| (0b0000110 << 16)",
        )
    };

    (vmzero.t $rd:ident) => {
        concat!(
            ".word 0b11110011100000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_mtriple!($rd), " << 0)",
            "| (0b0000110 << 16)",
        )
    };

    (vmzero.q $rd:ident) => {
        concat!(
            ".word 0b11110011100000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_mquad!($rd), " << 0)",
            "| (0b0000110 << 16)",
        )
    };

    // Performs element-wise floating point negation

    (vneg.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (0b0000010 << 16)",
        )
    };

    (vneg.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (0b0000010 << 16)",
        )
    };

    (vneg.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (0b0000010 << 16)",
        )
    };

    (vneg.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (0b0000010 << 16)",
        )
    };

    // Does nothing and wastes one VFPU cycle. Used to avoid pipeline hazards. This instruction does consume prefixes.

    (vnop) => {
        ".word 0b11111111111111110000000000000000"
    };

    // Performs element-wise floating point negated reciprocal

    (vnrcp.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (0b0011000 << 16)",
        )
    };

    (vnrcp.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (0b0011000 << 16)",
        )
    };

    (vnrcp.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (0b0011000 << 16)",
        )
    };

    (vnrcp.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (0b0011000 << 16)",
        )
    };

    // Performs element-wise floating point -sin(π/2⋅rs) operation

    (vnsin.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (0b0011010 << 16)",
        )
    };

    (vnsin.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (0b0011010 << 16)",
        )
    };

    (vnsin.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (0b0011010 << 16)",
        )
    };

    (vnsin.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (0b0011010 << 16)",
        )
    };

    // Performs element-wise one's complement (1.0f - x)

    (vocp.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (0b1000100 << 16)",
        )
    };

    (vocp.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (0b1000100 << 16)",
        )
    };

    (vocp.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (0b1000100 << 16)",
        )
    };

    (vocp.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (0b1000100 << 16)",
        )
    };

    // Writes ones (1.0f) into the destination register

    (vone.s $rd:ident $([$($rdp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (0b0000111 << 16)",
        )
    };

    (vone.p $rd:ident $([$($rdp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (0b0000111 << 16)",
        )
    };

    (vone.t $rd:ident $([$($rdp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (0b0000111 << 16)",
        )
    };

    (vone.q $rd:ident $([$($rdp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (0b0000111 << 16)",
        )
    };

    // Performs a vector-matrix homogeneous transform (matrix-vector product), with a vector result

    (vqmul.q $rd:ident, $rs:ident, $rt:ident) => {
        concat!(
            ".word 0b11110010100000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (", $crate::register_quad!($rt), " << 16)",
        )
    };

    // Performs element-wise floating point reciprocal

    (vrcp.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (0b0010000 << 16)",
        )
    };

    (vrcp.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (0b0010000 << 16)",
        )
    };

    (vrcp.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (0b0010000 << 16)",
        )
    };

    (vrcp.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (0b0010000 << 16)",
        )
    };

    // Performs element-wise floating point 1/exp2(rs) operation (equivalent to exp2(-rs))

    (vrexp2.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (0b0011100 << 16)",
        )
    };

    (vrexp2.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (0b0011100 << 16)",
        )
    };

    (vrexp2.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (0b0011100 << 16)",
        )
    };

    (vrexp2.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (0b0011100 << 16)",
        )
    };

    // Writes pseudorandom numbers to the destination elements so that each element (x) can assert 1.0f <= x < 2.0f

    (vrndf1.s $rd:ident $([$($rdp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (0b0100010 << 16)",
        )
    };

    (vrndf1.p $rd:ident $([$($rdp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (0b0100010 << 16)",
        )
    };

    (vrndf1.t $rd:ident $([$($rdp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (0b0100010 << 16)",
        )
    };

    (vrndf1.q $rd:ident $([$($rdp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (0b0100010 << 16)",
        )
    };

    // Writes pseudorandom numbers to the destination elements so that each element (x) can assert 2.0f <= x < 4.0f

    (vrndf2.s $rd:ident $([$($rdp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (0b0100011 << 16)",
        )
    };

    (vrndf2.p $rd:ident $([$($rdp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (0b0100011 << 16)",
        )
    };

    (vrndf2.t $rd:ident $([$($rdp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (0b0100011 << 16)",
        )
    };

    (vrndf2.q $rd:ident $([$($rdp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (0b0100011 << 16)",
        )
    };

    // Writes pseudorandom 32 bit numbers to the destination elements (full 32bit range)

    (vrndi.s $rd:ident $([$($rdp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (0b0100001 << 16)",
        )
    };

    (vrndi.p $rd:ident $([$($rdp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (0b0100001 << 16)",
        )
    };

    (vrndi.t $rd:ident $([$($rdp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (0b0100001 << 16)",
        )
    };

    (vrndi.q $rd:ident $([$($rdp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (0b0100001 << 16)",
        )
    };

    // Uses the integer value as a seed for the pseudorandom number generator.

    (vrnds.s $rs:ident) => {
        concat!(
            ".word 0b11010000000000000000000000000000",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (0b0100000 << 16)",
        )
    };

    // Calculates a rotation matrix row, given an angle argument

    (vrot.p $rd:ident, $rs:ident, [$($imm5:tt)*]) => {
        concat!(
            ".word 0b11110011101000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (", $crate::vrot_immediate_pair!($($imm5)*), " << 16)",
        )
    };

    (vrot.t $rd:ident, $rs:ident, [$($imm5:tt)*]) => {
        concat!(
            ".word 0b11110011101000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (", $crate::vrot_immediate_triple!($($imm5)*), " << 16)",
        )
    };

    (vrot.q $rd:ident, $rs:ident, [$($imm5:tt)*]) => {
        concat!(
            ".word 0b11110011101000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (", $crate::vrot_immediate_quad!($($imm5)*), " << 16)",
        )
    };

    // Performs element-wise floating pointreciprocal square root

    (vrsq.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (0b0010001 << 16)",
        )
    };

    (vrsq.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (0b0010001 << 16)",
        )
    };

    (vrsq.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (0b0010001 << 16)",
        )
    };

    (vrsq.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (0b0010001 << 16)",
        )
    };

    // Converts the input packed shorts into full 32 bit integers in the output register. The input is placed on the most significant bits of the output integer, while the least significant bits are filled with zeros.

    (vs2i.s $rd:ident $([$($rdp:tt)+])?, $rs:ident) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (0b0111011 << 16)",
        )
    };

    (vs2i.p $rd:ident $([$($rdp:tt)+])?, $rs:ident) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (0b0111011 << 16)",
        )
    };

    // Saturates inputs to the [0.0f ... 1.0f] range

    (vsat0.s $rd:ident $([$($rdp:tt)+])?, $rs:ident) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (0b0000100 << 16)",
        )
    };

    (vsat0.p $rd:ident $([$($rdp:tt)+])?, $rs:ident) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (0b0000100 << 16)",
        )
    };

    (vsat0.t $rd:ident $([$($rdp:tt)+])?, $rs:ident) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (0b0000100 << 16)",
        )
    };

    (vsat0.q $rd:ident $([$($rdp:tt)+])?, $rs:ident) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (0b0000100 << 16)",
        )
    };

    // Saturates inputs to the [-1.0f ... 1.0f] range

    (vsat1.s $rd:ident $([$($rdp:tt)+])?, $rs:ident) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (0b0000101 << 16)",
        )
    };

    (vsat1.p $rd:ident $([$($rdp:tt)+])?, $rs:ident) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (0b0000101 << 16)",
        )
    };

    (vsat1.t $rd:ident $([$($rdp:tt)+])?, $rs:ident) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (0b0000101 << 16)",
        )
    };

    (vsat1.q $rd:ident $([$($rdp:tt)+])?, $rs:ident) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (0b0000101 << 16)",
        )
    };

    // Rescales rs operand to have rt as exponent. This would be equivalent to ldexp(frexp(rs, NULL), rt + 128). If we express the number in its IEEE754 terms, that is, if rs can be expressed as ±m * 2^e, the instruction will replace "e" with the value of rt + 127 mod 256.

    (vsbn.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01100001000000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (", $crate::register_single!($rt), " << 16)",
        )
    };

    // Rescales rs operand to have zero as exponent, so that it is reduced to the [1.0, 2.0) interval. This is essentially equivalent to the vsbn instruction with rt=0.

    (vsbz.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (0b0110110 << 16)",
        )
    };

    // Scales a vector (element-wise) by an scalar factor

    (vscl.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b01100101000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (", $crate::register_single!($rt), " << 16)",
        )
    };

    (vscl.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b01100101000000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (", $crate::register_single!($rt), " << 16)",
        )
    };

    (vscl.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b01100101000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (", $crate::register_single!($rt), " << 16)",
        )
    };

    // Performs element-wise floating point comparison. The result is -1.0f, 0.0f or 1.0f depending on whether the input vs is less that vt, equal, or greater, respectively.

    (vscmp.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01101110100000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (", $crate::register_single!($rt), " << 16)",
        )
    };

    (vscmp.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01101110100000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (", $crate::register_pair!($rt), " << 16)",
        )
    };

    (vscmp.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01101110100000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (", $crate::register_triple!($rt), " << 16)",
        )
    };

    (vscmp.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01101110100000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (", $crate::register_quad!($rt), " << 16)",
        )
    };

    // Performs element-wise floating point bigger-or-equal comparison. The result will be 1.0 if vs is bigger or equal to vt, otherwise will be zero.

    (vsge.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01101111000000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (", $crate::register_single!($rt), " << 16)",
        )
    };

    (vsge.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01101111000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (", $crate::register_pair!($rt), " << 16)",
        )
    };

    (vsge.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01101111000000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (", $crate::register_triple!($rt), " << 16)",
        )
    };

    (vsge.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01101111000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (", $crate::register_quad!($rt), " << 16)",
        )
    };

    // Performs element-wise floating point sign(rs) operation. This function returns -1, 0 or 1 depending on whether the input is negative zero or positive respectively.

    (vsgn.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (0b1001010 << 16)",
        )
    };

    (vsgn.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (0b1001010 << 16)",
        )
    };

    (vsgn.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (0b1001010 << 16)",
        )
    };

    (vsgn.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (0b1001010 << 16)",
        )
    };

    // Performs element-wise floating point sin(π/2⋅rs) operation

    (vsin.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (0b0010010 << 16)",
        )
    };

    (vsin.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (0b0010010 << 16)",
        )
    };

    (vsin.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (0b0010010 << 16)",
        )
    };

    (vsin.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (0b0010010 << 16)",
        )
    };

    // Performs element-wise floating point less-than comparison. The result will be 1.0 if vs less than vt, otherwise will be zero.

    (vslt.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01101111100000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (", $crate::register_single!($rt), " << 16)",
        )
    };

    (vslt.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01101111100000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (", $crate::register_pair!($rt), " << 16)",
        )
    };

    (vslt.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01101111100000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (", $crate::register_triple!($rt), " << 16)",
        )
    };

    (vslt.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01101111100000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (", $crate::register_quad!($rt), " << 16)",
        )
    };

    // Performs element-wise one's complement (1.0f - x) with saturation to [0.0f ... 1.0f]

    (vsocp.s $rd:ident, $rs:ident) => {
        concat!(
            ".word 0b11010000000000000000000000000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (0b1000101 << 16)",
        )
    };

    (vsocp.p $rd:ident, $rs:ident) => {
        concat!(
            ".word 0b11010000000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (0b1000101 << 16)",
        )
    };

    // Performs element-wise floating point aproximate square root

    (vsqrt.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (0b0010110 << 16)",
        )
    };

    (vsqrt.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (0b0010110 << 16)",
        )
    };

    (vsqrt.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (0b0010110 << 16)",
        )
    };

    (vsqrt.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (0b0010110 << 16)",
        )
    };

    // Performs a min() sorting step between elements pairs 0-1 and 2-3, shuffling them depending on their values.

    (vsrt1.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (0b1000000 << 16)",
        )
    };

    // Performs a min() sorting step between elements pairs 3-0 and 1-2, shuffling them depending on their values.

    (vsrt2.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (0b1000001 << 16)",
        )
    };

    // Performs a max() sorting step between elements pairs 0-1 and 2-3, shuffling them depending on their values.

    (vsrt3.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (0b1001000 << 16)",
        )
    };

    // Performs a max() sorting step between elements pairs 3-0 and 1-2, shuffling them depending on their values.

    (vsrt4.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (0b1001001 << 16)",
        )
    };

    // Performs element-wise floating point subtraction

    (vsub.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01100000100000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (", $crate::register_single!($rt), " << 16)",
        )
    };

    (vsub.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01100000100000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (", $crate::register_pair!($rt), " << 16)",
        )
    };

    (vsub.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01100000100000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_triple!($rs), " << 8)",
            "| (", $crate::register_triple!($rt), " << 16)",
        )
    };

    (vsub.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
            ".word 0b01100000100000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (", $crate::register_quad!($rt), " << 16)",
        )
    };

    // Waits until all operations in the VFPU pipeline have completed

    (vsync) => {
        ".word 0b11111111111111110000001100100000"
    };

    // Converts four ABGR8888 color points to ABGR4444. The output 16 bit values are packed into a vector register pair.

    (vt4444.q $rd:ident, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (0b1011001 << 16)",
        )
    };

    // Converts four ABGR8888 color points to ABGR1555. The output 16 bit values are packed into a vector register pair.

    (vt5551.q $rd:ident, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (0b1011010 << 16)",
        )
    };

    // Converts four ABGR8888 color points to BGR565. The output 16 bit values are packed into a vector register pair.

    (vt5650.q $rd:ident, $rs:ident $([$($rsp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_quad!($rs), " << 8)",
            "| (0b1011011 << 16)",
        )
    };

    // Performs a vector-matrix transform (matrix-vector product), with a vector result

    (vtfm2.p $rd:ident, $rs:ident, $rt:ident) => {
        concat!(
            ".word 0b11110000100000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_mpair!($rs), " << 8)",
            "| (", $crate::register_pair!($rt), " << 16)",
        )
    };

    // Performs a vector-matrix transform (matrix-vector product), with a vector result

    (vtfm3.t $rd:ident, $rs:ident, $rt:ident) => {
        concat!(
            ".word 0b11110001000000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (", $crate::register_mtriple!($rs), " << 8)",
            "| (", $crate::register_triple!($rt), " << 16)",
        )
    };

    // Performs a vector-matrix transform (matrix-vector product), with a vector result

    (vtfm4.q $rd:ident, $rs:ident, $rt:ident) => {
        concat!(
            ".word 0b11110001100000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_mquad!($rs), " << 8)",
            "| (", $crate::register_quad!($rt), " << 16)",
        )
    };

    // Converts the input packed chars into full 32 bit integers in the output register. The input is placed on the most significant bits of the output integer, while the least significant bits are filled with zeros  XXXXXs.

    (vuc2ifs.s $rd:ident $([$($rdp:tt)+])?, $rs:ident) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (0b0111000 << 16)",
        )
    };

    // Converts the input packed shorts into full 32 bit integers in the output register. The input is placed on the most significant bits of the output integer, while the least significant bits are filled with zeros.

    (vus2i.s $rd:ident $([$($rdp:tt)+])?, $rs:ident) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (", $crate::register_single!($rs), " << 8)",
            "| (0b0111010 << 16)",
        )
    };

    (vus2i.p $rd:ident $([$($rdp:tt)+])?, $rs:ident) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (", $crate::register_pair!($rs), " << 8)",
            "| (0b0111010 << 16)",
        )
    };

    // Writes zeros (0.0f) into the destination register

    (vzero.s $rd:ident $([$($rdp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| (", $crate::register_single!($rd), " << 0)",
            "| (0b0000110 << 16)",
        )
    };

    (vzero.p $rd:ident $([$($rdp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b0000000010000000",
            "| (", $crate::register_pair!($rd), " << 0)",
            "| (0b0000110 << 16)",
        )
    };

    (vzero.t $rd:ident $([$($rdp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000000000000",
            "| (", $crate::register_triple!($rd), " << 0)",
            "| (0b0000110 << 16)",
        )
    };

    (vzero.q $rd:ident $([$($rdp:tt)+])?) => {
        concat!(
            $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
            ".word 0b11010000000000000000000000000000",
            "| 0b1000000010000000",
            "| (", $crate::register_quad!($rd), " << 0)",
            "| (0b0000110 << 16)",
        )
    };

    // Regular ASM fallback.
    ($($t:tt)*) => {
        $crate::stringify_asm!($($t)*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! vpfx_instr {
    ($callback:tt (stack: ) (cur: $($c:tt)+)) => {
        $crate::instruction!(
            $callback
            [$($c)+]
        )
    };

    ($callback:tt (stack: $( ( $($t:tt)+ ) )*) (cur: $($($c:tt)+)?)) => {
        $crate::instruction!(
            $callback
            $([ $($t)+ ]),*
            $(, [$($c)+] )?
        )
    };

    // When encountering a comma, dump the current symbol list onto the stack.
    ($callback:tt (stack: $($s:tt)*) (cur: $($c:tt)*) , $($t2:tt)*) => {
        $crate::vpfx_instr!($callback (stack: $($s)* ($($c)*)) (cur: ) $($t2)*)
    };

    // Add a token to the running list.
    ($callback:tt $stack:tt (cur: $($t:tt)*) $token:tt $($t2:tt)*) => {
        $crate::vpfx_instr!($callback $stack (cur: $($t)* $token) $($t2)*)
    };
}

/// Try to stringify a register name to its number.
///
/// Falls back to just re-emitting the same token stringified.
#[macro_export]
#[doc(hidden)]
macro_rules! try_stringify_mips_reg {
    (zero) => {"$0"};
    (at) => {"$1"};

    (v0) => {"$2"};
    (v1) => {"$3"};

    (a0) => {"$4"};
    (a1) => {"$5"};
    (a2) => {"$6"};
    (a3) => {"$7"};

    (t0) => {"$8"};
    (t1) => {"$9"};
    (t2) => {"$10"};
    (t3) => {"$11"};
    (t4) => {"$12"};
    (t5) => {"$13"};
    (t6) => {"$14"};
    (t7) => {"$15"};

    (s0) => {"$16"};
    (s1) => {"$17"};
    (s2) => {"$18"};
    (s3) => {"$19"};
    (s4) => {"$20"};
    (s5) => {"$21"};
    (s6) => {"$22"};
    (s7) => {"$23"};

    (t8) => {"$24"};
    (t9) => {"$25"};

    (k0) => {"$26"};
    (k1) => {"$27"};

    (gp) => {"$28"};
    (sp) => {"$29"};
    (fp) => {"$30"};
    (ra) => {"$31"};

    (f0) => {"$0"};
    (f1) => {"$1"};
    (f2) => {"$2"};
    (f3) => {"$3"};
    (f4) => {"$4"};
    (f5) => {"$5"};
    (f6) => {"$6"};
    (f7) => {"$7"};
    (f8) => {"$8"};
    (f9) => {"$9"};
    (f10) => {"$10"};
    (f11) => {"$11"};
    (f12) => {"$12"};
    (f13) => {"$13"};
    (f14) => {"$14"};
    (f15) => {"$15"};
    (f16) => {"$16"};
    (f17) => {"$17"};
    (f18) => {"$18"};
    (f19) => {"$19"};
    (f20) => {"$20"};
    (f21) => {"$21"};
    (f22) => {"$22"};
    (f23) => {"$23"};
    (f24) => {"$24"};
    (f25) => {"$25"};
    (f26) => {"$26"};
    (f27) => {"$27"};
    (f28) => {"$28"};
    (f29) => {"$29"};
    (f30) => {"$30"};
    (f31) => {"$31"};

    // Fallthrough case, in case this is e.g. an assembler format literal or
    // something else.
    ($($t:tt)+) => {stringify!($($t)+)};
}

// The registers here were obtained by disassembling test programs built with
// the C psp toolchain.
#[doc(hidden)]
#[macro_export]
macro_rules! register_single {
    // Smcr = 0brrmmmcc
    (S000) => {
        "0b0000000"
    };
    (S001) => {
        "0b0100000"
    };
    (S002) => {
        "0b1000000"
    };
    (S003) => {
        "0b1100000"
    };
    (S010) => {
        "0b0000001"
    };
    (S011) => {
        "0b0100001"
    };
    (S012) => {
        "0b1000001"
    };
    (S013) => {
        "0b1100001"
    };
    (S020) => {
        "0b0000010"
    };
    (S021) => {
        "0b0100010"
    };
    (S022) => {
        "0b1000010"
    };
    (S023) => {
        "0b1100010"
    };
    (S030) => {
        "0b0000011"
    };
    (S031) => {
        "0b0100011"
    };
    (S032) => {
        "0b1000011"
    };
    (S033) => {
        "0b1100011"
    };

    (S100) => {
        "0b0000100"
    };
    (S101) => {
        "0b0100100"
    };
    (S102) => {
        "0b1000100"
    };
    (S103) => {
        "0b1100100"
    };
    (S110) => {
        "0b0000101"
    };
    (S111) => {
        "0b0100101"
    };
    (S112) => {
        "0b1000101"
    };
    (S113) => {
        "0b1100101"
    };
    (S120) => {
        "0b0000110"
    };
    (S121) => {
        "0b0100110"
    };
    (S122) => {
        "0b1000110"
    };
    (S123) => {
        "0b1100110"
    };
    (S130) => {
        "0b0000111"
    };
    (S131) => {
        "0b0100111"
    };
    (S132) => {
        "0b1000111"
    };
    (S133) => {
        "0b1100111"
    };

    (S200) => {
        "0b0001000"
    };
    (S201) => {
        "0b0101000"
    };
    (S202) => {
        "0b1001000"
    };
    (S203) => {
        "0b1101000"
    };
    (S210) => {
        "0b0001001"
    };
    (S211) => {
        "0b0101001"
    };
    (S212) => {
        "0b1001001"
    };
    (S213) => {
        "0b1101001"
    };
    (S220) => {
        "0b0001010"
    };
    (S221) => {
        "0b0101010"
    };
    (S222) => {
        "0b1001010"
    };
    (S223) => {
        "0b1101010"
    };
    (S230) => {
        "0b0001011"
    };
    (S231) => {
        "0b0101011"
    };
    (S232) => {
        "0b1001011"
    };
    (S233) => {
        "0b1101011"
    };

    (S300) => {
        "0b0001100"
    };
    (S301) => {
        "0b0101100"
    };
    (S302) => {
        "0b1001100"
    };
    (S303) => {
        "0b1101100"
    };
    (S310) => {
        "0b0001101"
    };
    (S311) => {
        "0b0101101"
    };
    (S312) => {
        "0b1001101"
    };
    (S313) => {
        "0b1101101"
    };
    (S320) => {
        "0b0001110"
    };
    (S321) => {
        "0b0101110"
    };
    (S322) => {
        "0b1001110"
    };
    (S323) => {
        "0b1101110"
    };
    (S330) => {
        "0b0001111"
    };
    (S331) => {
        "0b0101111"
    };
    (S332) => {
        "0b1001111"
    };
    (S333) => {
        "0b1101111"
    };

    (S400) => {
        "0b0010000"
    };
    (S401) => {
        "0b0110000"
    };
    (S402) => {
        "0b1010000"
    };
    (S403) => {
        "0b1110000"
    };
    (S410) => {
        "0b0010001"
    };
    (S411) => {
        "0b0110001"
    };
    (S412) => {
        "0b1010001"
    };
    (S413) => {
        "0b1110001"
    };
    (S420) => {
        "0b0010010"
    };
    (S421) => {
        "0b0110010"
    };
    (S422) => {
        "0b1010010"
    };
    (S423) => {
        "0b1110010"
    };
    (S430) => {
        "0b0010011"
    };
    (S431) => {
        "0b0110011"
    };
    (S432) => {
        "0b1010011"
    };
    (S433) => {
        "0b1110011"
    };

    (S500) => {
        "0b0010100"
    };
    (S501) => {
        "0b0110100"
    };
    (S502) => {
        "0b1010100"
    };
    (S503) => {
        "0b1110100"
    };
    (S510) => {
        "0b0010101"
    };
    (S511) => {
        "0b0110101"
    };
    (S512) => {
        "0b1010101"
    };
    (S513) => {
        "0b1110101"
    };
    (S520) => {
        "0b0010110"
    };
    (S521) => {
        "0b0110110"
    };
    (S522) => {
        "0b1010110"
    };
    (S523) => {
        "0b1110110"
    };
    (S530) => {
        "0b0010111"
    };
    (S531) => {
        "0b0110111"
    };
    (S532) => {
        "0b1010111"
    };
    (S533) => {
        "0b1110111"
    };

    (S600) => {
        "0b0011000"
    };
    (S601) => {
        "0b0111000"
    };
    (S602) => {
        "0b1011000"
    };
    (S603) => {
        "0b1111000"
    };
    (S610) => {
        "0b0011001"
    };
    (S611) => {
        "0b0111001"
    };
    (S612) => {
        "0b1011001"
    };
    (S613) => {
        "0b1111001"
    };
    (S620) => {
        "0b0011010"
    };
    (S621) => {
        "0b0111010"
    };
    (S622) => {
        "0b1011010"
    };
    (S623) => {
        "0b1111010"
    };
    (S630) => {
        "0b0011011"
    };
    (S631) => {
        "0b0111011"
    };
    (S632) => {
        "0b1011011"
    };
    (S633) => {
        "0b1111011"
    };

    (S700) => {
        "0b0011100"
    };
    (S701) => {
        "0b0111100"
    };
    (S702) => {
        "0b1011100"
    };
    (S703) => {
        "0b1111100"
    };
    (S710) => {
        "0b0011101"
    };
    (S711) => {
        "0b0111101"
    };
    (S712) => {
        "0b1011101"
    };
    (S713) => {
        "0b1111101"
    };
    (S720) => {
        "0b0011110"
    };
    (S721) => {
        "0b0111110"
    };
    (S722) => {
        "0b1011110"
    };
    (S723) => {
        "0b1111110"
    };
    (S730) => {
        "0b0011111"
    };
    (S731) => {
        "0b0111111"
    };
    (S732) => {
        "0b1011111"
    };
    (S733) => {
        "0b1111111"
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! register_pair {
    (C000) => {
        "0b0000000"
    };
    (C002) => {
        "0b1000000"
    };
    (C010) => {
        "0b0000001"
    };
    (C012) => {
        "0b1000001"
    };
    (C020) => {
        "0b0000010"
    };
    (C022) => {
        "0b1000010"
    };
    (C030) => {
        "0b0000011"
    };
    (C032) => {
        "0b1000011"
    };
    (C100) => {
        "0b0000100"
    };
    (C102) => {
        "0b1000100"
    };
    (C110) => {
        "0b0000101"
    };
    (C112) => {
        "0b1000101"
    };
    (C120) => {
        "0b0000110"
    };
    (C122) => {
        "0b1000110"
    };
    (C130) => {
        "0b0000111"
    };
    (C132) => {
        "0b1000111"
    };
    (C200) => {
        "0b0001000"
    };
    (C202) => {
        "0b1001000"
    };
    (C210) => {
        "0b0001001"
    };
    (C212) => {
        "0b1001001"
    };
    (C220) => {
        "0b0001010"
    };
    (C222) => {
        "0b1001010"
    };
    (C230) => {
        "0b0001011"
    };
    (C232) => {
        "0b1001011"
    };
    (C300) => {
        "0b0001100"
    };
    (C302) => {
        "0b1001100"
    };
    (C310) => {
        "0b0001101"
    };
    (C312) => {
        "0b1001101"
    };
    (C320) => {
        "0b0001110"
    };
    (C322) => {
        "0b1001110"
    };
    (C330) => {
        "0b0001111"
    };
    (C332) => {
        "0b1001111"
    };
    (C400) => {
        "0b0010000"
    };
    (C402) => {
        "0b1010000"
    };
    (C410) => {
        "0b0010001"
    };
    (C412) => {
        "0b1010001"
    };
    (C420) => {
        "0b0010010"
    };
    (C422) => {
        "0b1010010"
    };
    (C430) => {
        "0b0010011"
    };
    (C432) => {
        "0b1010011"
    };
    (C500) => {
        "0b0010100"
    };
    (C502) => {
        "0b1010100"
    };
    (C510) => {
        "0b0010101"
    };
    (C512) => {
        "0b1010101"
    };
    (C520) => {
        "0b0010110"
    };
    (C522) => {
        "0b1010110"
    };
    (C530) => {
        "0b0010111"
    };
    (C532) => {
        "0b1010111"
    };
    (C600) => {
        "0b0011000"
    };
    (C602) => {
        "0b1011000"
    };
    (C610) => {
        "0b0011001"
    };
    (C612) => {
        "0b1011001"
    };
    (C620) => {
        "0b0011010"
    };
    (C622) => {
        "0b1011010"
    };
    (C630) => {
        "0b0011011"
    };
    (C632) => {
        "0b1011011"
    };
    (C700) => {
        "0b0011100"
    };
    (C702) => {
        "0b1011100"
    };
    (C710) => {
        "0b0011101"
    };
    (C712) => {
        "0b1011101"
    };
    (C720) => {
        "0b0011110"
    };
    (C722) => {
        "0b1011110"
    };
    (C730) => {
        "0b0011111"
    };
    (C732) => {
        "0b1011111"
    };

    (R000) => {
        "0b0100000"
    };
    (R001) => {
        "0b0100001"
    };
    (R002) => {
        "0b0100010"
    };
    (R003) => {
        "0b0100011"
    };
    (R020) => {
        "0b1100000"
    };
    (R021) => {
        "0b1100001"
    };
    (R022) => {
        "0b1100010"
    };
    (R023) => {
        "0b1100011"
    };
    (R100) => {
        "0b0100100"
    };
    (R101) => {
        "0b0100101"
    };
    (R102) => {
        "0b0100110"
    };
    (R103) => {
        "0b0100111"
    };
    (R120) => {
        "0b1100100"
    };
    (R121) => {
        "0b1100101"
    };
    (R122) => {
        "0b1100110"
    };
    (R123) => {
        "0b1100111"
    };
    (R200) => {
        "0b0101000"
    };
    (R201) => {
        "0b0101001"
    };
    (R202) => {
        "0b0101010"
    };
    (R203) => {
        "0b0101011"
    };
    (R220) => {
        "0b1101000"
    };
    (R221) => {
        "0b1101001"
    };
    (R222) => {
        "0b1101010"
    };
    (R223) => {
        "0b1101011"
    };
    (R300) => {
        "0b0101100"
    };
    (R301) => {
        "0b0101101"
    };
    (R302) => {
        "0b0101110"
    };
    (R303) => {
        "0b0101111"
    };
    (R320) => {
        "0b1101100"
    };
    (R321) => {
        "0b1101101"
    };
    (R322) => {
        "0b1101110"
    };
    (R323) => {
        "0b1101111"
    };
    (R400) => {
        "0b0110000"
    };
    (R401) => {
        "0b0110001"
    };
    (R402) => {
        "0b0110010"
    };
    (R403) => {
        "0b0110011"
    };
    (R420) => {
        "0b1110000"
    };
    (R421) => {
        "0b1110001"
    };
    (R422) => {
        "0b1110010"
    };
    (R423) => {
        "0b1110011"
    };
    (R500) => {
        "0b0110100"
    };
    (R501) => {
        "0b0110101"
    };
    (R502) => {
        "0b0110110"
    };
    (R503) => {
        "0b0110111"
    };
    (R520) => {
        "0b1110100"
    };
    (R521) => {
        "0b1110101"
    };
    (R522) => {
        "0b1110110"
    };
    (R523) => {
        "0b1110111"
    };
    (R600) => {
        "0b0111000"
    };
    (R601) => {
        "0b0111001"
    };
    (R602) => {
        "0b0111010"
    };
    (R603) => {
        "0b0111011"
    };
    (R620) => {
        "0b1111000"
    };
    (R621) => {
        "0b1111001"
    };
    (R622) => {
        "0b1111010"
    };
    (R623) => {
        "0b1111011"
    };
    (R700) => {
        "0b0111100"
    };
    (R701) => {
        "0b0111101"
    };
    (R702) => {
        "0b0111110"
    };
    (R703) => {
        "0b0111111"
    };
    (R720) => {
        "0b1111100"
    };
    (R721) => {
        "0b1111101"
    };
    (R722) => {
        "0b1111110"
    };
    (R723) => {
        "0b1111111"
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! register_triple {
    (C000) => {
        "0b0000000"
    };
    (C001) => {
        "0b1000000"
    };
    (C010) => {
        "0b0000001"
    };
    (C011) => {
        "0b1000001"
    };
    (C020) => {
        "0b0000010"
    };
    (C021) => {
        "0b1000010"
    };
    (C030) => {
        "0b0000011"
    };
    (C031) => {
        "0b1000011"
    };
    (C100) => {
        "0b0000100"
    };
    (C101) => {
        "0b1000100"
    };
    (C110) => {
        "0b0000101"
    };
    (C111) => {
        "0b1000101"
    };
    (C120) => {
        "0b0000110"
    };
    (C121) => {
        "0b1000110"
    };
    (C130) => {
        "0b0000111"
    };
    (C131) => {
        "0b1000111"
    };
    (C200) => {
        "0b0001000"
    };
    (C201) => {
        "0b1001000"
    };
    (C210) => {
        "0b0001001"
    };
    (C211) => {
        "0b1001001"
    };
    (C220) => {
        "0b0001010"
    };
    (C221) => {
        "0b1001010"
    };
    (C230) => {
        "0b0001011"
    };
    (C231) => {
        "0b1001011"
    };
    (C300) => {
        "0b0001100"
    };
    (C301) => {
        "0b1001100"
    };
    (C310) => {
        "0b0001101"
    };
    (C311) => {
        "0b1001101"
    };
    (C320) => {
        "0b0001110"
    };
    (C321) => {
        "0b1001110"
    };
    (C330) => {
        "0b0001111"
    };
    (C331) => {
        "0b1001111"
    };
    (C400) => {
        "0b0010000"
    };
    (C401) => {
        "0b1010000"
    };
    (C410) => {
        "0b0010001"
    };
    (C411) => {
        "0b1010001"
    };
    (C420) => {
        "0b0010010"
    };
    (C421) => {
        "0b1010010"
    };
    (C430) => {
        "0b0010011"
    };
    (C431) => {
        "0b1010011"
    };
    (C500) => {
        "0b0010100"
    };
    (C501) => {
        "0b1010100"
    };
    (C510) => {
        "0b0010101"
    };
    (C511) => {
        "0b1010101"
    };
    (C520) => {
        "0b0010110"
    };
    (C521) => {
        "0b1010110"
    };
    (C530) => {
        "0b0010111"
    };
    (C531) => {
        "0b1010111"
    };
    (C600) => {
        "0b0011000"
    };
    (C601) => {
        "0b1011000"
    };
    (C610) => {
        "0b0011001"
    };
    (C611) => {
        "0b1011001"
    };
    (C620) => {
        "0b0011010"
    };
    (C621) => {
        "0b1011010"
    };
    (C630) => {
        "0b0011011"
    };
    (C631) => {
        "0b1011011"
    };
    (C700) => {
        "0b0011100"
    };
    (C701) => {
        "0b1011100"
    };
    (C710) => {
        "0b0011101"
    };
    (C711) => {
        "0b1011101"
    };
    (C720) => {
        "0b0011110"
    };
    (C721) => {
        "0b1011110"
    };
    (C730) => {
        "0b0011111"
    };
    (C731) => {
        "0b1011111"
    };

    (R000) => {
        "0b0100000"
    };
    (R001) => {
        "0b0100001"
    };
    (R002) => {
        "0b0100010"
    };
    (R003) => {
        "0b0100011"
    };
    (R010) => {
        "0b1100000"
    };
    (R011) => {
        "0b1100001"
    };
    (R012) => {
        "0b1100010"
    };
    (R013) => {
        "0b1100011"
    };
    (R100) => {
        "0b0100100"
    };
    (R101) => {
        "0b0100101"
    };
    (R102) => {
        "0b0100110"
    };
    (R103) => {
        "0b0100111"
    };
    (R110) => {
        "0b1100100"
    };
    (R111) => {
        "0b1100101"
    };
    (R112) => {
        "0b1100110"
    };
    (R113) => {
        "0b1100111"
    };
    (R200) => {
        "0b0101000"
    };
    (R201) => {
        "0b0101001"
    };
    (R202) => {
        "0b0101010"
    };
    (R203) => {
        "0b0101011"
    };
    (R210) => {
        "0b1101000"
    };
    (R211) => {
        "0b1101001"
    };
    (R212) => {
        "0b1101010"
    };
    (R213) => {
        "0b1101011"
    };
    (R300) => {
        "0b0101100"
    };
    (R301) => {
        "0b0101101"
    };
    (R302) => {
        "0b0101110"
    };
    (R303) => {
        "0b0101111"
    };
    (R310) => {
        "0b1101100"
    };
    (R311) => {
        "0b1101101"
    };
    (R312) => {
        "0b1101110"
    };
    (R313) => {
        "0b1101111"
    };
    (R400) => {
        "0b0110000"
    };
    (R401) => {
        "0b0110001"
    };
    (R402) => {
        "0b0110010"
    };
    (R403) => {
        "0b0110011"
    };
    (R410) => {
        "0b1110000"
    };
    (R411) => {
        "0b1110001"
    };
    (R412) => {
        "0b1110010"
    };
    (R413) => {
        "0b1110011"
    };
    (R500) => {
        "0b0110100"
    };
    (R501) => {
        "0b0110101"
    };
    (R502) => {
        "0b0110110"
    };
    (R503) => {
        "0b0110111"
    };
    (R510) => {
        "0b1110100"
    };
    (R511) => {
        "0b1110101"
    };
    (R512) => {
        "0b1110110"
    };
    (R513) => {
        "0b1110111"
    };
    (R600) => {
        "0b0111000"
    };
    (R601) => {
        "0b0111001"
    };
    (R602) => {
        "0b0111010"
    };
    (R603) => {
        "0b0111011"
    };
    (R610) => {
        "0b1111000"
    };
    (R611) => {
        "0b1111001"
    };
    (R612) => {
        "0b1111010"
    };
    (R613) => {
        "0b1111011"
    };
    (R700) => {
        "0b0111100"
    };
    (R701) => {
        "0b0111101"
    };
    (R702) => {
        "0b0111110"
    };
    (R703) => {
        "0b0111111"
    };
    (R710) => {
        "0b1111100"
    };
    (R711) => {
        "0b1111101"
    };
    (R712) => {
        "0b1111110"
    };
    (R713) => {
        "0b1111111"
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! register_quad {
    (C000) => {
        "0b0000000"
    };
    (C010) => {
        "0b0000001"
    };
    (C020) => {
        "0b0000010"
    };
    (C030) => {
        "0b0000011"
    };
    (C100) => {
        "0b0000100"
    };
    (C110) => {
        "0b0000101"
    };
    (C120) => {
        "0b0000110"
    };
    (C130) => {
        "0b0000111"
    };
    (C200) => {
        "0b0001000"
    };
    (C210) => {
        "0b0001001"
    };
    (C220) => {
        "0b0001010"
    };
    (C230) => {
        "0b0001011"
    };
    (C300) => {
        "0b0001100"
    };
    (C310) => {
        "0b0001101"
    };
    (C320) => {
        "0b0001110"
    };
    (C330) => {
        "0b0001111"
    };
    (C400) => {
        "0b0010000"
    };
    (C410) => {
        "0b0010001"
    };
    (C420) => {
        "0b0010010"
    };
    (C430) => {
        "0b0010011"
    };
    (C500) => {
        "0b0010100"
    };
    (C510) => {
        "0b0010101"
    };
    (C520) => {
        "0b0010110"
    };
    (C530) => {
        "0b0010111"
    };
    (C600) => {
        "0b0011000"
    };
    (C610) => {
        "0b0011001"
    };
    (C620) => {
        "0b0011010"
    };
    (C630) => {
        "0b0011011"
    };
    (C700) => {
        "0b0011100"
    };
    (C710) => {
        "0b0011101"
    };
    (C720) => {
        "0b0011110"
    };
    (C730) => {
        "0b0011111"
    };

    (R000) => {
        "0b0100000"
    };
    (R001) => {
        "0b0100001"
    };
    (R002) => {
        "0b0100010"
    };
    (R003) => {
        "0b0100011"
    };
    (R100) => {
        "0b0100100"
    };
    (R101) => {
        "0b0100101"
    };
    (R102) => {
        "0b0100110"
    };
    (R103) => {
        "0b0100111"
    };
    (R200) => {
        "0b0101000"
    };
    (R201) => {
        "0b0101001"
    };
    (R202) => {
        "0b0101010"
    };
    (R203) => {
        "0b0101011"
    };
    (R300) => {
        "0b0101100"
    };
    (R301) => {
        "0b0101101"
    };
    (R302) => {
        "0b0101110"
    };
    (R303) => {
        "0b0101111"
    };
    (R400) => {
        "0b0110000"
    };
    (R401) => {
        "0b0110001"
    };
    (R402) => {
        "0b0110010"
    };
    (R403) => {
        "0b0110011"
    };
    (R500) => {
        "0b0110100"
    };
    (R501) => {
        "0b0110101"
    };
    (R502) => {
        "0b0110110"
    };
    (R503) => {
        "0b0110111"
    };
    (R600) => {
        "0b0111000"
    };
    (R601) => {
        "0b0111001"
    };
    (R602) => {
        "0b0111010"
    };
    (R603) => {
        "0b0111011"
    };
    (R700) => {
        "0b0111100"
    };
    (R701) => {
        "0b0111101"
    };
    (R702) => {
        "0b0111110"
    };
    (R703) => {
        "0b0111111"
    };
}

/// Matrix variant of `register_pair!`
#[doc(hidden)]
#[macro_export]
macro_rules! register_mpair {
    (M000) => {
        "0b0000000"
    };
    (M002) => {
        "0b1000000"
    };
    (M020) => {
        "0b0000010"
    };
    (M022) => {
        "0b1000010"
    };
    (M100) => {
        "0b0000100"
    };
    (M102) => {
        "0b1000100"
    };
    (M120) => {
        "0b0000110"
    };
    (M122) => {
        "0b1000110"
    };
    (M200) => {
        "0b0001000"
    };
    (M202) => {
        "0b1001000"
    };
    (M220) => {
        "0b0001010"
    };
    (M222) => {
        "0b1001010"
    };
    (M300) => {
        "0b0001100"
    };
    (M302) => {
        "0b1001100"
    };
    (M320) => {
        "0b0001110"
    };
    (M322) => {
        "0b1001110"
    };
    (M400) => {
        "0b0010000"
    };
    (M402) => {
        "0b1010000"
    };
    (M420) => {
        "0b0010010"
    };
    (M422) => {
        "0b1010010"
    };
    (M500) => {
        "0b0010100"
    };
    (M502) => {
        "0b1010100"
    };
    (M520) => {
        "0b0010110"
    };
    (M522) => {
        "0b1010110"
    };
    (M600) => {
        "0b0011000"
    };
    (M602) => {
        "0b1011000"
    };
    (M620) => {
        "0b0011010"
    };
    (M622) => {
        "0b1011010"
    };
    (M700) => {
        "0b0011100"
    };
    (M702) => {
        "0b1011100"
    };
    (M720) => {
        "0b0011110"
    };
    (M722) => {
        "0b1011110"
    };

    (E000) => {
        "0b0100000"
    };
    (E002) => {
        "0b0100010"
    };
    (E020) => {
        "0b1100000"
    };
    (E022) => {
        "0b1100010"
    };
    (E100) => {
        "0b0100100"
    };
    (E102) => {
        "0b0100110"
    };
    (E120) => {
        "0b1100100"
    };
    (E122) => {
        "0b1100110"
    };
    (E200) => {
        "0b0101000"
    };
    (E202) => {
        "0b0101010"
    };
    (E220) => {
        "0b1101000"
    };
    (E222) => {
        "0b1101010"
    };
    (E300) => {
        "0b0101100"
    };
    (E302) => {
        "0b0101110"
    };
    (E320) => {
        "0b1101100"
    };
    (E322) => {
        "0b1101110"
    };
    (E400) => {
        "0b0110000"
    };
    (E402) => {
        "0b0110010"
    };
    (E420) => {
        "0b1110000"
    };
    (E422) => {
        "0b1110010"
    };
    (E500) => {
        "0b0110100"
    };
    (E502) => {
        "0b0110110"
    };
    (E520) => {
        "0b1110100"
    };
    (E522) => {
        "0b1110110"
    };
    (E600) => {
        "0b0111000"
    };
    (E602) => {
        "0b0111010"
    };
    (E620) => {
        "0b1111000"
    };
    (E622) => {
        "0b1111010"
    };
    (E700) => {
        "0b0111100"
    };
    (E702) => {
        "0b0111110"
    };
    (E720) => {
        "0b1111100"
    };
    (E722) => {
        "0b1111110"
    };
}

/// Matrix variant of `register_triple!`
#[doc(hidden)]
#[macro_export]
macro_rules! register_mtriple {
    (M000) => {
        "0b0000000"
    };
    (M001) => {
        "0b1000000"
    };
    (M010) => {
        "0b0000001"
    };
    (M011) => {
        "0b1000001"
    };
    (M100) => {
        "0b0000100"
    };
    (M101) => {
        "0b1000100"
    };
    (M110) => {
        "0b0000101"
    };
    (M111) => {
        "0b1000101"
    };
    (M200) => {
        "0b0001000"
    };
    (M201) => {
        "0b1001000"
    };
    (M210) => {
        "0b0001001"
    };
    (M211) => {
        "0b1001001"
    };
    (M300) => {
        "0b0001100"
    };
    (M301) => {
        "0b1001100"
    };
    (M310) => {
        "0b0001101"
    };
    (M311) => {
        "0b1001101"
    };
    (M400) => {
        "0b0010000"
    };
    (M401) => {
        "0b1010000"
    };
    (M410) => {
        "0b0010001"
    };
    (M411) => {
        "0b1010001"
    };
    (M500) => {
        "0b0010100"
    };
    (M501) => {
        "0b1010100"
    };
    (M510) => {
        "0b0010101"
    };
    (M511) => {
        "0b1010101"
    };
    (M600) => {
        "0b0011000"
    };
    (M601) => {
        "0b1011000"
    };
    (M610) => {
        "0b0011001"
    };
    (M611) => {
        "0b1011001"
    };
    (M700) => {
        "0b0011100"
    };
    (M701) => {
        "0b1011100"
    };
    (M710) => {
        "0b0011101"
    };
    (M711) => {
        "0b1011101"
    };

    (E000) => {
        "0b0100000"
    };
    (E001) => {
        "0b0100001"
    };
    (E010) => {
        "0b1100000"
    };
    (E011) => {
        "0b1100001"
    };
    (E100) => {
        "0b0100100"
    };
    (E101) => {
        "0b0100101"
    };
    (E110) => {
        "0b1100100"
    };
    (E111) => {
        "0b1100101"
    };
    (E200) => {
        "0b0101000"
    };
    (E201) => {
        "0b0101001"
    };
    (E210) => {
        "0b1101000"
    };
    (E211) => {
        "0b1101001"
    };
    (E300) => {
        "0b0101100"
    };
    (E301) => {
        "0b0101101"
    };
    (E310) => {
        "0b1101100"
    };
    (E311) => {
        "0b1101101"
    };
    (E400) => {
        "0b0110000"
    };
    (E401) => {
        "0b0110001"
    };
    (E410) => {
        "0b1110000"
    };
    (E411) => {
        "0b1110001"
    };
    (E500) => {
        "0b0110100"
    };
    (E501) => {
        "0b0110101"
    };
    (E510) => {
        "0b1110100"
    };
    (E511) => {
        "0b1110101"
    };
    (E600) => {
        "0b0111000"
    };
    (E601) => {
        "0b0111001"
    };
    (E610) => {
        "0b1111000"
    };
    (E611) => {
        "0b1111001"
    };
    (E700) => {
        "0b0111100"
    };
    (E701) => {
        "0b0111101"
    };
    (E710) => {
        "0b1111100"
    };
    (E711) => {
        "0b1111101"
    };
}

/// Matrix variant of `register_quad!`
#[doc(hidden)]
#[macro_export]
macro_rules! register_mquad {
    (M000) => {
        "0b0000000"
    };
    (M100) => {
        "0b0000100"
    };
    (M200) => {
        "0b0001000"
    };
    (M300) => {
        "0b0001100"
    };
    (M400) => {
        "0b0010000"
    };
    (M500) => {
        "0b0010100"
    };
    (M600) => {
        "0b0011000"
    };
    (M700) => {
        "0b0011100"
    };

    (E000) => {
        "0b0100000"
    };
    (E100) => {
        "0b0100100"
    };
    (E200) => {
        "0b0101000"
    };
    (E300) => {
        "0b0101100"
    };
    (E400) => {
        "0b0110000"
    };
    (E500) => {
        "0b0110100"
    };
    (E600) => {
        "0b0111000"
    };
    (E700) => {
        "0b0111100"
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! vfpu_const {
    (VFPU_HUGE) => {
        "1"
    };
    (VFPU_SQRT2) => {
        "2"
    };
    (VFPU_SQRT1_2) => {
        "3"
    };
    (VFPU_2_SQRTPI) => {
        "4"
    };
    (VFPU_2_PI) => {
        "5"
    };
    (VFPU_1_PI) => {
        "6"
    };
    (VFPU_PI_4) => {
        "7"
    };
    (VFPU_PI_2) => {
        "8"
    };
    (VFPU_PI) => {
        "9"
    };
    (VFPU_E) => {
        "10"
    };
    (VFPU_LOG2E) => {
        "11"
    };
    (VFPU_LOG10E) => {
        "12"
    };
    (VFPU_LN2) => {
        "13"
    };
    (VFPU_LN10) => {
        "14"
    };
    (VFPU_2PI) => {
        "15"
    };
    (VFPU_PI_6) => {
        "16"
    };
    (VFPU_LOG10TWO) => {
        "17"
    };
    (VFPU_LOG2TEN) => {
        "18"
    };
    (VFPU_SQRT3_2) => {
        "19"
    };
    (VFPU_SQRT4_3) => {
        "20"
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! instruction_prefix {
    (X) => {"0"}; (Y) => {"1"}; (Z) => {"2"}; (W) => {"3"};

    (|X|) => {"0x00100"};
    (|Y|) => {"0x00101"};
    (|Z|) => {"0x00102"};
    (|W|) => {"0x00103"};

    (0) => {"0x01000"};
    (1) => {"0x01001"};
    (2) => {"0x01002"};
    (1/2) => {"0x01003"};
    (3) => {"0x01100"};
    (1/3) => {"0x01101"};
    (1/4) => {"0x01102"};
    (1/6) => {"0x01103"};

    (- $($tt:tt)*) => {
        concat!(
            "(0x10000 | ", $crate::instruction_prefix!($($tt)*), ")",
        )
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! instruction_prefix_d {
    () => {
        "0"
    };
    (0) => {
        "1"
    };
    (1) => {
        "3"
    };
    (M) => {
        "0x100"
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! vrot_immediate_pair {
    // There are duplicates here, but this is fine. They come from the quad
    // variant of this macro. Any resulting immediate number should do.
    (C, S) => {
        "0"
    };
    (S, C) => {
        "1"
    };
    (S, 0) => {
        "2"
    };
    (S, 0) => {
        "3"
    };
    (C, S) => {
        "4"
    };
    (S, C) => {
        "5"
    };
    (0, S) => {
        "6"
    };
    (0, S) => {
        "7"
    };
    (C, 0) => {
        "8"
    };
    (0, C) => {
        "9"
    };
    (S, S) => {
        "10"
    };
    (0, 0) => {
        "11"
    };
    (C, 0) => {
        "12"
    };
    (0, C) => {
        "13"
    };
    (0, 0) => {
        "14"
    };
    (S, S) => {
        "15"
    };
    (C, -S) => {
        "16"
    };
    (-S, C) => {
        "17"
    };
    (-S, 0) => {
        "18"
    };
    (-S, 0) => {
        "19"
    };
    (C, -S) => {
        "20"
    };
    (-S, C) => {
        "21"
    };
    (0, -S) => {
        "22"
    };
    (0, -S) => {
        "23"
    };
    (C, 0) => {
        "24"
    };
    (0, C) => {
        "25"
    };
    (-S, -S) => {
        "26"
    };
    (0, 0) => {
        "27"
    };
    (C, 0) => {
        "28"
    };
    (0, C) => {
        "29"
    };
    (0, 0) => {
        "30"
    };
    (-S, -S) => {
        "31"
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! vrot_immediate_triple {
    // Duplicates, like `vrot_immediate_pair!`, are fine.
    (C, S, S) => {
        "0"
    };
    (S, C, 0) => {
        "1"
    };
    (S, 0, C) => {
        "2"
    };
    (S, 0, 0) => {
        "3"
    };
    (C, S, 0) => {
        "4"
    };
    (S, C, S) => {
        "5"
    };
    (0, S, C) => {
        "6"
    };
    (0, S, 0) => {
        "7"
    };
    (C, 0, S) => {
        "8"
    };
    (0, C, S) => {
        "9"
    };
    (S, S, C) => {
        "10"
    };
    (0, 0, S) => {
        "11"
    };
    (C, 0, 0) => {
        "12"
    };
    (0, C, 0) => {
        "13"
    };
    (0, 0, C) => {
        "14"
    };
    (S, S, S) => {
        "15"
    };
    (C, -S, -S) => {
        "16"
    };
    (-S, C, 0) => {
        "17"
    };
    (-S, 0, C) => {
        "18"
    };
    (-S, 0, 0) => {
        "19"
    };
    (C, -S, 0) => {
        "20"
    };
    (-S, C, -S) => {
        "21"
    };
    (0, -S, C) => {
        "22"
    };
    (0, -S, 0) => {
        "23"
    };
    (C, 0, -S) => {
        "24"
    };
    (0, C, -S) => {
        "25"
    };
    (-S, -S, C) => {
        "26"
    };
    (0, 0, -S) => {
        "27"
    };
    (C, 0, 0) => {
        "28"
    };
    (0, C, 0) => {
        "29"
    };
    (0, 0, C) => {
        "30"
    };
    (-S, -S, -S) => {
        "31"
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! vrot_immediate_quad {
    (C, S, S, S) => {
        "0"
    };
    (S, C, 0, 0) => {
        "1"
    };
    (S, 0, C, 0) => {
        "2"
    };
    (S, 0, 0, C) => {
        "3"
    };
    (C, S, 0, 0) => {
        "4"
    };
    (S, C, S, S) => {
        "5"
    };
    (0, S, C, 0) => {
        "6"
    };
    (0, S, 0, C) => {
        "7"
    };
    (C, 0, S, 0) => {
        "8"
    };
    (0, C, S, 0) => {
        "9"
    };
    (S, S, C, S) => {
        "10"
    };
    (0, 0, S, C) => {
        "11"
    };
    (C, 0, 0, S) => {
        "12"
    };
    (0, C, 0, S) => {
        "13"
    };
    (0, 0, C, S) => {
        "14"
    };
    (S, S, S, C) => {
        "15"
    };
    (C, -S, -S, -S) => {
        "16"
    };
    (-S, C, 0, 0) => {
        "17"
    };
    (-S, 0, C, 0) => {
        "18"
    };
    (-S, 0, 0, C) => {
        "19"
    };
    (C, -S, 0, 0) => {
        "20"
    };
    (-S, C, -S, -S) => {
        "21"
    };
    (0, -S, C, 0) => {
        "22"
    };
    (0, -S, 0, C) => {
        "23"
    };
    (C, 0, -S, 0) => {
        "24"
    };
    (0, C, -S, 0) => {
        "25"
    };
    (-S, -S, C, -S) => {
        "26"
    };
    (0, 0, -S, C) => {
        "27"
    };
    (C, 0, 0, -S) => {
        "28"
    };
    (0, C, 0, -S) => {
        "29"
    };
    (0, 0, C, -S) => {
        "30"
    };
    (-S, -S, -S, C) => {
        "31"
    };
}
