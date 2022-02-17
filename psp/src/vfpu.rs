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

    // vadd.s 0110 0000 0 ttttttt 0 sssssss 0 ddddddd
    (vadd.s $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte ", $crate::register_single!($t),
            "\n.byte 0b01100000",
        )
    };

    // vadd.p 0110 0000 0 ttttttt 0 sssssss 1 ddddddd
    (vadd.p $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte ", $crate::register_pair!($t),
            "\n.byte 0b01100000",
        )
    };

    // vadd.t 0110 0000 0 ttttttt 1 sssssss 0 ddddddd
    (vadd.t $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte ", $crate::register_triple!($t),
            "\n.byte 0b01100000",
        )
    };

    // vadd.q 0110 0000 0 ttttttt 1 sssssss 1 ddddddd
    (vadd.q $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte ", $crate::register_quad!($t),
            "\n.byte 0b01100000",
        )
    };

    // vsub.s 0110 0000 1 ttttttt 0 sssssss 0 ddddddd
    (vsub.s $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0x80 | ", $crate::register_single!($t),
            "\n.byte 0b01100000",
        )
    };

    // vsub.p 0110 0000 1 ttttttt 0 sssssss 1 ddddddd
    (vsub.p $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0x80 | ", $crate::register_pair!($t),
            "\n.byte 0b01100000",
        )
    };

    // vsub.t 0110 0000 1 ttttttt 1 sssssss 0 ddddddd
    (vsub.t $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0x80 | ", $crate::register_triple!($t),
            "\n.byte 0b01100000",
        )
    };

    // vsub.q 0110 0000 1 ttttttt 1 sssssss 1 ddddddd
    (vsub.q $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0x80 | ", $crate::register_quad!($t),
            "\n.byte 0b01100000",
        )
    };

    // vdiv.s 0110 0011 1 ttttttt 0 sssssss 0 ddddddd
    (vdiv.s $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0x80 | ", $crate::register_single!($t),
            "\n.byte 0b01100011",
        )
    };

    // vdiv.p 0110 0011 1 ttttttt 0 sssssss 1 ddddddd
    (vdiv.p $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0x80 | ", $crate::register_pair!($t),
            "\n.byte 0b01100011",
        )
    };

    // vdiv.t 0110 0011 1 ttttttt 1 sssssss 0 ddddddd
    (vdiv.t $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0x80 | ", $crate::register_triple!($t),
            "\n.byte 0b01100011",
        )
    };

    // vdiv.q 0110 0011 1 ttttttt 1 sssssss 1 ddddddd
    (vdiv.q $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0x80 | ", $crate::register_quad!($t),
            "\n.byte 0b01100011",
        )
    };

    // vmul.s 0110 0100 0 ttttttt 0 sssssss 0 ddddddd
    (vmul.s $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte ", $crate::register_single!($t),
            "\n.byte 0b01100100",
        )
    };

    // vmul.p 0110 0100 0 ttttttt 0 sssssss 1 ddddddd
    (vmul.p $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte ", $crate::register_pair!($t),
            "\n.byte 0b01100100",
        )
    };

    // vmul.t 0110 0100 0 ttttttt 1 sssssss 0 ddddddd
    (vmul.t $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte ", $crate::register_triple!($t),
            "\n.byte 0b01100100",
        )
    };

    // vmul.q 0110 0100 0 ttttttt 1 sssssss 1 ddddddd
    (vmul.q $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte ", $crate::register_quad!($t),
            "\n.byte 0b01100100",
        )
    };

    // vdot.p 0110 0100 1 ttttttt 0 sssssss 1 ddddddd
    (vdot.p $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0x80 | ", $crate::register_pair!($t),
            "\n.byte 0b01100100",
        )
    };

    // vdot.t 0110 0100 1 ttttttt 1 sssssss 0 ddddddd
    (vdot.t $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0x80 | ", $crate::register_triple!($t),
            "\n.byte 0b01100100",
        )
    };

    // vdot.q 0110 0100 1 ttttttt 1 sssssss 1 ddddddd
    (vdot.q $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0x80 | ", $crate::register_quad!($t),
            "\n.byte 0b01100100",
        )
    };

    // vhdp.p 0110 0110 0 ttttttt 0 sssssss 1 ddddddd
    (vhdp.p $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte ", $crate::register_pair!($t),
            "\n.byte 0b01100110",
        )
    };

    // vhdp.t 0110 0110 0 ttttttt 1 sssssss 0 ddddddd
    (vhdp.t $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte ", $crate::register_triple!($t),
            "\n.byte 0b01100110",
        )
    };

    // vhdp.q 0110 0110 0 ttttttt 1 sssssss 1 ddddddd
    (vhdp.q $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte ", $crate::register_quad!($t),
            "\n.byte 0b01100110",
        )
    };

    // vmin.s 0110 1101 0 ttttttt 0 sssssss 0 ddddddd
    (vmin.s $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte ", $crate::register_single!($t),
            "\n.byte 0b01101101",
        )
    };

    // vmin.p 0110 1101 0 ttttttt 0 sssssss 1 ddddddd
    (vmin.p $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte ", $crate::register_pair!($t),
            "\n.byte 0b01101101",
        )
    };

    // vmin.t 0110 1101 0 ttttttt 1 sssssss 0 ddddddd
    (vmin.t $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte ", $crate::register_triple!($t),
            "\n.byte 0b01101101",
        )
    };

    // vmin.q 0110 1101 0 ttttttt 1 sssssss 1 ddddddd
    (vmin.q $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte ", $crate::register_quad!($t),
            "\n.byte 0b01101101",
        )
    };

    // vmax.s 0110 1101 1 ttttttt 0 sssssss 0 ddddddd
    (vmax.s $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0x80 | ", $crate::register_single!($t),
            "\n.byte 0b01101101",
        )
    };

    // vmax.p 0110 1101 1 ttttttt 0 sssssss 1 ddddddd
    (vmax.p $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0x80 | ", $crate::register_pair!($t),
            "\n.byte 0b01101101",
        )
    };

    // vmax.t 0110 1101 1 ttttttt 1 sssssss 0 ddddddd
    (vmax.t $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0x80 | ", $crate::register_triple!($t),
            "\n.byte 0b01101101",
        )
    };

    // vmax.q 0110 1101 1 ttttttt 1 sssssss 1 ddddddd
    (vmax.q $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0x80 | ", $crate::register_quad!($t),
            "\n.byte 0b01101101",
        )
    };

    // vabs.s 1101 0000 0 0000001 0 sssssss 0 ddddddd
    (vabs.s $d:ident, $s:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0b00000001",
            "\n.byte 0b11010000",
        )
    };

    // vabs.p 1101 0000 0 0000001 0 sssssss 1 ddddddd
    (vabs.p $d:ident, $s:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0b00000001",
            "\n.byte 0b11010000",
        )
    };

    // vabs.t 1101 0000 0 0000001 1 sssssss 0 ddddddd
    (vabs.t $d:ident, $s:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0b00000001",
            "\n.byte 0b11010000",
        )
    };

    // vabs.q 1101 0000 0 0000001 1 sssssss 1 ddddddd
    (vabs.q $d:ident, $s:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0b00000001",
            "\n.byte 0b11010000",
        )
    };

    // vneg.s 1101 0000 0 0000010 0 sssssss 0 ddddddd
    (vneg.s $d:ident, $s:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0b00000010",
            "\n.byte 0b11010000",
        )
    };

    // vneg.p 1101 0000 0 0000010 0 sssssss 1 ddddddd
    (vneg.p $d:ident, $s:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0b00000010",
            "\n.byte 0b11010000",
        )
    };

    // vneg.t 1101 0000 0 0000010 1 sssssss 0 ddddddd
    (vneg.t $d:ident, $s:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0b00000010",
            "\n.byte 0b11010000",
        )
    };

    // vneg.q 1101 0000 0 0000010 1 sssssss 1 ddddddd
    (vneg.q $d:ident, $s:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0b00000010",
            "\n.byte 0b11010000",
        )
    };

    // vidt.p 1101 0000 0 0000011 0 0000000 1 ddddddd
    (vidt.p $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte 0b00000000",
            "\n.byte 0b00000011",
            "\n.byte 0b11010000",
        )
    };

    // vidt.t 1101 0000 0 0000011 1 0000000 0 ddddddd
    (vidt.t $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80",
            "\n.byte 0b00000011",
            "\n.byte 0b11010000",
        )
    };

    // vidt.q 1101 0000 0 0000011 1 0000000 1 ddddddd
    (vidt.q $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80",
            "\n.byte 0b00000011",
            "\n.byte 0b11010000",
        )
    };

    // vzero.s 1101 0000 0 0000110 0 0000000 0 ddddddd
    (vzero.s $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte 0b00000000",
            "\n.byte 0b00000110",
            "\n.byte 0b11010000",
        )
    };

    // vzero.p 1101 0000 0 0000110 0 0000000 1 ddddddd
    (vzero.p $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte 0b00000000",
            "\n.byte 0b00000110",
            "\n.byte 0b11010000",
        )
    };

    // vzero.t 1101 0000 0 0000110 1 0000000 0 ddddddd
    (vzero.t $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80",
            "\n.byte 0b00000110",
            "\n.byte 0b11010000",
        )
    };

    // vzero.q 1101 0000 0 0000110 1 0000000 1 ddddddd
    (vzero.q $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80",
            "\n.byte 0b00000110",
            "\n.byte 0b11010000",
        )
    };

    // vone.s 1101 0000 0 0000111 0 0000000 0 ddddddd
    (vone.s $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte 0b00000000",
            "\n.byte 0b00000111",
            "\n.byte 0b11010000",
        )
    };

    // vone.p 1101 0000 0 0000111 0 0000000 1 ddddddd
    (vone.p $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte 0b00000000",
            "\n.byte 0b00000111",
            "\n.byte 0b11010000",
        )
    };

    // vone.t 1101 0000 0 0000111 1 0000000 0 ddddddd
    (vone.t $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80",
            "\n.byte 0b00000111",
            "\n.byte 0b11010000",
        )
    };

    // vone.q 1101 0000 0 0000111 1 0000000 1 ddddddd
    (vone.q $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80",
            "\n.byte 0b00000111",
            "\n.byte 0b11010000",
        )
    };

    // vrcp.s 1101 0000 0 0010000 0 sssssss 0 ddddddd
    (vrcp.s $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0b00010000",
            "\n.byte 0b11010000",
        )
    };

    // vrcp.p 1101 0000 0 0010000 0 sssssss 1 ddddddd
    (vrcp.p $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0b00010000",
            "\n.byte 0b11010000",
        )
    };

    // vrcp.t 1101 0000 0 0010000 1 sssssss 0 ddddddd
    (vrcp.t $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0b00010000",
            "\n.byte 0b11010000",
        )
    };

    // vrcp.q 1101 0000 0 0010000 1 sssssss 1 ddddddd
    (vrcp.q $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0b00010000",
            "\n.byte 0b11010000",
        )
    };

    // vrsq.s 1101 0000 0 0010001 0 sssssss 0 ddddddd
    (vrsq.s $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0b00010001",
            "\n.byte 0b11010000",
        )
    };

    // vrsq.p 1101 0000 0 0010001 0 sssssss 1 ddddddd
    (vrsq.p $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0b00010001",
            "\n.byte 0b11010000",
        )
    };

    // vrsq.t 1101 0000 0 0010001 1 sssssss 0 ddddddd
    (vrsq.t $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0b00010001",
            "\n.byte 0b11010000",
        )
    };

    // vrsq.q 1101 0000 0 0010001 1 sssssss 1 ddddddd
    (vrsq.q $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0b00010001",
            "\n.byte 0b11010000",
        )
    };

    // vsin.s 1101 0000 0 0010010 0 sssssss 0 ddddddd
    (vsin.s $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0b00010010",
            "\n.byte 0b11010000",
        )
    };

    // vsin.p 1101 0000 0 0010010 0 sssssss 1 ddddddd
    (vsin.p $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0b00010010",
            "\n.byte 0b11010000",
        )
    };

    // vsin.t 1101 0000 0 0010010 1 sssssss 0 ddddddd
    (vsin.t $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0b00010010",
            "\n.byte 0b11010000",
        )
    };

    // vsin.q 1101 0000 0 0010010 1 sssssss 1 ddddddd
    (vsin.q $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0b00010010",
            "\n.byte 0b11010000",
        )
    };

    // vcos.s 1101 0000 0 0010011 0 sssssss 0 ddddddd
    (vcos.s $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0b00010011",
            "\n.byte 0b11010000",
        )
    };

    // vcos.p 1101 0000 0 0010011 0 sssssss 1 ddddddd
    (vcos.p $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0b00010011",
            "\n.byte 0b11010000",
        )
    };

    // vcos.t 1101 0000 0 0010011 1 sssssss 0 ddddddd
    (vcos.t $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0b00010011",
            "\n.byte 0b11010000",
        )
    };

    // vcos.q 1101 0000 0 0010011 1 sssssss 1 ddddddd
    (vcos.q $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0b00010011",
            "\n.byte 0b11010000",
        )
    };

    // vexp2.s 1101 0000 0 0010100 0 sssssss 0 ddddddd
    (vexp2.s $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0b00010100",
            "\n.byte 0b11010000",
        )
    };

    // vexp2.p 1101 0000 0 0010100 0 sssssss 1 ddddddd
    (vexp2.p $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0b00010100",
            "\n.byte 0b11010000",
        )
    };

    // vexp2.t 1101 0000 0 0010100 1 sssssss 0 ddddddd
    (vexp2.t $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0b00010100",
            "\n.byte 0b11010000",
        )
    };

    // vexp2.q 1101 0000 0 0010100 1 sssssss 1 ddddddd
    (vexp2.q $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0b00010100",
            "\n.byte 0b11010000",
        )
    };

    // vlog2.s 1101 0000 0 0010101 0 sssssss 0 ddddddd
    (vlog2.s $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0b00010101",
            "\n.byte 0b11010000",
        )
    };

    // vlog2.p 1101 0000 0 0010101 0 sssssss 1 ddddddd
    (vlog2.p $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0b00010101",
            "\n.byte 0b11010000",
        )
    };

    // vlog2.t 1101 0000 0 0010101 1 sssssss 0 ddddddd
    (vlog2.t $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0b00010101",
            "\n.byte 0b11010000",
        )
    };

    // vlog2.q 1101 0000 0 0010101 1 sssssss 1 ddddddd
    (vlog2.q $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0b00010101",
            "\n.byte 0b11010000",
        )
    };

    // vsqrt.s 1101 0000 0 0010110 0 sssssss 0 ddddddd
    (vsqrt.s $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0b00010110",
            "\n.byte 0b11010000",
        )
    };

    // vsqrt.p 1101 0000 0 0010110 0 sssssss 1 ddddddd
    (vsqrt.p $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0b00010110",
            "\n.byte 0b11010000",
        )
    };

    // vsqrt.t 1101 0000 0 0010110 1 sssssss 0 ddddddd
    (vsqrt.t $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0b00010110",
            "\n.byte 0b11010000",
        )
    };

    // vsqrt.q 1101 0000 0 0010110 1 sssssss 1 ddddddd
    (vsqrt.q $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0b00010110",
            "\n.byte 0b11010000",
        )
    };

    // vasin.s 1101 0000 0 0010111 0 sssssss 0 ddddddd
    (vasin.s $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0b00010111",
            "\n.byte 0b11010000",
        )
    };

    // vasin.p 1101 0000 0 0010111 0 sssssss 1 ddddddd
    (vasin.p $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0b00010111",
            "\n.byte 0b11010000",
        )
    };

    // vasin.t 1101 0000 0 0010111 1 sssssss 0 ddddddd
    (vasin.t $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0b00010111",
            "\n.byte 0b11010000",
        )
    };

    // vasin.q 1101 0000 0 0010111 1 sssssss 1 ddddddd
    (vasin.q $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0b00010111",
            "\n.byte 0b11010000",
        )
    };

    // vnrcp.s 1101 0000 0 0011000 0 sssssss 0 ddddddd
    (vnrcp.s $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0b00011000",
            "\n.byte 0b11010000",
        )
    };

    // vnrcp.p 1101 0000 0 0011000 0 sssssss 1 ddddddd
    (vnrcp.p $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0b00011000",
            "\n.byte 0b11010000",
        )
    };

    // vnrcp.t 1101 0000 0 0011000 1 sssssss 0 ddddddd
    (vnrcp.t $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0b00011000",
            "\n.byte 0b11010000",
        )
    };

    // vnrcp.q 1101 0000 0 0011000 1 sssssss 1 ddddddd
    (vnrcp.q $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0b00011000",
            "\n.byte 0b11010000",
        )
    };

    // vnsin.s 1101 0000 0 0011010 0 sssssss 0 ddddddd
    (vnsin.s $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0b00011010",
            "\n.byte 0b11010000",
        )
    };

    // vnsin.p 1101 0000 0 0011010 0 sssssss 1 ddddddd
    (vnsin.p $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0b00011010",
            "\n.byte 0b11010000",
        )
    };

    // vnsin.t 1101 0000 0 0011010 1 sssssss 0 ddddddd
    (vnsin.t $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0b00011010",
            "\n.byte 0b11010000",
        )
    };

    // vnsin.q 1101 0000 0 0011010 1 sssssss 1 ddddddd
    (vnsin.q $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0b00011010",
            "\n.byte 0b11010000",
        )
    };

    // vrexp2.s 1101 0000 0 0011100 0 sssssss 0 ddddddd
    (vrexp2.s $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0b00011100",
            "\n.byte 0b11010000",
        )
    };

    // vrexp2.p 1101 0000 0 0011100 0 sssssss 1 ddddddd
    (vrexp2.p $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0b00011100",
            "\n.byte 0b11010000",
        )
    };

    // vrexp2.t 1101 0000 0 0011100 1 sssssss 0 ddddddd
    (vrexp2.t $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0b00011100",
            "\n.byte 0b11010000",
        )
    };

    // vrexp2.q 1101 0000 0 0011100 1 sssssss 1 ddddddd
    (vrexp2.q $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0b00011100",
            "\n.byte 0b11010000",
        )
    };

    // vi2uc.q 1101 0000 0 0111100 1 sssssss 1 ddddddd
    (vi2uc.q $d:ident, $s:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0b00111100",
            "\n.byte 0b11010000",
        )
    };

    // vi2s.p 1101 0000 0 0111111 0 sssssss 1 ddddddd
    (vi2s.p $d:ident, $s:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0b00111111",
            "\n.byte 0b11010000",
        )
    };

    // vi2s.q 1101 0000 0 0111111 1 sssssss 1 ddddddd
    (vi2s.q $d:ident, $s:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0b00111111",
            "\n.byte 0b11010000",
        )
    };

    // vsgn.s 1101 0000 0 1001010 0 sssssss 0 ddddddd
    (vsgn.s $d:ident, $s:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0b01001010",
            "\n.byte 0b11010000",
        )
    };

    // vsgn.p 1101 0000 0 1001010 0 sssssss 1 ddddddd
    (vsgn.p $d:ident, $s:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0b01001010",
            "\n.byte 0b11010000",
        )
    };

    // vsgn.t 1101 0000 0 1001010 1 sssssss 0 ddddddd
    (vsgn.t $d:ident, $s:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0b01001010",
            "\n.byte 0b11010000",
        )
    };

    // vsgn.q 1101 0000 0 1001010 1 sssssss 1 ddddddd
    (vsgn.q $d:ident, $s:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0b01001010",
            "\n.byte 0b11010000",
        )
    };

    // vcst.s 1101 0000 0 11aaaaa 0 0000000 0 ddddddd
    (vcst.s $d:ident, $a:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte 0b00000000",
            "\n.byte 0b01100000 | ", $crate::vfpu_const!($a),
            "\n.byte 0b11010000",
        )
    };

    // vcst.p 1101 0000 0 11aaaaa 0 0000000 1 ddddddd
    (vcst.p $d:ident, $a:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte 0b00000000",
            "\n.byte 0b01100000 | ", $crate::vfpu_const!($a),
            "\n.byte 0b11010000",
        )
    };

    // vcst.t 1101 0000 0 11aaaaa 1 0000000 0 ddddddd
    (vcst.t $d:ident, $a:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80",
            "\n.byte 0b01100000 | ", $crate::vfpu_const!($a),
            "\n.byte 0b11010000",
        )
    };

    // vcst.q 1101 0000 0 11aaaaa 1 0000000 1 ddddddd
    (vcst.q $d:ident, $a:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80",
            "\n.byte 0b01100000 | ", $crate::vfpu_const!($a),
            "\n.byte 0b11010000",
        )
    };

    // Float to int, rounds to nearest
    // vf2in.s 1101 0010 0 SSSSSSS 0 sssssss 0 ddddddd
    (vf2in.s $d:ident, $s:ident, $scale:expr) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte ", stringify!($scale),
            "\n.byte 0b11010010",
        )
    };

    // vf2in.p 1101 0010 0 SSSSSSS 0 sssssss 1 ddddddd
    (vf2in.p $d:ident, $s:ident, $scale:expr) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte ", stringify!($scale),
            "\n.byte 0b11010010",
        )
    };

    // vf2in.t 1101 0010 0 SSSSSSS 1 sssssss 0 ddddddd
    (vf2in.t $d:ident, $s:ident, $scale:expr) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte ", stringify!($scale),
            "\n.byte 0b11010010",
        )
    };

    // vf2in.q 1101 0010 0 SSSSSSS 1 sssssss 1 ddddddd
    (vf2in.q $d:ident, $s:ident, $scale:expr) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte ", stringify!($scale),
            "\n.byte 0b11010010",
        )
    };

    // vi2f.s 1101 0010 1 SSSSSSS 0 sssssss 0 ddddddd
    (vi2f.s $d:ident, $s:ident, $scale:expr) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0x80 | ", stringify!($scale),
            "\n.byte 0b11010010",
        )
    };

    // vi2f.p 1101 0010 1 SSSSSSS 0 sssssss 1 ddddddd
    (vi2f.p $d:ident, $s:ident, $scale:expr) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0x80 | ", stringify!($scale),
            "\n.byte 0b11010010",
        )
    };

    // vi2f.t 1101 0010 1 SSSSSSS 1 sssssss 0 ddddddd
    (vi2f.t $d:ident, $s:ident, $scale:expr) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0x80 | ", stringify!($scale),
            "\n.byte 0b11010010",
        )
    };

    // vi2f.q 1101 0010 1 SSSSSSS 1 sssssss 1 ddddddd
    (vi2f.q $d:ident, $s:ident, $scale:expr) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0x80 | ", stringify!($scale),
            "\n.byte 0b11010010",
        )
    };

    // vmmul.p 1111 0000 0 ttttttt 0 sSsssss 1 ddddddd (*inverted 5th S bit)
    (vmmul.p $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_mpair!($d),
            "\n.byte ", $crate::register_mpair!($s), " ^ 0b0100000",
            "\n.byte ", $crate::register_mpair!($t),
            "\n.byte 0b11110000",
        )
    };

    // vmmul.t 1111 0000 0 ttttttt 1 sSsssss 0 ddddddd (*inverted 5th S bit)
    (vmmul.t $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_mtriple!($d),
            "\n.byte 0x80 | ", $crate::register_mtriple!($s), " ^ 0b0100000",
            "\n.byte ", $crate::register_mtriple!($t),
            "\n.byte 0b11110000",
        )
    };

    // vmmul.q 1111 0000 0 ttttttt 1 sSsssss 1 ddddddd (*inverted 5th S bit)
    (vmmul.q $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_mquad!($d),
            "\n.byte 0x80 | ", $crate::register_mquad!($s), " ^ 0b0100000",
            "\n.byte ", $crate::register_mquad!($t),
            "\n.byte 0b11110000",
        )
    };

    // vhtfm2.p 1111 0000 1 ttttttt 0 sssssss 0 ddddddd
    (vhtfm2.p $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_mpair!($s),
            "\n.byte 0x80 | ", $crate::register_pair!($t),
            "\n.byte 0b11110000",
        )
    };

    // vtfm2.p 1111 0000 1 ttttttt 0 sssssss 1 ddddddd
    (vtfm2.p $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_mpair!($s),
            "\n.byte 0x80 | ", $crate::register_pair!($t),
            "\n.byte 0b11110000",
        )
    };

    // vhtfm3.t 1111 0001 0 ttttttt 0 sssssss 1 ddddddd
    (vhtfm3.t $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_triple!($d),
            "\n.byte ", $crate::register_mtriple!($s),
            "\n.byte ", $crate::register_triple!($t),
            "\n.byte 0b11110001",
        )
    };

    // vtfm3.t 1111 0001 0 ttttttt 1 sssssss 0 ddddddd
    (vtfm3.t $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_mtriple!($s),
            "\n.byte ", $crate::register_triple!($t),
            "\n.byte 0b11110001",
        )
    };

    // vhtfm4.q 1111 0001 1 ttttttt 1 sssssss 0 ddddddd
    (vhtfm4.q $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_mquad!($s),
            "\n.byte 0x80 | ", $crate::register_quad!($t),
            "\n.byte 0b11110001",
        )
    };

    // vtfm4.q 1111 0001 1 ttttttt 1 sssssss 1 ddddddd
    (vtfm4.q $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_mquad!($s),
            "\n.byte 0x80 | ", $crate::register_quad!($t),
            "\n.byte 0b11110001",
        )
    };

    // vmidt.p 1111 0011 1 0000011 0 0000000 1 ddddddd
    (vmidt.p $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_mpair!($d),
            "\n.byte 0b00000000",
            "\n.byte 0b10000011",
            "\n.byte 0b11110011",
        )
    };

    // vmidt.t 1111 0011 1 0000011 1 0000000 0 ddddddd
    (vmidt.t $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_mtriple!($d),
            "\n.byte 0x80",
            "\n.byte 0b10000011",
            "\n.byte 0b11110011",
        )
    };

    // vmidt.q 1111 0011 1 0000011 1 0000000 1 ddddddd
    (vmidt.q $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_mquad!($d),
            "\n.byte 0x80",
            "\n.byte 0b10000011",
            "\n.byte 0b11110011",
        )
    };

    // vmzero.p 1111 0011 1 0000110 0 0000000 1 ddddddd
    (vmzero.p $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_mpair!($d),
            "\n.byte 0b00000000",
            "\n.byte 0b10000110",
            "\n.byte 0b11110011",
        )
    };

    // vmzero.t 1111 0011 1 0000110 1 0000000 0 ddddddd
    (vmzero.t $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_mtriple!($d),
            "\n.byte 0x80",
            "\n.byte 0b10000110",
            "\n.byte 0b11110011",
        )
    };

    // vmzero.q 1111 0011 1 0000110 1 0000000 1 ddddddd
    (vmzero.q $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_mquad!($d),
            "\n.byte 0x80",
            "\n.byte 0b10000110",
            "\n.byte 0b11110011",
        )
    };

    // vscl.p 0110 0101 0 ttttttt 0 sssssss 1 ddddddd
    (vscl.p $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte ", $crate::register_single!($t),
            "\n.byte 0b01100101",
        )
    };

    // vscl.t 0110 0101 0 ttttttt 1 sssssss 0 ddddddd
    (vscl.t $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte ", $crate::register_single!($t),
            "\n.byte 0b01100101",
        )
    };

    // vscl.q 0110 0101 0 ttttttt 1 sssssss 1 ddddddd
    (vscl.q $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte ", $crate::register_single!($t),
            "\n.byte 0b01100101",
        )
    };

    // vmov.s 1101 0000 0 0000000 0 sssssss 0 ddddddd
    (vmov.s $d:ident, $s:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0",
            "\n.byte 0b11010000",
        )
    };

    // vmov.p 1101 0000 00000000 0 sssssss 1 ddddddd
    (vmov.p $d:ident, $s:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0",
            "\n.byte 0b11010000",
        )
    };

    // vmov.t 1101 0000 00000000 1 sssssss 0 ddddddd
    (vmov.t $d:ident, $s:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0",
            "\n.byte 0b11010000",
        )
    };

    // vmov.q 1101 0000 00000000 1 sssssss 1 ddddddd
    (vmov.q $d:ident, $s:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0",
            "\n.byte 0b11010000",
        )
    };

    // vmmov.p 1111 0011 1 0000000 0 sssssss 1 ddddddd
    (vmmov.p $d:ident, $s:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_mpair!($d),
            "\n.byte ", $crate::register_mpair!($s),
            "\n.byte 0x80",
            "\n.byte 0b11110011",
        )
    };

    // vmmov.t 1111 0011 1 0000000 1 sssssss 0 ddddddd
    (vmmov.t $d:ident, $s:ident) => {
        concat!(
            "\n.byte ", $crate::register_mtriple!($d),
            "\n.byte 0x80 | ", $crate::register_mtriple!($s),
            "\n.byte 0x80",
            "\n.byte 0b11110011",
        )
    };

    // vmmov.q 1111 0011 1 0000000 1 sssssss 1 ddddddd
    (vmmov.q $d:ident, $s:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_mquad!($d),
            "\n.byte 0x80 | ", $crate::register_mquad!($s),
            "\n.byte 0x80",
            "\n.byte 0b11110011",
        )
    };

    // viim.s 1101 1111 0 ddddddd iiiiiiii iiiiiiii
    (viim.s $d:ident, $i:literal) => {
        concat!(
            "\n.byte ", stringify!($i), " & 0xff",
            "\n.byte ", stringify!($i), " >> 8",
            "\n.byte ", $crate::register_single!($d),
            "\n.byte 0b11011111",
        )
    };

    // vrot.p 1111 0011 101iiiii 0 sssssss 1 ddddddd
    (vrot.p $d:ident, $s:ident, [ $($tt:tt)* ]) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0b10100000 | ", $crate::vrot_immediate_pair!($($tt)*),
            "\n.byte 0b11110011",
        )
    };

    // vrot.t 1111 0011 101iiiii 1 sssssss 0 ddddddd
    (vrot.t $d:ident, $s:ident, [ $($tt:tt)* ]) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_single!($s),
            "\n.byte 0b10100000 | ", $crate::vrot_immediate_triple!($($tt)*),
            "\n.byte 0b11110011",
        )
    };

    // vrot.q 1111 0011 101iiiii 1 sssssss 1 ddddddd
    (vrot.q $d:ident, $s:ident, [ $($tt:tt)* ]) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_single!($s),
            "\n.byte 0b10100000 | ", $crate::vrot_immediate_quad!($($tt)*),
            "\n.byte 0b11110011",
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
    (S000) => {"0b0000000"}; (S001) => {"0b0100000"}; (S002) => {"0b1000000"}; (S003) => {"0b1100000"};
    (S010) => {"0b0000001"}; (S011) => {"0b0100001"}; (S012) => {"0b1000001"}; (S013) => {"0b1100001"};
    (S020) => {"0b0000010"}; (S021) => {"0b0100010"}; (S022) => {"0b1000010"}; (S023) => {"0b1100010"};
    (S030) => {"0b0000011"}; (S031) => {"0b0100011"}; (S032) => {"0b1000011"}; (S033) => {"0b1100011"};

    (S100) => {"0b0000100"}; (S101) => {"0b0100100"}; (S102) => {"0b1000100"}; (S103) => {"0b1100100"};
    (S110) => {"0b0000101"}; (S111) => {"0b0100101"}; (S112) => {"0b1000101"}; (S113) => {"0b1100101"};
    (S120) => {"0b0000110"}; (S121) => {"0b0100110"}; (S122) => {"0b1000110"}; (S123) => {"0b1100110"};
    (S130) => {"0b0000111"}; (S131) => {"0b0100111"}; (S132) => {"0b1000111"}; (S133) => {"0b1100111"};

    (S200) => {"0b0001000"}; (S201) => {"0b0101000"}; (S202) => {"0b1001000"}; (S203) => {"0b1101000"};
    (S210) => {"0b0001001"}; (S211) => {"0b0101001"}; (S212) => {"0b1001001"}; (S213) => {"0b1101001"};
    (S220) => {"0b0001010"}; (S221) => {"0b0101010"}; (S222) => {"0b1001010"}; (S223) => {"0b1101010"};
    (S230) => {"0b0001011"}; (S231) => {"0b0101011"}; (S232) => {"0b1001011"}; (S233) => {"0b1101011"};

    (S300) => {"0b0001100"}; (S301) => {"0b0101100"}; (S302) => {"0b1001100"}; (S303) => {"0b1101100"};
    (S310) => {"0b0001101"}; (S311) => {"0b0101101"}; (S312) => {"0b1001101"}; (S313) => {"0b1101101"};
    (S320) => {"0b0001110"}; (S321) => {"0b0101110"}; (S322) => {"0b1001110"}; (S323) => {"0b1101110"};
    (S330) => {"0b0001111"}; (S331) => {"0b0101111"}; (S332) => {"0b1001111"}; (S333) => {"0b1101111"};

    (S400) => {"0b0010000"}; (S401) => {"0b0110000"}; (S402) => {"0b1010000"}; (S403) => {"0b1110000"};
    (S410) => {"0b0010001"}; (S411) => {"0b0110001"}; (S412) => {"0b1010001"}; (S413) => {"0b1110001"};
    (S420) => {"0b0010010"}; (S421) => {"0b0110010"}; (S422) => {"0b1010010"}; (S423) => {"0b1110010"};
    (S430) => {"0b0010011"}; (S431) => {"0b0110011"}; (S432) => {"0b1010011"}; (S433) => {"0b1110011"};

    (S500) => {"0b0010100"}; (S501) => {"0b0110100"}; (S502) => {"0b1010100"}; (S503) => {"0b1110100"};
    (S510) => {"0b0010101"}; (S511) => {"0b0110101"}; (S512) => {"0b1010101"}; (S513) => {"0b1110101"};
    (S520) => {"0b0010110"}; (S521) => {"0b0110110"}; (S522) => {"0b1010110"}; (S523) => {"0b1110110"};
    (S530) => {"0b0010111"}; (S531) => {"0b0110111"}; (S532) => {"0b1010111"}; (S533) => {"0b1110111"};

    (S600) => {"0b0011000"}; (S601) => {"0b0111000"}; (S602) => {"0b1011000"}; (S603) => {"0b1111000"};
    (S610) => {"0b0011001"}; (S611) => {"0b0111001"}; (S612) => {"0b1011001"}; (S613) => {"0b1111001"};
    (S620) => {"0b0011010"}; (S621) => {"0b0111010"}; (S622) => {"0b1011010"}; (S623) => {"0b1111010"};
    (S630) => {"0b0011011"}; (S631) => {"0b0111011"}; (S632) => {"0b1011011"}; (S633) => {"0b1111011"};

    (S700) => {"0b0011100"}; (S701) => {"0b0111100"}; (S702) => {"0b1011100"}; (S703) => {"0b1111100"};
    (S710) => {"0b0011101"}; (S711) => {"0b0111101"}; (S712) => {"0b1011101"}; (S713) => {"0b1111101"};
    (S720) => {"0b0011110"}; (S721) => {"0b0111110"}; (S722) => {"0b1011110"}; (S723) => {"0b1111110"};
    (S730) => {"0b0011111"}; (S731) => {"0b0111111"}; (S732) => {"0b1011111"}; (S733) => {"0b1111111"};
}

