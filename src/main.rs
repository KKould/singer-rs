mod error;

use crate::error::SingerError;
use rodio::{OutputStream, OutputStreamHandle, Sink};
use std::io::{stdin, BufReader, Write};
use std::path::Path;

struct MusicHandler {
    _stream: OutputStream,
    _handler: OutputStreamHandle,
    sink: Sink,
}

fn play<P: AsRef<Path>>(path: P) -> Result<MusicHandler, SingerError> {
    let (stream, handler) = OutputStream::try_default()?;
    let sink = Sink::try_new(&handler)?;

    let file = std::fs::File::open(path)?;
    sink.append(rodio::Decoder::new(BufReader::new(file))?);

    Ok(MusicHandler {
        _stream: stream,
        _handler: handler,
        sink,
    })
}

fn main() -> Result<(), SingerError> {
    let mut song = None;

    loop {
        print!("🎤 ");
        std::io::stdout().flush()?;

        let mut input = String::new();
        stdin().read_line(&mut input)?;

        for command in input.trim().split(" | ") {
            let mut parts = command.split_whitespace();

            if let Some(command) = parts.next() {
                let mut args = parts.peekable();

                match command {
                    "play" => {
                        if let Some(path) = args.peek() {
                            song = Some(play(path)?);
                        } else if let Some(handler) = song.as_ref() {
                            handler.sink.play();
                        }
                    }
                    "pause" => {
                        if let Some(handler) = song.as_ref() {
                            handler.sink.pause();
                        }
                    }
                    "stop" => {
                        if let Some(handler) = song.take() {
                            drop(handler);
                        }
                    }
                    "exit" | "quit" => return Ok(()),
                    _ => (),
                }
            }
        }
    }
}
