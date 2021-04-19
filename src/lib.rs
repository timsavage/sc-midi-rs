mod read;
mod timecode;

#[allow(non_camel_case_types)]
pub type u3 = u8;
#[allow(non_camel_case_types)]
pub type u4 = u8;
#[allow(non_camel_case_types)]
pub type u7 = u8;
pub type Channel = u7;
#[allow(non_camel_case_types)]
pub type i14 = i16;
#[allow(non_camel_case_types)]
pub type u14 = u16;

pub use read::{MidiEvent, MidiReader, SysExID};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