#[doc(hidden)]
#[macro_export]
macro_rules! register_pair {
    (C000) => {"0b0000000"}; (C002) => {"0b1000000"}; (C010) => {"0b0000001"}; (C012) => {"0b1000001"};
    (C020) => {"0b0000010"}; (C022) => {"0b1000010"}; (C030) => {"0b0000011"}; (C032) => {"0b1000011"};
    (C100) => {"0b0000100"}; (C102) => {"0b1000100"}; (C110) => {"0b0000101"}; (C112) => {"0b1000101"};
    (C120) => {"0b0000110"}; (C122) => {"0b1000110"}; (C130) => {"0b0000111"}; (C132) => {"0b1000111"};
    (C200) => {"0b0001000"}; (C202) => {"0b1001000"}; (C210) => {"0b0001001"}; (C212) => {"0b1001001"};
    (C220) => {"0b0001010"}; (C222) => {"0b1001010"}; (C230) => {"0b0001011"}; (C232) => {"0b1001011"};
    (C300) => {"0b0001100"}; (C302) => {"0b1001100"}; (C310) => {"0b0001101"}; (C312) => {"0b1001101"};
    (C320) => {"0b0001110"}; (C322) => {"0b1001110"}; (C330) => {"0b0001111"}; (C332) => {"0b1001111"};
    (C400) => {"0b0010000"}; (C402) => {"0b1010000"}; (C410) => {"0b0010001"}; (C412) => {"0b1010001"};
    (C420) => {"0b0010010"}; (C422) => {"0b1010010"}; (C430) => {"0b0010011"}; (C432) => {"0b1010011"};
    (C500) => {"0b0010100"}; (C502) => {"0b1010100"}; (C510) => {"0b0010101"}; (C512) => {"0b1010101"};
    (C520) => {"0b0010110"}; (C522) => {"0b1010110"}; (C530) => {"0b0010111"}; (C532) => {"0b1010111"};
    (C600) => {"0b0011000"}; (C602) => {"0b1011000"}; (C610) => {"0b0011001"}; (C612) => {"0b1011001"};
    (C620) => {"0b0011010"}; (C622) => {"0b1011010"}; (C630) => {"0b0011011"}; (C632) => {"0b1011011"};
    (C700) => {"0b0011100"}; (C702) => {"0b1011100"}; (C710) => {"0b0011101"}; (C712) => {"0b1011101"};
    (C720) => {"0b0011110"}; (C722) => {"0b1011110"}; (C730) => {"0b0011111"}; (C732) => {"0b1011111"};

    (R000) => {"0b0100000"}; (R001) => {"0b0100001"}; (R002) => {"0b0100010"}; (R003) => {"0b0100011"};
    (R020) => {"0b1100000"}; (R021) => {"0b1100001"}; (R022) => {"0b1100010"}; (R023) => {"0b1100011"};
    (R100) => {"0b0100100"}; (R101) => {"0b0100101"}; (R102) => {"0b0100110"}; (R103) => {"0b0100111"};
    (R120) => {"0b1100100"}; (R121) => {"0b1100101"}; (R122) => {"0b1100110"}; (R123) => {"0b1100111"};
    (R200) => {"0b0101000"}; (R201) => {"0b0101001"}; (R202) => {"0b0101010"}; (R203) => {"0b0101011"};
    (R220) => {"0b1101000"}; (R221) => {"0b1101001"}; (R222) => {"0b1101010"}; (R223) => {"0b1101011"};
    (R300) => {"0b0101100"}; (R301) => {"0b0101101"}; (R302) => {"0b0101110"}; (R303) => {"0b0101111"};
    (R320) => {"0b1101100"}; (R321) => {"0b1101101"}; (R322) => {"0b1101110"}; (R323) => {"0b1101111"};
    (R400) => {"0b0110000"}; (R401) => {"0b0110001"}; (R402) => {"0b0110010"}; (R403) => {"0b0110011"};
    (R420) => {"0b1110000"}; (R421) => {"0b1110001"}; (R422) => {"0b1110010"}; (R423) => {"0b1110011"};
    (R500) => {"0b0110100"}; (R501) => {"0b0110101"}; (R502) => {"0b0110110"}; (R503) => {"0b0110111"};
    (R520) => {"0b1110100"}; (R521) => {"0b1110101"}; (R522) => {"0b1110110"}; (R523) => {"0b1110111"};
    (R600) => {"0b0111000"}; (R601) => {"0b0111001"}; (R602) => {"0b0111010"}; (R603) => {"0b0111011"};
    (R620) => {"0b1111000"}; (R621) => {"0b1111001"}; (R622) => {"0b1111010"}; (R623) => {"0b1111011"};
    (R700) => {"0b0111100"}; (R701) => {"0b0111101"}; (R702) => {"0b0111110"}; (R703) => {"0b0111111"};
    (R720) => {"0b1111100"}; (R721) => {"0b1111101"}; (R722) => {"0b1111110"}; (R723) => {"0b1111111"};
}

