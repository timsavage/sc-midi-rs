extern crate sc_midi;

use std::io;
use std::fs::File;
use std::io::Read;

fn main() -> io::Result<()> {
    let mut midi_reader = sc_midi::MidiReader::new();
    let mut file = File::open("/dev/midi2")?;

    loop {
        let mut buf = [0u8; 1];
        if file.read(&mut buf)? > 0 {
            midi_reader.handle_byte(buf[0]).map(|event| println!("{}" ,event));
        }
    }

    Ok(())
}