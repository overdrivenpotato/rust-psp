use psp::sys::{sceAudioOutput, sceAudioSetChannelDataLen, sceAudioGetChannelRestLen};

static TETRIS_SONG: [u8; 3402490] = *include_bytes!("../assets/tetris.pcm.raw"); 

const MAX_VOL: i32 = 0x8000;
pub const MAX_SAMPLES: usize = 65472;

/// Called once per loop of the game, handles audio.
///
/// # Parameters
/// - `channel`: An audio channel initialized by `sceAudioChReserve`
/// - `start_pos`: The starting position from which to play audio
/// - `restlen`: How much audio remains to be played
///
/// # Return Value
///
/// `(restlen, start_pos)`
pub unsafe fn process_audio_loop(channel: i32, mut start_pos: usize, mut restlen: i32) -> (i32, usize) {
    if (start_pos+MAX_SAMPLES*2) < TETRIS_SONG.len() {
        if restlen == 0 {
                sceAudioOutput(
                    channel,
                    MAX_VOL,
                    TETRIS_SONG.as_ptr().add(start_pos) as *mut _
                );
            start_pos += MAX_SAMPLES*2;
        }
    } else {
        let remainder: i32 = (((TETRIS_SONG.len() % (MAX_SAMPLES*2)/2)+63) & !63) as i32;
        if restlen == 0 {
            sceAudioSetChannelDataLen(channel, remainder);
            sceAudioOutput(
                channel,
                MAX_VOL,
                TETRIS_SONG.as_ptr().add(start_pos) as *mut _
            );
            start_pos += (remainder*2) as usize;
        }
        if start_pos >= TETRIS_SONG.len() {
            start_pos = 0;
            sceAudioSetChannelDataLen(channel, MAX_SAMPLES as i32);
        }
    }

    restlen = sceAudioGetChannelRestLen(channel);
    (restlen, start_pos)
}