#[doc(hidden)]
#[macro_export]
macro_rules! register_triple {
    (C000) => {"0b0000000"}; (C001) => {"0b1000000"}; (C010) => {"0b0000001"}; (C011) => {"0b1000001"};
    (C020) => {"0b0000010"}; (C021) => {"0b1000010"}; (C030) => {"0b0000011"}; (C031) => {"0b1000011"};
    (C100) => {"0b0000100"}; (C101) => {"0b1000100"}; (C110) => {"0b0000101"}; (C111) => {"0b1000101"};
    (C120) => {"0b0000110"}; (C121) => {"0b1000110"}; (C130) => {"0b0000111"}; (C131) => {"0b1000111"};
    (C200) => {"0b0001000"}; (C201) => {"0b1001000"}; (C210) => {"0b0001001"}; (C211) => {"0b1001001"};
    (C220) => {"0b0001010"}; (C221) => {"0b1001010"}; (C230) => {"0b0001011"}; (C231) => {"0b1001011"};
    (C300) => {"0b0001100"}; (C301) => {"0b1001100"}; (C310) => {"0b0001101"}; (C311) => {"0b1001101"};
    (C320) => {"0b0001110"}; (C321) => {"0b1001110"}; (C330) => {"0b0001111"}; (C331) => {"0b1001111"};
    (C400) => {"0b0010000"}; (C401) => {"0b1010000"}; (C410) => {"0b0010001"}; (C411) => {"0b1010001"};
    (C420) => {"0b0010010"}; (C421) => {"0b1010010"}; (C430) => {"0b0010011"}; (C431) => {"0b1010011"};
    (C500) => {"0b0010100"}; (C501) => {"0b1010100"}; (C510) => {"0b0010101"}; (C511) => {"0b1010101"};
    (C520) => {"0b0010110"}; (C521) => {"0b1010110"}; (C530) => {"0b0010111"}; (C531) => {"0b1010111"};
    (C600) => {"0b0011000"}; (C601) => {"0b1011000"}; (C610) => {"0b0011001"}; (C611) => {"0b1011001"};
    (C620) => {"0b0011010"}; (C621) => {"0b1011010"}; (C630) => {"0b0011011"}; (C631) => {"0b1011011"};
    (C700) => {"0b0011100"}; (C701) => {"0b1011100"}; (C710) => {"0b0011101"}; (C711) => {"0b1011101"};
    (C720) => {"0b0011110"}; (C721) => {"0b1011110"}; (C730) => {"0b0011111"}; (C731) => {"0b1011111"};

    (R000) => {"0b0100000"}; (R001) => {"0b0100001"}; (R002) => {"0b0100010"}; (R003) => {"0b0100011"};
    (R010) => {"0b1100000"}; (R011) => {"0b1100001"}; (R012) => {"0b1100010"}; (R013) => {"0b1100011"};
    (R100) => {"0b0100100"}; (R101) => {"0b0100101"}; (R102) => {"0b0100110"}; (R103) => {"0b0100111"};
    (R110) => {"0b1100100"}; (R111) => {"0b1100101"}; (R112) => {"0b1100110"}; (R113) => {"0b1100111"};
    (R200) => {"0b0101000"}; (R201) => {"0b0101001"}; (R202) => {"0b0101010"}; (R203) => {"0b0101011"};
    (R210) => {"0b1101000"}; (R211) => {"0b1101001"}; (R212) => {"0b1101010"}; (R213) => {"0b1101011"};
    (R300) => {"0b0101100"}; (R301) => {"0b0101101"}; (R302) => {"0b0101110"}; (R303) => {"0b0101111"};
    (R310) => {"0b1101100"}; (R311) => {"0b1101101"}; (R312) => {"0b1101110"}; (R313) => {"0b1101111"};
    (R400) => {"0b0110000"}; (R401) => {"0b0110001"}; (R402) => {"0b0110010"}; (R403) => {"0b0110011"};
    (R410) => {"0b1110000"}; (R411) => {"0b1110001"}; (R412) => {"0b1110010"}; (R413) => {"0b1110011"};
    (R500) => {"0b0110100"}; (R501) => {"0b0110101"}; (R502) => {"0b0110110"}; (R503) => {"0b0110111"};
    (R510) => {"0b1110100"}; (R511) => {"0b1110101"}; (R512) => {"0b1110110"}; (R513) => {"0b1110111"};
    (R600) => {"0b0111000"}; (R601) => {"0b0111001"}; (R602) => {"0b0111010"}; (R603) => {"0b0111011"};
    (R610) => {"0b1111000"}; (R611) => {"0b1111001"}; (R612) => {"0b1111010"}; (R613) => {"0b1111011"};
    (R700) => {"0b0111100"}; (R701) => {"0b0111101"}; (R702) => {"0b0111110"}; (R703) => {"0b0111111"};
    (R710) => {"0b1111100"}; (R711) => {"0b1111101"}; (R712) => {"0b1111110"}; (R713) => {"0b1111111"};
}

