mod error;

use crate::error::SingerError;
use regex::Regex;
use rodio::{OutputStream, OutputStreamHandle, Sink};
use std::io::{stdin, BufReader, Write};

pub(crate) const BANNER: &str = "
 .oooooo..o  o8o                                                    ooooooooo.    .oooooo..o
d8P'    `Y8  `\"'                                                    `888   `Y88. d8P'    `Y8
Y88bo.      oooo  ooo. .oo.    .oooooooo  .ooooo.  oooo d8b          888   .d88' Y88bo.
 `\"Y8888o.  `888  `888P\"Y88b  888' `88b  d88' `88b `888\"\"8P          888ooo88P'   `\"Y8888o.
     `\"Y88b  888   888   888  888   888  888ooo888  888     8888888  888`88b.         `\"Y88b
oo     .d8P  888   888   888  `88bod8P'  888    .o  888              888  `88b.  oo     .d8P
8\"\"88888P'  o888o o888o o888o `8oooooo.  `Y8bod8P' d888b            o888o  o888o 8\"\"88888P'
                              d\"     YD
                              \"Y88888P'


";

struct MusicHandler {
    _stream: OutputStream,
    _handler: OutputStreamHandle,
    sink: Sink,
}

fn play(path: &str) -> Result<MusicHandler, SingerError> {
    let regex = Regex::new(r#"^"(.*)"$"#).unwrap();

    let (stream, handler) = OutputStream::try_default()?;
    let sink = Sink::try_new(&handler)?;

    let file = std::fs::File::open(regex.replace(path, "$1").as_ref())?;
    sink.append(rodio::Decoder::new(BufReader::new(file))?);

    Ok(MusicHandler {
        _stream: stream,
        _handler: handler,
        sink,
    })
}

fn main() -> Result<(), SingerError> {
    println!("{} \nVersion: {}\n", BANNER, env!("CARGO_PKG_VERSION"));
    let mut song = None;

    loop {
        print!("🎤 ");
        std::io::stdout().flush()?;

        let mut input = String::new();
        stdin().read_line(&mut input)?;

        for command in input.trim().split(" | ") {
            let regex = Regex::new(r"\s+").unwrap();
            let mut parts = regex.splitn(command, 2);

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
