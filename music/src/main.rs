extern crate rodio;

use rodio::source::SineWave;
use rodio::Sink;
use rodio::Source;  
use std::time::Duration;

fn main() {
    // let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    // let sink = Sink::try_new(&stream_handle).unwrap();

    // let notes = [
    //     261.63, // ド C4
    //     293.66, // レ D4
    //     329.63, // ミ E4
    //     349.23, // ファ F4
    //     392.00, // ソ G4
    //     440.00, // ラ A4
    //     493.88, // シ B4
    //     523.25, // ド C5
    // ];

    // for &freq in &notes {
    //     let source = SineWave::new(freq).take_duration(Duration::from_secs(1));
    //     sink.append(source);
    //     sink.sleep_until_end();
    // }
    test1();
}

fn test1() {
    let (stream, stream_handle) = rodio::OutputStream::try_default().expect("Failed to get the default audio output device");
    let sink = Sink::try_new(&stream_handle).unwrap();

    let notes = [
        261.63, // ド C4
        261.63, // ド
        293.66, // レ D4
        293.66, // レ
        329.63, // ミ E4
        329.63, // ミ
        293.66, // レ
        261.63, // ド C4
        261.63, // ド
        392.00, // ソ G4
        392.00, // ソ
        349.23, // ファ F4
        349.23, // ファ
        329.63, // ミ E4
        293.66, // レ D4
        293.66, // レ
        261.63, // ド C4
    ];

    let durations = [
        Duration::from_millis(500),
        Duration::from_millis(500),
        Duration::from_millis(500),
        Duration::from_millis(500),
        Duration::from_millis(500),
        Duration::from_millis(500),
        Duration::from_millis(1000),
        Duration::from_millis(500),
        Duration::from_millis(500),
        Duration::from_millis(500),
        Duration::from_millis(500),
        Duration::from_millis(500),
        Duration::from_millis(500),
        Duration::from_millis(500),
        Duration::from_millis(500),
        Duration::from_millis(500),
        Duration::from_millis(1000),
    ];

    for (freq, &duration) in notes.iter().zip(&durations) {
        let source = SineWave::new(*freq).take_duration(duration);
        sink.append(source);
        sink.sleep_until_end();
    }
}