#[doc(hidden)]
#[macro_export]
macro_rules! register_quad {
    (C000) => {"0b0000000"}; (C010) => {"0b0000001"}; (C020) => {"0b0000010"}; (C030) => {"0b0000011"};
    (C100) => {"0b0000100"}; (C110) => {"0b0000101"}; (C120) => {"0b0000110"}; (C130) => {"0b0000111"};
    (C200) => {"0b0001000"}; (C210) => {"0b0001001"}; (C220) => {"0b0001010"}; (C230) => {"0b0001011"};
    (C300) => {"0b0001100"}; (C310) => {"0b0001101"}; (C320) => {"0b0001110"}; (C330) => {"0b0001111"};
    (C400) => {"0b0010000"}; (C410) => {"0b0010001"}; (C420) => {"0b0010010"}; (C430) => {"0b0010011"};
    (C500) => {"0b0010100"}; (C510) => {"0b0010101"}; (C520) => {"0b0010110"}; (C530) => {"0b0010111"};
    (C600) => {"0b0011000"}; (C610) => {"0b0011001"}; (C620) => {"0b0011010"}; (C630) => {"0b0011011"};
    (C700) => {"0b0011100"}; (C710) => {"0b0011101"}; (C720) => {"0b0011110"}; (C730) => {"0b0011111"};

    (R000) => {"0b0100000"}; (R001) => {"0b0100001"}; (R002) => {"0b0100010"}; (R003) => {"0b0100011"};
    (R100) => {"0b0100100"}; (R101) => {"0b0100101"}; (R102) => {"0b0100110"}; (R103) => {"0b0100111"};
    (R200) => {"0b0101000"}; (R201) => {"0b0101001"}; (R202) => {"0b0101010"}; (R203) => {"0b0101011"};
    (R300) => {"0b0101100"}; (R301) => {"0b0101101"}; (R302) => {"0b0101110"}; (R303) => {"0b0101111"};
    (R400) => {"0b0110000"}; (R401) => {"0b0110001"}; (R402) => {"0b0110010"}; (R403) => {"0b0110011"};
    (R500) => {"0b0110100"}; (R501) => {"0b0110101"}; (R502) => {"0b0110110"}; (R503) => {"0b0110111"};
    (R600) => {"0b0111000"}; (R601) => {"0b0111001"}; (R602) => {"0b0111010"}; (R603) => {"0b0111011"};
    (R700) => {"0b0111100"}; (R701) => {"0b0111101"}; (R702) => {"0b0111110"}; (R703) => {"0b0111111"};
}

