extern crate rodio;

use rodio::source::SineWave;
use rodio::Sink;
use rodio::Source;  
use std::time::Duration;

fn main() {
    let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let notes = [
        261.63, // ド C4
        293.66, // レ D4
        329.63, // ミ E4
        349.23, // ファ F4
        392.00, // ソ G4
        440.00, // ラ A4
        493.88, // シ B4
        523.25, // ド C5
    ];

    for &freq in &notes {
        let source = SineWave::new(freq).take_duration(Duration::from_secs(1));
        sink.append(source);
        sink.sleep_until_end();
    }
}
