#![no_std]
#![no_main]

use psp::sys::{
    sceAudioChReserve, sceAudioOutput, sceAudioOutputBlocking,
    sceAudioSetChannelDataLen, sceAudioGetChannelRestLen,
};

psp::module!("sound_sample", 1, 1);

// In a real scenario you would probably load this out of a file at runtime
// I leave that as an exercise to the user ;)
// 44.1KHz signed 16-bit PCM in RAW (no header) format 
static AUDIO_CLIP: [u8; 718828] = *include_bytes!("../assets/showtime.rawpcm16"); 

const MAX_VOL: i32 = 0x8000;
const MAX_SAMPLES: usize = 65472;
const CHANNEL: i32 = 0;

fn psp_main() {
    psp::enable_home_button();
    unsafe { 
        sceAudioChReserve(CHANNEL, MAX_SAMPLES as i32, psp::sys::AudioFormat::Stereo) 
    };
    let mut start_pos: usize = 0;
    let mut restlen = 0;
    while (start_pos+MAX_SAMPLES*4) < AUDIO_CLIP.len() {
        if restlen > 0 {
            psp::dprintln!("Check it out, I can do other stuff while audio plays!");
        } else {
            unsafe {
                sceAudioOutput(
                    CHANNEL,
                    MAX_VOL,
                    AUDIO_CLIP.as_ptr().add(start_pos) as *mut _
                )
            };
            start_pos += MAX_SAMPLES*4;
        }
        restlen = unsafe { sceAudioGetChannelRestLen(CHANNEL) };
    }
    let remainder: i32 = (((AUDIO_CLIP.len() % (MAX_SAMPLES*4)/4)+63) & !63) as i32;
    unsafe { sceAudioSetChannelDataLen(CHANNEL, remainder)};
    unsafe {
        // Blocking here so the program doesn't exit while we're finishing
        // up outputting audio
        sceAudioOutputBlocking(
            CHANNEL,
            MAX_VOL,
            AUDIO_CLIP.as_ptr().add(start_pos) as *mut _)
    };
}