/// Matrix variant of `register_pair!`
#[doc(hidden)]
#[macro_export]
macro_rules! register_mpair {
    (M000) => {"0b0000000"}; (M002) => {"0b1000000"}; (M020) => {"0b0000010"}; (M022) => {"0b1000010"};
    (M100) => {"0b0000100"}; (M102) => {"0b1000100"}; (M120) => {"0b0000110"}; (M122) => {"0b1000110"};
    (M200) => {"0b0001000"}; (M202) => {"0b1001000"}; (M220) => {"0b0001010"}; (M222) => {"0b1001010"};
    (M300) => {"0b0001100"}; (M302) => {"0b1001100"}; (M320) => {"0b0001110"}; (M322) => {"0b1001110"};
    (M400) => {"0b0010000"}; (M402) => {"0b1010000"}; (M420) => {"0b0010010"}; (M422) => {"0b1010010"};
    (M500) => {"0b0010100"}; (M502) => {"0b1010100"}; (M520) => {"0b0010110"}; (M522) => {"0b1010110"};
    (M600) => {"0b0011000"}; (M602) => {"0b1011000"}; (M620) => {"0b0011010"}; (M622) => {"0b1011010"};
    (M700) => {"0b0011100"}; (M702) => {"0b1011100"}; (M720) => {"0b0011110"}; (M722) => {"0b1011110"};

    (E000) => {"0b0100000"}; (E002) => {"0b0100010"}; (E020) => {"0b1100000"}; (E022) => {"0b1100010"};
    (E100) => {"0b0100100"}; (E102) => {"0b0100110"}; (E120) => {"0b1100100"}; (E122) => {"0b1100110"};
    (E200) => {"0b0101000"}; (E202) => {"0b0101010"}; (E220) => {"0b1101000"}; (E222) => {"0b1101010"};
    (E300) => {"0b0101100"}; (E302) => {"0b0101110"}; (E320) => {"0b1101100"}; (E322) => {"0b1101110"};
    (E400) => {"0b0110000"}; (E402) => {"0b0110010"}; (E420) => {"0b1110000"}; (E422) => {"0b1110010"};
    (E500) => {"0b0110100"}; (E502) => {"0b0110110"}; (E520) => {"0b1110100"}; (E522) => {"0b1110110"};
    (E600) => {"0b0111000"}; (E602) => {"0b0111010"}; (E620) => {"0b1111000"}; (E622) => {"0b1111010"};
    (E700) => {"0b0111100"}; (E702) => {"0b0111110"}; (E720) => {"0b1111100"}; (E722) => {"0b1111110"};
}

