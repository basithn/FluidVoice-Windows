use std::io::Cursor;
use rodio::{Decoder, OutputStream, Sink, Source};
use std::sync::{Arc, Mutex};

// Embed the sounds into the binary
// Note: We'll create dummy wav files if they don't exist yet, 
// or generate them programmatically to stay self-contained.
// For MVP simplicity, we will generate a simple sine wave beep.

pub struct AudioFeedback {
    _stream: OutputStream,
    stream_handle: rodio::OutputStreamHandle,
}

impl AudioFeedback {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        Self { _stream, stream_handle }
    }

    pub fn play_start(&self) {
        self.play_tone(440.0, 0.1); // A4, short
    }

    pub fn play_stop(&self) {
        self.play_tone(330.0, 0.1); // E4, short
        std::thread::sleep(std::time::Duration::from_millis(100));
        self.play_tone(220.0, 0.2); // A3, longer
    }

    pub fn play_error(&self) {
        self.play_tone(150.0, 0.3); // Low tone
    }

    fn play_tone(&self, freq: f32, duration_secs: f32) {
        let sink = Sink::try_new(&self.stream_handle).unwrap();
        let source = rodio::source::SineWave::new(freq)
            .take_duration(std::time::Duration::from_secs_f32(duration_secs))
            .amplify(0.2); // Volume at 20%
        sink.append(source);
        sink.detach(); // Fire and forget
    }
}
