//! Debug support.
//!
//! You should use the `dprintln!` and `dprint!` macros.

use crate::sys;
use core::fmt;

/// Like `println!`, but prints to the PSP screen.
#[macro_export]
macro_rules! dprintln {
    () => {
        $crate::dprint("\n")
    };
    ($($arg:tt)*) => {{
        $crate::dprint!($($arg)*);
        $crate::dprint!("\n");
    }};
}

/// Like `print!`, but prints to the PSP screen.
#[macro_export]
macro_rules! dprint {
    ($($arg:tt)*) => {{
        $crate::debug::print_args(core::format_args!($($arg)*))
    }}
}

// TODO: Wrap this in some kind of a mutex.
static mut CHARS: CharBuffer = CharBuffer::new();

/// Update the screen.
fn update() {
    unsafe {
        init();
        clear_screen(0);

        for (i, line) in CHARS.lines().enumerate() {
            put_str::<MsxFont>(
                &line.chars[0..line.len],
                0,
                i * MsxFont::CHAR_HEIGHT,
                0xffff_ffff,
            )
        }
    }
}

trait Font {
    const CHAR_WIDTH: usize;
    const CHAR_HEIGHT: usize;

    fn put_char(x: usize, y: usize, color: u32, c: u8);
}

struct MsxFont;

impl Font for MsxFont {
    const CHAR_HEIGHT: usize = 10;
    const CHAR_WIDTH: usize = 6;

    fn put_char(x: usize, y: usize, color: u32, c: u8) {
        unsafe {
            let mut ptr = VRAM_BASE.add(x + y * BUFFER_WIDTH);

            for i in 0..8 {
                for j in 0..8 {
                    if MSX_FONT[c as usize * 8 + i] & (0b1000_0000 >> j) != 0 {
                        *ptr = color;
                    }

                    ptr = ptr.offset(1);
                }

                ptr = ptr.add(BUFFER_WIDTH - 8);
            }
        }
    }
}

const BUFFER_WIDTH: usize = 512;
const DISPLAY_HEIGHT: usize = 272;
const DISPLAY_WIDTH: usize = 480;
static mut VRAM_BASE: *mut u32 = 0 as *mut u32;

unsafe fn clear_screen(color: u32) {
    let mut ptr = VRAM_BASE;

    for _ in 0..(BUFFER_WIDTH * DISPLAY_HEIGHT) {
        *ptr = color;
        ptr = ptr.offset(1);
    }
}

unsafe fn put_str<T: Font>(s: &[u8], x: usize, y: usize, color: u32) {
    if y > DISPLAY_HEIGHT {
        return;
    }

    for (i, c) in s.iter().enumerate() {
        if i >= (DISPLAY_WIDTH / T::CHAR_WIDTH) {
            break;
        }

        if *c as u32 <= 255 && *c != b'\0' {
            T::put_char(T::CHAR_WIDTH * i + x, y, color, *c);
        }
    }
}

unsafe fn init() {
    // The OR operation here specifies the address bypasses cache.
    VRAM_BASE = (0x4000_0000u32 | sys::sceGeEdramGetAddr() as u32) as *mut u32;

    // TODO: Change sys types to usize.
    sys::sceDisplaySetMode(sys::DisplayMode::Lcd, DISPLAY_WIDTH, DISPLAY_HEIGHT);
    sys::sceDisplaySetFrameBuf(
        VRAM_BASE as *const u8,
        BUFFER_WIDTH,
        sys::DisplayPixelFormat::Psm8888,
        sys::DisplaySetBufSync::NextFrame,
    );
}

#[doc(hidden)]
pub fn print_args(arguments: core::fmt::Arguments<'_>) {
    use fmt::Write;

    unsafe {
        let _ = write!(CHARS, "{}", arguments);
    }

    update();
}

// TODO: Move to font.
const ROWS: usize = DISPLAY_HEIGHT / MsxFont::CHAR_HEIGHT;
const COLS: usize = DISPLAY_WIDTH / MsxFont::CHAR_WIDTH;

#[derive(Copy, Clone)]
struct Line {
    chars: [u8; COLS],
    len: usize,
}

impl Line {
    const fn new() -> Self {
        Self {
            chars: [0; COLS],
            len: 0,
        }
    }
}

struct CharBuffer {
    lines: [Line; ROWS],
    written: usize,
    advance_next: bool,
}

impl CharBuffer {
    const fn new() -> Self {
        Self {
            lines: [Line::new(); ROWS],
            written: 0,
            advance_next: false,
        }
    }

    fn advance(&mut self) {
        self.written += 1;
        if self.written >= ROWS {
            *self.current_line() = Line::new();
        }
    }

    fn current_line(&mut self) -> &mut Line {
        &mut self.lines[self.written % ROWS]
    }

    fn add(&mut self, c: u8) {
        if self.advance_next {
            self.advance_next = false;
            self.advance();
        }

        match c {
            b'\n' => self.advance_next = true,
            b'\t' => {
                self.add(b' ');
                self.add(b' ');
                self.add(b' ');
                self.add(b' ');
            }

            _ => {
                if self.current_line().len == COLS {
                    self.advance();
                }

                let line = self.current_line();
                line.chars[line.len] = c;
                line.len += 1;
            }
        }
    }

    fn lines(&self) -> LineIter<'_> {
        LineIter { buf: self, pos: 0 }
    }
}

impl fmt::Write for CharBuffer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe {
            for c in s.chars() {
                match c as u32 {
                    0..=255 => CHARS.add(c as u8),
                    _ => CHARS.add(0),
                }
            }
        }

        Ok(())
    }
}

struct LineIter<'a> {
    buf: &'a CharBuffer,
    pos: usize,
}

impl<'a> Iterator for LineIter<'a> {
    type Item = Line;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < core::cmp::min(self.buf.written + 1, ROWS) {
            let idx = if self.buf.written > ROWS {
                (self.buf.written + 1 + self.pos) % ROWS
            } else {
                self.pos
            };

            let line = self.buf.lines[idx];
            self.pos += 1;
            Some(line)
        } else {
            None
        }
    }
}

/// Raw MSX font.
///
/// This is an 8bit x 256 black and white image.
const MSX_FONT: [u8; 2048] = *include_bytes!("msxfont.bin");