/// Matrix variant of `register_triple!`
#[doc(hidden)]
#[macro_export]
macro_rules! register_mtriple {
    (M000) => {"0b0000000"}; (M001) => {"0b1000000"}; (M010) => {"0b0000001"}; (M011) => {"0b1000001"};
    (M100) => {"0b0000100"}; (M101) => {"0b1000100"}; (M110) => {"0b0000101"}; (M111) => {"0b1000101"};
    (M200) => {"0b0001000"}; (M201) => {"0b1001000"}; (M210) => {"0b0001001"}; (M211) => {"0b1001001"};
    (M300) => {"0b0001100"}; (M301) => {"0b1001100"}; (M310) => {"0b0001101"}; (M311) => {"0b1001101"};
    (M400) => {"0b0010000"}; (M401) => {"0b1010000"}; (M410) => {"0b0010001"}; (M411) => {"0b1010001"};
    (M500) => {"0b0010100"}; (M501) => {"0b1010100"}; (M510) => {"0b0010101"}; (M511) => {"0b1010101"};
    (M600) => {"0b0011000"}; (M601) => {"0b1011000"}; (M610) => {"0b0011001"}; (M611) => {"0b1011001"};
    (M700) => {"0b0011100"}; (M701) => {"0b1011100"}; (M710) => {"0b0011101"}; (M711) => {"0b1011101"};

    (E000) => {"0b0100000"}; (E001) => {"0b0100001"}; (E010) => {"0b1100000"}; (E011) => {"0b1100001"};
    (E100) => {"0b0100100"}; (E101) => {"0b0100101"}; (E110) => {"0b1100100"}; (E111) => {"0b1100101"};
    (E200) => {"0b0101000"}; (E201) => {"0b0101001"}; (E210) => {"0b1101000"}; (E211) => {"0b1101001"};
    (E300) => {"0b0101100"}; (E301) => {"0b0101101"}; (E310) => {"0b1101100"}; (E311) => {"0b1101101"};
    (E400) => {"0b0110000"}; (E401) => {"0b0110001"}; (E410) => {"0b1110000"}; (E411) => {"0b1110001"};
    (E500) => {"0b0110100"}; (E501) => {"0b0110101"}; (E510) => {"0b1110100"}; (E511) => {"0b1110101"};
    (E600) => {"0b0111000"}; (E601) => {"0b0111001"}; (E610) => {"0b1111000"}; (E611) => {"0b1111001"};
    (E700) => {"0b0111100"}; (E701) => {"0b0111101"}; (E710) => {"0b1111100"}; (E711) => {"0b1111101"};
}

