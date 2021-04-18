mod messages;
mod read;

pub use read::{
    SysExID,
    MidiEvent,
    u14, u3, u4, u7,
    MidiReader
};


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
