//! VFPU support.

/// A macro-based VFPU assembler.
#[macro_export]
macro_rules! vfpu_asm {
    (
        $(
            $($opcode:ident $($arg:tt $(($base:ident))? $(, [$flag:literal])?),*)?
            $(.mips $asm:literal)?
        );*;

        // Optional in / out constraints
        $(

            // Output constraints
            : $($out_constraint:literal ($out_expr:expr)),*

            // Input constraints
            $(
                : $($in_constraint:literal ($in_expr:expr)),*

                // Clobbers
                $(
                    : $($clobber:literal),*
                )?
            )?
        )?
    ) => {
        llvm_asm!(
            concat!(
                ".set push\n",
                ".set noreorder\n",
                ".align 2\n",
                $(
                    $($crate::instruction!($opcode $($arg $(($base))? $(, $flag)?),*))?
                    $($crate::instruction!(mips $asm))?

                    , "\n"
                ),*,
                ".set pop"
            )

            $(
                : $($out_constraint ($out_expr)),*

                $(
                    :$($in_constraint ($in_expr)),*

                    $(
                        : $($clobber),*
                    )?
                )?
            )?
        );
    }
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
    (lv_q $t:ident, $s:ident) => { $crate::instruction!(lv_q $t, 0($s)) };

    // lv.q 110110ss sssttttt oooooooo oooooo0t
    (lv_q $t:ident, $offset:literal ( $s:ident )) => {
        concat!(
            "\n.byte (((", stringify!($offset), " / 4) << 2) & 0xff) | ((",
                $crate::register_quad!($t), " >> 5) & 1)",
            "\n.byte (", stringify!($offset), " / 4) >> 6",
            "\n.byte ((", $crate::register_mips!($s), " << 5) & 0xff) | (",
                $crate::register_quad!($t), "& 0b11111)",
            "\n.byte 0b11011000 | (", $crate::register_mips!($s), " >> 3)",
        )
    };

    // No offset, no writeback
    (sv_q $t:ident, $s:ident) => {
        $crate::instruction!(sv_q $t, 0($s), wb:0)
    };

    // No offset, has writeback
    (sv_q $t:ident, $s:ident, wb) => {
        $crate::instruction!(sv_q $t, 0($s), wb:1)
    };

    // Has offset, no writeback
    (sv_q $t:ident, $offset:literal ( $s:ident )) => {
        $crate::instruction!(sv_q $t, $offset ($s), wb:0)
    };

    // Has offset, has writeback
    (sv_q $t:ident, $offset:literal ( $s:ident ), wb) => {
        $crate::instruction!(sv_q $t, $offset ($s), wb:1)
    };

    // sv.q 111110ss sssttttt oooooooo oooooowt
    (sv_q $t:ident, $offset:literal ( $s:ident ), wb:$wb:literal) => {
        concat!(
            "\n.byte (((", stringify!($offset), " / 4) << 2) & 0xff) | ((",
                $crate::register_quad!($t), " >> 5) & 1)",
                " | (", stringify!($wb), " << 1)",
            "\n.byte (", stringify!($offset), " / 4) >> 6",
            "\n.byte ((", $crate::register_mips!($s), " << 5) & 0xff) | (",
                $crate::register_quad!($t), " & 0b11111)",
            "\n.byte 0b11111000 | (", $crate::register_mips!($s), " >> 3)",
        )
    };

    // mtv: 0100 1000 111 sssss 0000 0000 0 ddddddd
    (mtv $s:ident, $d:ident) => {
        concat!(
            ".byte ", $crate::register_single!($d),
            "\n.byte 0\n",
            ".byte 0b11100000 | ", $crate::register_mips!($s),
            "\n.byte 0b01001000\n",
        )
    };

    // mfv: 0100 1000 011 ddddd 000000000 sssssss
    (mfv $d:ident, $s:ident) => {
        concat!(
            ".byte ", $crate::register_single!($s),
            "\n.byte 0\n",
            ".byte 0b01100000 | ", $crate::register_mips!($d),
            "\n.byte 0b01001000\n",
        )
    };

    // vadd.s: 0110 0000 0 ttttttt 0 sssssss 0 ddddddd
    (vadd_s $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte ", $crate::register_single!($t),
            "\n.byte 0b01100000",
        )
    };

    // vadd.p: 0110 0000 0 ttttttt 0 sssssss 1 ddddddd
    (vadd_p $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte ", $crate::register_pair!($t),
            "\n.byte 0b01100000",
        )
    };

    // vadd.t: 0110 0000 0 ttttttt 1 sssssss 0 ddddddd
    (vadd_t $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte ", $crate::register_triple!($t),
            "\n.byte 0b01100000",
        )
    };

    // vadd.q: 0110 0000 0 ttttttt 1 sssssss 1 ddddddd
    (vadd_q $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte ", $crate::register_quad!($t),
            "\n.byte 0b01100000",
        )
    };

    // vsub.s: 0110 1000 0 ttttttt 0 sssssss 0 ddddddd	
    (vsub_s $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte ", $crate::register_single!($t),
            "\n.byte 0b01101000",
        )
    };

    // vsub.p: 0110 1000 0 ttttttt 0 sssssss 1 ddddddd	
    (vsub_p $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte ", $crate::register_pair!($t),
            "\n.byte 0b01101000",
        )
    };

    // vsub.t: 0110 1000 0 ttttttt 1 sssssss 0 ddddddd	
    (vsub_t $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte ", $crate::register_triple!($t),
            "\n.byte 0b01101000",
        )
    };

    // vsub.q: 0110 1000 0 ttttttt 1 sssssss 1 ddddddd	
    (vsub_q $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte ", $crate::register_quad!($t),
            "\n.byte 0b01101000",
        )
    };

    // vdiv.s: 0110 0011 1 ttttttt 0 sssssss 0 ddddddd	 
    (vdiv_s $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0x80 | ", $crate::register_single!($t),
            "\n.byte 0b01100011",
        )
    };

    // vdiv.p: 0110 0011 1 ttttttt 0 sssssss 1 ddddddd	 
    (vdiv_p $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0x80 | ", $crate::register_pair!($t),
            "\n.byte 0b01100011",
        )
    };

    // vdiv.t: 0110 0011 1 ttttttt 1 sssssss 0 ddddddd	 
    (vdiv_t $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0x80 | ", $crate::register_triple!($t),
            "\n.byte 0b01100011",
        )
    };

    // vdiv.q: 0110 0011 1 ttttttt 1 sssssss 1 ddddddd	 
    (vdiv_q $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0x80 | ", $crate::register_quad!($t),
            "\n.byte 0b01100011",
        )
    };

    // vmul.s: 0110 0100 0 ttttttt 0 sssssss 0 ddddddd	 
    (vmul_s $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte ", $crate::register_single!($t),
            "\n.byte 0b01100100",
        )
    };

    // vmul.p: 0110 0100 0 ttttttt 0 sssssss 1 ddddddd	 
    (vmul_p $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte ", $crate::register_pair!($t),
            "\n.byte 0b01100100",
        )
    };

    // vmul.t: 0110 0100 0 ttttttt 1 sssssss 0 ddddddd	 
    (vmul_t $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte ", $crate::register_triple!($t),
            "\n.byte 0b01100100",
        )
    };

    // vmul.q: 0110 0100 0 ttttttt 1 sssssss 1 ddddddd	 
    (vmul_q $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte ", $crate::register_quad!($t),
            "\n.byte 0b01100100",
        )
    };

    // vdot.p: 0110 0100 1 ttttttt 0 sssssss 1 ddddddd	 
    (vdot_p $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0x80 | ", $crate::register_pair!($t),
            "\n.byte 0b01100100",
        )
    };

    // vdot.t: 0110 0100 1 ttttttt 1 sssssss 0 ddddddd	 
    (vdot_t $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0x80 | ", $crate::register_triple!($t),
            "\n.byte 0b01100100",
        )
    };

    // vdot.q: 0110 0100 1 ttttttt 1 sssssss 1 ddddddd	 
    (vdot_q $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0x80 | ", $crate::register_quad!($t),
            "\n.byte 0b01100100",
        )
    };

    // vhdp.p: 0110 0110 0 ttttttt 0 sssssss 1 ddddddd	 
    (vhdp_p $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte ", $crate::register_pair!($t),
            "\n.byte 0b01100110",
        )
    };

    // vhdp.t: 0110 0110 0 ttttttt 1 sssssss 0 ddddddd	 
    (vhdp_t $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte ", $crate::register_triple!($t),
            "\n.byte 0b01100110",
        )
    };

    // vhdp.q: 0110 0110 0 ttttttt 1 sssssss 1 ddddddd	 
    (vhdp_q $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte ", $crate::register_quad!($t),
            "\n.byte 0b01100110",
        )
    };

    // vmin.s: 0110 1101 0 ttttttt 0 sssssss 0 ddddddd	 
    (vmin_s $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte ", $crate::register_single!($t),
            "\n.byte 0b01101101",
        )
    };

    // vmin.p: 0110 1101 0 ttttttt 0 sssssss 1 ddddddd	 
    (vmin_p $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte ", $crate::register_pair!($t),
            "\n.byte 0b01101101",
        )
    };

    // vmin.t: 0110 1101 0 ttttttt 1 sssssss 0 ddddddd	 
    (vmin_t $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte ", $crate::register_triple!($t),
            "\n.byte 0b01101101",
        )
    };

    // vmin.q: 0110 1101 0 ttttttt 1 sssssss 1 ddddddd	 
    (vmin_q $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte ", $crate::register_quad!($t),
            "\n.byte 0b01101101",
        )
    };

    // vmax.s: 0110 1101 1 ttttttt 0 sssssss 0 ddddddd	 
    (vmax_s $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0x80 | ", $crate::register_single!($t),
            "\n.byte 0b01101101",
        )
    };

    // vmax.p: 0110 1101 1 ttttttt 0 sssssss 1 ddddddd	 
    (vmax_p $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0x80 | ", $crate::register_pair!($t),
            "\n.byte 0b01101101",
        )
    };

    // vmax.t: 0110 1101 1 ttttttt 1 sssssss 0 ddddddd	 
    (vmax_t $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0x80 | ", $crate::register_triple!($t),
            "\n.byte 0b01101101",
        )
    };

    // vmax.q: 0110 1101 1 ttttttt 1 sssssss 1 ddddddd
    (vmax_q $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0x80 | ", $crate::register_quad!($t),
            "\n.byte 0b01101101",
        )
    };

    // vabs.s 1101 0000 0 0000001 0 sssssss 0 ddddddd
    (vabs_s $d:ident, $s:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0b00000001",
            "\n.byte 0b11010000",
        )
    };

    // vabs.p 1101 0000 0 0000001 0 sssssss 1 ddddddd
    (vabs_p $d:ident, $s:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0b00000001",
            "\n.byte 0b11010000",
        )
    };

    // vabs.t 1101 0000 0 0000001 1 sssssss 0 ddddddd
    (vabs_t $d:ident, $s:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0b00000001",
            "\n.byte 0b11010000",
        )
    };

    // vabs.q 1101 0000 0 0000001 1 sssssss 1 ddddddd
    (vabs_q $d:ident, $s:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0b00000001",
            "\n.byte 0b11010000",
        )
    };

    // vneg.s 1101 0000 0 0000010 0 sssssss 0 ddddddd
    (vneg_s $d:ident, $s:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0b00000010",
            "\n.byte 0b11010000",
        )
    };

    // vneg.p 1101 0000 0 0000010 0 sssssss 1 ddddddd
    (vneg_p $d:ident, $s:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0b00000010",
            "\n.byte 0b11010000",
        )
    };

    // vneg.t 1101 0000 0 0000010 1 sssssss 0 ddddddd
    (vneg_t $d:ident, $s:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0b00000010",
            "\n.byte 0b11010000",
        )
    };

    // vneg.q 1101 0000 0 0000010 1 sssssss 1 ddddddd
    (vneg_q $d:ident, $s:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0b00000010",
            "\n.byte 0b11010000",
        )
    };

    // vidt.p 1101 0000 0 0000011 0 0000000 1 ddddddd	 
    (vidt_p $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte 0b00000000",
            "\n.byte 0b00000011",
            "\n.byte 0b11010000",
        )
    };

    // vidt.t 1101 0000 0 0000011 1 0000000 0 ddddddd 
    (vidt_t $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80",
            "\n.byte 0b00000011",
            "\n.byte 0b11010000",
        )
    };

    // vidt.q 1101 0000 0 0000011 1 0000000 1 ddddddd 
    (vidt_q $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80",
            "\n.byte 0b00000011",
            "\n.byte 0b11010000",
        )
    };

    // vzero.s 1101 0000 0 0000110 0 0000000 0 ddddddd
    (vzero_s $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte 0b00000000",
            "\n.byte 0b00000110",
            "\n.byte 0b11010000",
        )
    };

    // vzero.p 1101 0000 0 0000110 0 0000000 1 ddddddd
    (vzero_p $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte 0b00000000",
            "\n.byte 0b00000110",
            "\n.byte 0b11010000",
        )
    };

    // vzero.t 1101 0000 0 0000110 1 0000000 0 ddddddd
    (vzero_t $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80",
            "\n.byte 0b00000110",
            "\n.byte 0b11010000",
        )
    };

    // vzero.q 1101 0000 0 0000110 1 0000000 1 ddddddd
    (vzero_q $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80",
            "\n.byte 0b00000110",
            "\n.byte 0b11010000",
        )
    };

    // vone.s 1101 0000 0 0000111 0 0000000 0 ddddddd
    (vone_s $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte 0b00000000",
            "\n.byte 0b00000111",
            "\n.byte 0b11010000",
        )
    };

    // vone.p 1101 0000 0 0000111 0 0000000 1 ddddddd
    (vone_p $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte 0b00000000",
            "\n.byte 0b00000111",
            "\n.byte 0b11010000",
        )
    };

    // vone.t 1101 0000 0 0000111 1 0000000 0 ddddddd
    (vone_t $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80",
            "\n.byte 0b00000111",
            "\n.byte 0b11010000",
        )
    };

    // vone.q 1101 0000 0 0000111 1 0000000 1 ddddddd
    (vone_q $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80",
            "\n.byte 0b00000111",
            "\n.byte 0b11010000",
        )
    };

    // vrcp.s 1101 0000 0 0010000 0 sssssss 0 ddddddd	 
    (vrcp_s $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0b00010000",
            "\n.byte 0b11010000",
        )
    };

    // vrcp.p 1101 0000 0 0010000 0 sssssss 1 ddddddd	 
    (vrcp_p $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0b00010000",
            "\n.byte 0b11010000",
        )
    };

    // vrcp.t 1101 0000 0 0010000 1 sssssss 0 ddddddd	 
    (vrcp_t $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0b00010000",
            "\n.byte 0b11010000",
        )
    };

    // vrcp.q 1101 0000 0 0010000 1 sssssss 1 ddddddd	 
    (vrcp_q $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0b00010000",
            "\n.byte 0b11010000",
        )
    };

    // vrsq.s 1101 0000 0 0010001 0 sssssss 0 ddddddd	 
    (vrsq_s $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0b00010001",
            "\n.byte 0b11010000",
        )
    };

    // vrsq.p 1101 0000 0 0010001 0 sssssss 1 ddddddd	 
    (vrsq_p $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0b00010001",
            "\n.byte 0b11010000",
        )
    };

    // vrsq.t 1101 0000 0 0010001 1 sssssss 0 ddddddd	 
    (vrsq_t $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0b00010001",
            "\n.byte 0b11010000",
        )
    };

    // vrsq.q 1101 0000 0 0010001 1 sssssss 1 ddddddd	 
    (vrsq_q $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0b00010001",
            "\n.byte 0b11010000",
        )
    };

    // vsin.s 1101 0000 0 0010010 0 sssssss 0 ddddddd	 
    (vsin_s $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0b00010010",
            "\n.byte 0b11010000",
        )
    };

    // vsin.p 1101 0000 0 0010010 0 sssssss 1 ddddddd	 
    (vsin_p $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0b00010010",
            "\n.byte 0b11010000",
        )
    };

    // vsin.t 1101 0000 0 0010010 1 sssssss 0 ddddddd	 
    (vsin_t $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0b00010010",
            "\n.byte 0b11010000",
        )
    };

    // vsin.q 1101 0000 0 0010010 1 sssssss 1 ddddddd	 
    (vsin_q $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0b00010010",
            "\n.byte 0b11010000",
        )
    };

    // vcos.s 1101 0000 0 0010011 0 sssssss 0 ddddddd	 
    (vcos_s $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0b00010011",
            "\n.byte 0b11010000",
        )
    };

    // vcos.p 1101 0000 0 0010011 0 sssssss 1 ddddddd	 
    (vcos_p $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0b00010011",
            "\n.byte 0b11010000",
        )
    };

    // vcos.t 1101 0000 0 0010011 1 sssssss 0 ddddddd	 
    (vcos_t $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0b00010011",
            "\n.byte 0b11010000",
        )
    };

    // vcos.q 1101 0000 0 0010011 1 sssssss 1 ddddddd	 
    (vcos_q $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0b00010011",
            "\n.byte 0b11010000",
        )
    };

    // vexp2.s 1101 0000 0 0010100 0 sssssss 0 ddddddd	 
    (vexp2_s $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0b00010100",
            "\n.byte 0b11010000",
        )
    };

    // vexp2.p 1101 0000 0 0010100 0 sssssss 1 ddddddd	 
    (vexp2_p $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0b00010100",
            "\n.byte 0b11010000",
        )
    };

    // vexp2.t 1101 0000 0 0010100 1 sssssss 0 ddddddd	 
    (vexp2_t $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0b00010100",
            "\n.byte 0b11010000",
        )
    };

    // vexp2.q 1101 0000 0 0010100 1 sssssss 1 ddddddd	 
    (vexp2_q $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0b00010100",
            "\n.byte 0b11010000",
        )
    };

    // vlog2.s 1101 0000 0 0010101 0 sssssss 0 ddddddd	 
    (vlog2_s $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0b00010101",
            "\n.byte 0b11010000",
        )
    };

    // vlog2.p 1101 0000 0 0010101 0 sssssss 1 ddddddd	 
    (vlog2_p $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0b00010101",
            "\n.byte 0b11010000",
        )
    };

    // vlog2.t 1101 0000 0 0010101 1 sssssss 0 ddddddd	 
    (vlog2_t $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0b00010101",
            "\n.byte 0b11010000",
        )
    };

    // vlog2.q 1101 0000 0 0010101 1 sssssss 1 ddddddd	 
    (vlog2_q $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0b00010101",
            "\n.byte 0b11010000",
        )
    };

    // vsqrt.s 1101 0000 0 0010110 0 sssssss 0 ddddddd	 
    (vsqrt_s $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0b00010110",
            "\n.byte 0b11010000",
        )
    };

    // vsqrt.p 1101 0000 0 0010110 0 sssssss 1 ddddddd	 
    (vsqrt_p $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0b00010110",
            "\n.byte 0b11010000",
        )
    };

    // vsqrt.t 1101 0000 0 0010110 1 sssssss 0 ddddddd	 
    (vsqrt_t $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0b00010110",
            "\n.byte 0b11010000",
        )
    };

    // vsqrt.q 1101 0000 0 0010110 1 sssssss 1 ddddddd	 
    (vsqrt_q $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0b00010110",
            "\n.byte 0b11010000",
        )
    };

    // vasin.s 1101 0000 0 0010111 0 sssssss 0 ddddddd	 
    (vasin_s $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0b00010111",
            "\n.byte 0b11010000",
        )
    };

    // vasin.p 1101 0000 0 0010111 0 sssssss 1 ddddddd	 
    (vasin_p $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0b00010111",
            "\n.byte 0b11010000",
        )
    };

    // vasin.t 1101 0000 0 0010111 1 sssssss 0 ddddddd	 
    (vasin_t $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0b00010111",
            "\n.byte 0b11010000",
        )
    };

    // vasin.q 1101 0000 0 0010111 1 sssssss 1 ddddddd	 
    (vasin_q $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0b00010111",
            "\n.byte 0b11010000",
        )
    };

    // vnrcp.s 1101 0000 0 0011000 0 sssssss 0 ddddddd	 
    (vnrcp_s $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0b00011000",
            "\n.byte 0b11010000",
        )
    };

    // vnrcp.p 1101 0000 0 0011000 0 sssssss 1 ddddddd	 
    (vnrcp_p $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0b00011000",
            "\n.byte 0b11010000",
        )
    };

    // vnrcp.t 1101 0000 0 0011000 1 sssssss 0 ddddddd	 
    (vnrcp_t $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0b00011000",
            "\n.byte 0b11010000",
        )
    };

    // vnrcp.q 1101 0000 0 0011000 1 sssssss 1 ddddddd	 
    (vnrcp_q $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0b00011000",
            "\n.byte 0b11010000",
        )
    };

    // vnsin.s 1101 0000 0 0011010 0 sssssss 0 ddddddd	 
    (vnsin_s $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0b00011010",
            "\n.byte 0b11010000",
        )
    };

    // vnsin.p 1101 0000 0 0011010 0 sssssss 1 ddddddd	 
    (vnsin_p $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0b00011010",
            "\n.byte 0b11010000",
        )
    };

    // vnsin.t 1101 0000 0 0011010 1 sssssss 0 ddddddd	 
    (vnsin_t $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0b00011010",
            "\n.byte 0b11010000",
        )
    };

    // vnsin.q 1101 0000 0 0011010 1 sssssss 1 ddddddd
    (vnsin_q $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0b00011010",
            "\n.byte 0b11010000",
        )
    };

    // vrexp2.s 1101 0000 0 0011100 0 sssssss 0 ddddddd	 
    (vrexp2_s $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0b00011100",
            "\n.byte 0b11010000",
        )
    };

    // vrexp2.p 1101 0000 0 0011100 0 sssssss 1 ddddddd	 
    (vrexp2_p $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0b00011100",
            "\n.byte 0b11010000",
        )
    };

    // vrexp2.t 1101 0000 0 0011100 1 sssssss 0 ddddddd	 
    (vrexp2_t $s:ident, $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0b00011100",
            "\n.byte 0b11010000",
        )
    };

    // vrexp2.q 1101 0000 0 0011100 1 sssssss 1 ddddddd	 
    (vrexp2_q $s:ident, $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0b00011100",
            "\n.byte 0b11010000",
        )
    };

    // vi2uc.q 1101 0000 0 0111100 1 sssssss 1 ddddddd
    (vi2uc_q $d:ident, $s:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0b00111100",
            "\n.byte 0b11010000",
        )
    };

    // vi2s.p 1101 0000 0 0111111 0 sssssss 1 ddddddd
    (vi2s_p $d:ident, $s:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0b00111111",
            "\n.byte 0b11010000",
        )
    };

    // vi2s.q 1101 0000 0 0111111 1 sssssss 1 ddddddd
    (vi2s_q $d:ident, $s:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0b00111111",
            "\n.byte 0b11010000",
        )
    };

    // vsgn.s 1101 0000 0 1001010 0 sssssss 0 ddddddd
    (vsgn_s $d:ident, $s:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0b01001010",
            "\n.byte 0b11010000",
        )
    };

    // vsgn.p 1101 0000 0 1001010 0 sssssss 1 ddddddd
    (vsgn_p $d:ident, $s:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0b01001010",
            "\n.byte 0b11010000",
        )
    };

    // vsgn.t 1101 0000 0 1001010 1 sssssss 0 ddddddd
    (vsgn_t $d:ident, $s:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0b01001010",
            "\n.byte 0b11010000",
        )
    };

    // vsgn.q 1101 0000 0 1001010 1 sssssss 1 ddddddd
    (vsgn_q $d:ident, $s:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0b01001010",
            "\n.byte 0b11010000",
        )
    };

    // vcst.s 1101 0000 0 11aaaaa 0 0000000 0 ddddddd	 
    (vcst_s $d:ident, $a:ident) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte 0b00000000",
            "\n.byte 0b01100000 | ", $crate::vfpu_const!($a),
            "\n.byte 0b11010000",
        )
    };

    // vcst.p 1101 0000 0 11aaaaa 0 0000000 1 ddddddd	 
    (vcst_p $d:ident, $a:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte 0b00000000",
            "\n.byte 0b01100000 | ", $crate::vfpu_const!($a),
            "\n.byte 0b11010000",
        )
    };

    // vcst.t 1101 0000 0 11aaaaa 1 0000000 0 ddddddd	 
    (vcst_t $d:ident, $a:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80",
            "\n.byte 0b01100000 | ", $crate::vfpu_const!($a),
            "\n.byte 0b11010000",
        )
    };

    // vcst.q 1101 0000 0 11aaaaa 1 0000000 1 ddddddd	 
    (vcst_q $d:ident, $a:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80",
            "\n.byte 0b01100000 | ", $crate::vfpu_const!($a),
            "\n.byte 0b11010000",
        )
    };

    // Float to int, rounds to nearest
    // vf2in.s 1101 0010 0 SSSSSSS 0 sssssss 0 ddddddd
    (vf2in_s $d:ident, $s:ident, $scale:expr) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte ", stringify!($scale),
            "\n.byte 0b11010010",
        )
    };

    // vf2in.p 1101 0010 0 SSSSSSS 0 sssssss 1 ddddddd	 
    (vf2in_p $d:ident, $s:ident, $scale:expr) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte ", stringify!($scale),
            "\n.byte 0b11010010",
        )
    };

    // vf2in.t 1101 0010 0 SSSSSSS 1 sssssss 0 ddddddd	 
    (vf2in_t $d:ident, $s:ident, $scale:expr) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte ", stringify!($scale),
            "\n.byte 0b11010010",
        )
    };

    // vf2in.q 1101 0010 0 SSSSSSS 1 sssssss 1 ddddddd	 
    (vf2in_q $d:ident, $s:ident, $scale:expr) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte ", stringify!($scale),
            "\n.byte 0b11010010",
        )
    };

    // vi2f.s 1101 0010 1 SSSSSSS 0 sssssss 0 ddddddd
    (vi2f_s $d:ident, $s:ident, $scale:expr) => {
        concat!(
            "\n.byte ", $crate::register_single!($d),
            "\n.byte ", $crate::register_single!($s),
            "\n.byte 0x80 | ", stringify!($scale),
            "\n.byte 0b11010010",
        )
    };

    // vi2f.p 1101 0010 1 SSSSSSS 0 sssssss 1 ddddddd	 
    (vi2f_p $d:ident, $s:ident, $scale:expr) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_pair!($s),
            "\n.byte 0x80 | ", stringify!($scale),
            "\n.byte 0b11010010",
        )
    };

    // vi2f.t 1101 0010 1 SSSSSSS 1 sssssss 0 ddddddd	 
    (vi2f_t $d:ident, $s:ident, $scale:expr) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_triple!($s),
            "\n.byte 0x80 | ", stringify!($scale),
            "\n.byte 0b11010010",
        )
    };

    // vi2f.q 1101 0010 1 SSSSSSS 1 sssssss 1 ddddddd	 
    (vi2f_q $d:ident, $s:ident, $scale:expr) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_quad!($s),
            "\n.byte 0x80 | ", stringify!($scale),
            "\n.byte 0b11010010",
        )
    };

    // vmmul.p 1111 0000 0 ttttttt 0 sSsssss 1 ddddddd (*inverted 5th S bit)
    (vmmul_p $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_mpair!($d),
            "\n.byte ", $crate::register_mpair!($s), " ^ 0b0100000",
            "\n.byte ", $crate::register_mpair!($t),
            "\n.byte 0b11110000",
        )
    };

    // vmmul.t 1111 0000 0 ttttttt 1 sSsssss 0 ddddddd (*inverted 5th S bit)
    (vmmul_t $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_mtriple!($d),
            "\n.byte 0x80 | ", $crate::register_mtriple!($s), " ^ 0b0100000",
            "\n.byte ", $crate::register_mtriple!($t),
            "\n.byte 0b11110000",
        )
    };

    // vmmul.q 1111 0000 0 ttttttt 1 sSsssss 1 ddddddd (*inverted 5th S bit)
    (vmmul_q $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_mquad!($d),
            "\n.byte 0x80 | ", $crate::register_mquad!($s), " ^ 0b0100000",
            "\n.byte ", $crate::register_mquad!($t),
            "\n.byte 0b11110000",
        )
    };

    // vhtfm2.p 1111 0000 1 ttttttt 0 sssssss 0 ddddddd	 
    (vhtfm2_p $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_mpair!($s),
            "\n.byte 0x80 | ", $crate::register_pair!($t),
            "\n.byte 0b11110000",
        )
    };

    // vtfm2.p 1111 0000 1 ttttttt 0 sssssss 1 ddddddd	 
    (vtfm2_p $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_pair!($d),
            "\n.byte ", $crate::register_mpair!($s),
            "\n.byte 0x80 | ", $crate::register_pair!($t),
            "\n.byte 0b11110000",
        )
    };

    // vhtfm3.t 1111 0001 0 ttttttt 0 sssssss 1 ddddddd	 
    (vhtfm3_t $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_triple!($d),
            "\n.byte ", $crate::register_mtriple!($s),
            "\n.byte ", $crate::register_triple!($t),
            "\n.byte 0b11110001",
        )
    };

    // vtfm3.t 1111 0001 0 ttttttt 1 sssssss 0 ddddddd	 
    (vtfm3_t $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_triple!($d),
            "\n.byte 0x80 | ", $crate::register_mtriple!($s),
            "\n.byte ", $crate::register_triple!($t),
            "\n.byte 0b11110001",
        )
    };

    // vhtfm4.q 1111 0001 1 ttttttt 1 sssssss 0 ddddddd	 
    (vhtfm4_q $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_mquad!($s),
            "\n.byte 0x80 | ", $crate::register_quad!($t),
            "\n.byte 0b11110001",
        )
    };

    // vtfm4.q 1111 0001 1 ttttttt 1 sssssss 1 ddddddd	 
    (vtfm4_q $d:ident, $s:ident, $t:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_quad!($d),
            "\n.byte 0x80 | ", $crate::register_mquad!($s),
            "\n.byte 0x80 | ", $crate::register_quad!($t),
            "\n.byte 0b11110001",
        )
    };

    // vmidt.p 1111 0011 1 0000011 0 0000000 1 ddddddd
    (vmidt_p $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_mpair!($d),
            "\n.byte 0b00000000",
            "\n.byte 0b10000011",
            "\n.byte 0b11110011",
        )
    };

    // vmidt.t 1111 0011 1 0000011 1 0000000 0 ddddddd
    (vmidt_t $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_mtriple!($d),
            "\n.byte 0x80",
            "\n.byte 0b10000011",
            "\n.byte 0b11110011",
        )
    };

    // vmidt.q 1111 0011 1 0000011 1 0000000 1 ddddddd
    (vmidt_q $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_mquad!($d),
            "\n.byte 0x80",
            "\n.byte 0b10000011",
            "\n.byte 0b11110011",
        )
    };

    // vmzero.p 1111 0011 1 0000110 0 0000000 1 ddddddd
    (vmzero_p $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_mpair!($d),
            "\n.byte 0b00000000",
            "\n.byte 0b10000110",
            "\n.byte 0b11110011",
        )
    };

    // vmzero.t 1111 0011 1 0000110 1 0000000 0 ddddddd
    (vmzero_t $d:ident) => {
        concat!(
            "\n.byte ", $crate::register_mtriple!($d),
            "\n.byte 0x80",
            "\n.byte 0b10000110",
            "\n.byte 0b11110011",
        )
    };

    // vmzero.q 1111 0011 1 0000110 1 0000000 1 ddddddd
    (vmzero_q $d:ident) => {
        concat!(
            "\n.byte 0x80 | ", $crate::register_mquad!($d),
            "\n.byte 0x80",
            "\n.byte 0b10000110",
            "\n.byte 0b11110011",
        )
    };

    // Raw MIPS assembly code.
    (mips $expr:expr) => {
        concat!("\n", $expr, "\n")
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! register_mips {
    (zero) => {"0"};
    (at) => {"1"};

    (v0) => {"2"};
    (v1) => {"3"};

    (a0) => {"4"};
    (a1) => {"5"};
    (a2) => {"6"};
    (a3) => {"7"};

    (t0) => {"8"};
    (t1) => {"9"};
    (t2) => {"10"};
    (t3) => {"11"};
    (t4) => {"12"};
    (t5) => {"13"};
    (t6) => {"14"};
    (t7) => {"15"};

    (s0) => {"16"};
    (s1) => {"17"};
    (s2) => {"18"};
    (s3) => {"19"};
    (s4) => {"20"};
    (s5) => {"21"};
    (s6) => {"22"};
    (s7) => {"23"};

    (t8) => {"24"};
    (t9) => {"25"};

    (k0) => {"26"};
    (k1) => {"27"};

    (gp) => {"28"};
    (sp) => {"29"};
    (fp) => {"30"};
    (ra) => {"31"};
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
}