/// Matrix variant of `register_quad!`
#[doc(hidden)]
#[macro_export]
macro_rules! register_mquad {
    (M000) => {"0b0000000"}; (M100) => {"0b0000100"}; (M200) => {"0b0001000"}; (M300) => {"0b0001100"};
    (M400) => {"0b0010000"}; (M500) => {"0b0010100"}; (M600) => {"0b0011000"}; (M700) => {"0b0011100"};

    (E000) => {"0b0100000"}; (E100) => {"0b0100100"}; (E200) => {"0b0101000"}; (E300) => {"0b0101100"};
    (E400) => {"0b0110000"}; (E500) => {"0b0110100"}; (E600) => {"0b0111000"}; (E700) => {"0b0111100"};
}

#[doc(hidden)]
#[macro_export]
macro_rules! vfpu_const {
    (VFPU_HUGE) => {"1"};
    (VFPU_SQRT2) => {"2"};
    (VFPU_SQRT1_2) => {"3"};
    (VFPU_2_SQRTPI) => {"4"};
    (VFPU_2_PI) => {"5"};
    (VFPU_1_PI) => {"6"};
    (VFPU_PI_4) => {"7"};
    (VFPU_PI_2) => {"8"};
    (VFPU_PI) => {"9"};
    (VFPU_E) => {"10"};
    (VFPU_LOG2E) => {"11"};
    (VFPU_LOG10E) => {"12"};
    (VFPU_LN2) => {"13"};
    (VFPU_LN10) => {"14"};
    (VFPU_2PI) => {"15"};
    (VFPU_PI_6) => {"16"};
    (VFPU_LOG10TWO) => {"17"};
    (VFPU_LOG2TEN) => {"18"};
    (VFPU_SQRT3_2) => {"19"};
    (VFPU_SQRT4_3) => {"20"};
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
    () => {"0"};
    (0) => {"1"};
    (1) => {"3"};
    (M) => {"0x100"};
}

