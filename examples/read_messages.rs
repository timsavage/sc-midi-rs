extern crate sc_midi;

use std::fs::File;
use std::io;
use std::io::{Read, Write};

use sc_midi::MidiEvent;
use std::borrow::Borrow;

fn main() -> io::Result<()> {
    let mut midi_reader = sc_midi::MidiReader::new();
    let mut file = File::open("/dev/midi2")?;

    loop {
        let mut buf = [0u8; 16];

        let count = file.read(&mut buf)?;
        if count > 0 {
            for byte in buf[..count].iter() {
                midi_reader
                    .handle_byte(*byte)
                    .filter(|event| match event {
                        MidiEvent::PitchBend(_, _) => false,
                        _ => true,
                    })
                    .map(|event| println!("{}", event));
            }
        }
    }
}