#[doc(hidden)]
#[macro_export]
macro_rules! vrot_immediate_pair {
    // There are duplicates here, but this is fine. They come from the quad
    // variant of this macro. Any resulting immediate number should do.

    (C, S) => {"0"};
    (S, C) => {"1"};
    (S, 0) => {"2"};
    (S, 0) => {"3"};
    (C, S) => {"4"};
    (S, C) => {"5"};
    (0, S) => {"6"};
    (0, S) => {"7"};
    (C, 0) => {"8"};
    (0, C) => {"9"};
    (S, S) => {"10"};
    (0, 0) => {"11"};
    (C, 0) => {"12"};
    (0, C) => {"13"};
    (0, 0) => {"14"};
    (S, S) => {"15"};
    (C, -S) => {"16"};
    (-S, C) => {"17"};
    (-S, 0) => {"18"};
    (-S, 0) => {"19"};
    (C, -S) => {"20"};
    (-S, C) => {"21"};
    (0, -S) => {"22"};
    (0, -S) => {"23"};
    (C, 0) => {"24"};
    (0, C) => {"25"};
    (-S, -S) => {"26"};
    (0, 0) => {"27"};
    (C, 0) => {"28"};
    (0, C) => {"29"};
    (0, 0) => {"30"};
    (-S, -S) => {"31"};
}

#[doc(hidden)]
#[macro_export]
macro_rules! vrot_immediate_triple {
    // Duplicates, like `vrot_immediate_pair!`, are fine.

    (C, S, S) => {"0"};
    (S, C, 0) => {"1"};
    (S, 0, C) => {"2"};
    (S, 0, 0) => {"3"};
    (C, S, 0) => {"4"};
    (S, C, S) => {"5"};
    (0, S, C) => {"6"};
    (0, S, 0) => {"7"};
    (C, 0, S) => {"8"};
    (0, C, S) => {"9"};
    (S, S, C) => {"10"};
    (0, 0, S) => {"11"};
    (C, 0, 0) => {"12"};
    (0, C, 0) => {"13"};
    (0, 0, C) => {"14"};
    (S, S, S) => {"15"};
    (C, -S, -S) => {"16"};
    (-S, C, 0) => {"17"};
    (-S, 0, C) => {"18"};
    (-S, 0, 0) => {"19"};
    (C, -S, 0) => {"20"};
    (-S, C, -S) => {"21"};
    (0, -S, C) => {"22"};
    (0, -S, 0) => {"23"};
    (C, 0, -S) => {"24"};
    (0, C, -S) => {"25"};
    (-S, -S, C) => {"26"};
    (0, 0, -S) => {"27"};
    (C, 0, 0) => {"28"};
    (0, C, 0) => {"29"};
    (0, 0, C) => {"30"};
    (-S, -S, -S) => {"31"};
}

#[doc(hidden)]
#[macro_export]
macro_rules! vrot_immediate_quad {
    (C, S, S, S) => {"0"};
    (S, C, 0, 0) => {"1"};
    (S, 0, C, 0) => {"2"};
    (S, 0, 0, C) => {"3"};
    (C, S, 0, 0) => {"4"};
    (S, C, S, S) => {"5"};
    (0, S, C, 0) => {"6"};
    (0, S, 0, C) => {"7"};
    (C, 0, S, 0) => {"8"};
    (0, C, S, 0) => {"9"};
    (S, S, C, S) => {"10"};
    (0, 0, S, C) => {"11"};
    (C, 0, 0, S) => {"12"};
    (0, C, 0, S) => {"13"};
    (0, 0, C, S) => {"14"};
    (S, S, S, C) => {"15"};
    (C, -S, -S, -S) => {"16"};
    (-S, C, 0, 0) => {"17"};
    (-S, 0, C, 0) => {"18"};
    (-S, 0, 0, C) => {"19"};
    (C, -S, 0, 0) => {"20"};
    (-S, C, -S, -S) => {"21"};
    (0, -S, C, 0) => {"22"};
    (0, -S, 0, C) => {"23"};
    (C, 0, -S, 0) => {"24"};
    (0, C, -S, 0) => {"25"};
    (-S, -S, C, -S) => {"26"};
    (0, 0, -S, C) => {"27"};
    (C, 0, 0, -S) => {"28"};
    (0, C, 0, -S) => {"29"};
    (0, 0, C, -S) => {"30"};
    (-S, -S, -S, C) => {"31"};
}
