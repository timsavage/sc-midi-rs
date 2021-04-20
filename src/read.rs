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

const STATUS_MASK: u8 = 0b1000_0000;
const CHANNEL_MASK: u8 = 0b0000_1111;
const SYSTEM_MASK: u8 = 0b1111_0000;
const SYSTEM_RT_MASK: u8 = 0b1111_1000;

const MESSAGE_NONE: u8 = 0x00;
// Channel Voice Messages
const MESSAGE_NOTE_OFF: u8 = 0x80;
const MESSAGE_NOTE_ON: u8 = 0x90;
const MESSAGE_POLY_AFTER_TOUCH: u8 = 0xA0; // Polyphonic AfterTouch
const MESSAGE_CONTROLLER_CHANGE: u8 = 0xB0; // Controller Change / Channel Mode
const MESSAGE_PROGRAM_CHANGE: u8 = 0xC0;
const MESSAGE_CHANNEL_AFTER_TOUCH: u8 = 0xD0;
const MESSAGE_PITCH_BEND: u8 = 0xE0;
// System Common Messages
const MESSAGE_SYS_EX_START: u8 = 0xF0; // System Exclusive Start
const MESSAGE_MTC_QUARTER_FRAME: u8 = 0xF1; // Time Code Quarter Frame
const MESSAGE_SONG_POSITION_PTR: u8 = 0xF2; // Song Position Pointer
const MESSAGE_SONG_SELECT: u8 = 0xF3;
const MESSAGE_TUNE_REQUEST: u8 = 0xF6;
const MESSAGE_SYS_EX_END: u8 = 0xF7; // System Exclusive End
                                     // System Realtime Messages
const MESSAGE_CLOCK: u8 = 0xF8;
const MESSAGE_START: u8 = 0xFA;
const MESSAGE_CONTINUE: u8 = 0xFB;
const MESSAGE_STOP: u8 = 0xFC;
const MESSAGE_ACTIVE_SENSING: u8 = 0xFE;
const MESSAGE_SYSTEM_RESET: u8 = 0xFF;

// Channel Mode messages
const CHANNEL_MODE_ALL_SOUND_OFF: u7 = 0x78;
const CHANNEL_MODE_RESET_ALL: u7 = 0x79;
const CHANNEL_MODE_LOCAL_CONTROL: u7 = 0x7A;
const CHANNEL_MODE_ALL_NOTES_OFF: u7 = 0x7B;
const CHANNEL_MODE_OMNI_ON: u7 = 0x7C;
const CHANNEL_MODE_OMNI_OFF: u7 = 0x7D;
const CHANNEL_MODE_MONO_ON: u7 = 0x7E;
const CHANNEL_MODE_POLYPHONIC_ON: u7 = 0x7F;

///
/// Size of System Exclusive ID
///
#[derive(Debug, Eq, PartialEq)]
pub enum SysExID {
    Byte(u7),
    Word(u14),
}

///
/// Generated Midi events
///
#[derive(Debug, Eq, PartialEq)]
pub enum MidiEvent {
    // Channel events
    NoteOff(Channel, u7, u7),              // Channel, Key, Velocity
    AllNotesOff(Channel),                  // Channel
    NoteOn(Channel, u7, u7),               // Channel, Key, Velocity
    PolyphonicAfterTouch(Channel, u7, u7), // Channel, Key, Pressure
    ControllerChange(Channel, u7, u7),     // Channel, Control, Value
    AllSoundOff,
    ResetAllControllers,
    LocalControl(Channel, bool),    // Channel, On
    OmniMode(Channel, bool),        // Channel, On
    MonoMode(Channel, u7),          // Channel, Num Channels
    PolyphonicMode(Channel),        // Channel
    ProgramChange(Channel, u7),     // Channel, Program Num
    ChannelAfterTouch(Channel, u7), // Channel, Pressure
    PitchBend(Channel, i14),        // Channel, Amount
    // System Common events
    MTCQuarterFrame(u3, u4),  // Type, Value
    SongPositionPointer(u14), // MIDI beats (1 beat = 6 MIDI clocks)
    SongSelect(u7),           // Song Number
    TuneRequest,
    // System Realtime events
    Clock,
    Start,
    Continue,
    Stop,
    ActiveSensing,
    SystemReset,
    // System Exclusive events
    SystemExclusiveStart(SysExID), // SysEx ID (Byte or Word)
    SystemExclusiveData(u7),       // Data byte
    SystemExclusiveEnd,
}

impl std::fmt::Display for MidiEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MidiEvent::NoteOff(channel, key, velocity) => {
                    format!("Note Off: {}; {}; {}", channel, key, velocity)
                }
                MidiEvent::NoteOn(channel, key, velocity) => {
                    format!("Note On: {}; {}; {}", channel, key, velocity)
                }
                MidiEvent::PolyphonicAfterTouch(channel, key, pressure) => {
                    format!("Polyphonic After-Touch: {}; {}; {}", channel, key, pressure)
                }
                MidiEvent::ControllerChange(channel, control, value) => {
                    format!(
                        "Controller Change: {}; 0x{:02X}; {}",
                        channel, control, value
                    )
                }
                MidiEvent::AllSoundOff => String::from("All Sound Off"),
                MidiEvent::ResetAllControllers => String::from("Reset All Controllers"),
                MidiEvent::AllNotesOff(channel) => {
                    format!("All Notes Off: {}", channel)
                }
                MidiEvent::LocalControl(channel, on) => {
                    format!("Local Control: {}; {}", channel, on)
                }
                MidiEvent::OmniMode(channel, on) => {
                    format!("Omni Model: {}; {}", channel, on)
                }
                MidiEvent::MonoMode(channel, num_channels) => {
                    format!("MonoMode: {}; {}", channel, num_channels)
                }
                MidiEvent::PolyphonicMode(channel) => {
                    format!("PolyphonicMode: {}", channel)
                }
                MidiEvent::ProgramChange(channel, program_num) => {
                    format!("Program Change: {}; {}", channel, program_num)
                }
                MidiEvent::ChannelAfterTouch(channel, pressure) => {
                    format!("Channel After-Touch: {}; {}", channel, pressure)
                }
                MidiEvent::PitchBend(channel, amount) => {
                    format!("Pitch-Bend: {}; {}", channel, amount)
                }
                MidiEvent::MTCQuarterFrame(_, _) => String::from("MIDI Time Code Quarter Frame"),
                MidiEvent::SongPositionPointer(position) => {
                    format!("Song Position Pointer: {}", position)
                }
                MidiEvent::SongSelect(song_num) => {
                    format!("Song Select: {}", song_num)
                }
                MidiEvent::TuneRequest => String::from("Tune Request"),
                MidiEvent::Clock => String::from("Timing Clock"),
                MidiEvent::Start => String::from("Start"),
                MidiEvent::Continue => String::from("Continue"),
                MidiEvent::Stop => String::from("Stop"),
                MidiEvent::ActiveSensing => String::from("Active Sensing"),
                MidiEvent::SystemReset => String::from("System Reset"),
                MidiEvent::SystemExclusiveStart(_) => String::from("System Exclusive Start"),
                MidiEvent::SystemExclusiveData(_) => String::from("System Exclusive Data"),
                MidiEvent::SystemExclusiveEnd => String::from("System Exclusive End"),
            }
        )
    }
}

#[inline]
fn to_u14(msb: u7, lsb: u7) -> u14 {
    ((msb as u14) << 7) | (lsb as u14)
}

#[inline]
fn to_i14(msb: u7, lsb: u7) -> i14 {
    let word = ((msb as u14) << 7) | (lsb as u14);
    word as i16 - 0x1FFF
}

///
/// State-machine message phase
///
enum Phase {
    Start,
    ByteTwo,
    ByteThree,
    SysExID,
    SysExData,
}

///
/// **Midi Reader**
///
/// Handles an inbound MIDI byte stream and returns events
///
pub struct MidiReader {
    phase: Phase,
    message: u8,   // Current message
    channel: u7,   // Current channel (for channel messages)
    data_byte: u7, // Data byte
}

impl MidiReader {
    pub fn new() -> Self {
        Self {
            phase: Phase::Start,
            message: MESSAGE_NONE,
            channel: 0,
            data_byte: 0,
        }
    }

    ///
    /// **Handle a data byte**
    ///
    /// Returns a MidiEvent if message is complete
    ///
    pub fn handle_byte(&mut self, byte: u8) -> Option<MidiEvent> {
        // Check for realtime messages
        if (byte & SYSTEM_RT_MASK) == SYSTEM_RT_MASK {
            return self.handle_system_rt_message(byte);
        }

        let is_status_byte = (byte & STATUS_MASK) > 0;
        if is_status_byte {
            // Reset Phase on status byte
            self.phase = Phase::Start;
            self.channel = 0;
            self.data_byte = 0;
        }

        match self.phase {
            Phase::Start => {
                // On reset ignore bytes until a status byte is received.
                if is_status_byte {
                    self.phase = Phase::ByteTwo;

                    if (byte & SYSTEM_MASK) == SYSTEM_MASK {
                        // Start byte is the message for system messages
                        self.message = byte;
                        return self.handle_system_message();
                    }

                    // Split start byte for channel messages
                    self.message = byte & SYSTEM_MASK;
                    self.channel = byte & CHANNEL_MASK;
                }
                None
            }
            Phase::ByteTwo => self.handle_byte_two(byte),
            Phase::ByteThree => self.handle_byte_three(byte),
            Phase::SysExID => {
                self.phase = Phase::SysExData;
                Some(MidiEvent::SystemExclusiveStart(SysExID::Word(to_u14(
                    self.data_byte,
                    byte,
                ))))
            }
            Phase::SysExData => Some(MidiEvent::SystemExclusiveData(byte)),
        }
    }

    ///
    /// **Handle System RT messages**
    ///
    /// These messages can be interleaved into the message stream and are always a single byte.
    ///
    fn handle_system_rt_message(&mut self, byte: u8) -> Option<MidiEvent> {
        match byte {
            MESSAGE_CLOCK => Some(MidiEvent::Clock),
            MESSAGE_START => Some(MidiEvent::Start),
            MESSAGE_CONTINUE => Some(MidiEvent::Continue),
            MESSAGE_STOP => Some(MidiEvent::Stop),
            MESSAGE_ACTIVE_SENSING => Some(MidiEvent::ActiveSensing),
            MESSAGE_SYSTEM_RESET => Some(MidiEvent::SystemReset),
            _ => None,
        }
    }

    ///
    /// **Handle System (One byte) messages**
    ///
    fn handle_system_message(&mut self) -> Option<MidiEvent> {
        match self.message {
            MESSAGE_TUNE_REQUEST => {
                self.phase = Phase::Start;
                Some(MidiEvent::TuneRequest)
            }
            MESSAGE_SYS_EX_END => {
                self.phase = Phase::Start;
                Some(MidiEvent::SystemExclusiveEnd)
            }
            _ => None,
        }
    }

    ///
    /// **Handle Two byte messages**
    ///
    fn handle_byte_two(&mut self, byte: u8) -> Option<MidiEvent> {
        match self.message {
            MESSAGE_PROGRAM_CHANGE => {
                self.phase = Phase::ByteTwo;
                Some(MidiEvent::ProgramChange(self.channel, byte))
            }
            MESSAGE_CHANNEL_AFTER_TOUCH => {
                self.phase = Phase::ByteTwo;
                Some(MidiEvent::ChannelAfterTouch(self.channel, byte))
            }
            MESSAGE_MTC_QUARTER_FRAME => {
                self.phase = Phase::ByteTwo;
                Some(MidiEvent::MTCQuarterFrame(
                    (byte & 0b0111_0000) >> 4,
                    byte & 0b0000_1111,
                ))
            }
            MESSAGE_SONG_SELECT => {
                self.phase = Phase::Start;
                Some(MidiEvent::SongSelect(byte))
            }
            MESSAGE_SYS_EX_START => {
                if byte == 0 {
                    self.phase = Phase::ByteThree;
                    None
                } else {
                    // Sys Ex uses ID Byte
                    self.phase = Phase::SysExData;
                    Some(MidiEvent::SystemExclusiveStart(SysExID::Byte(byte)))
                }
            }
            _ => {
                self.phase = Phase::ByteThree;
                self.data_byte = byte;
                None
            }
        }
    }

    ///
    /// **Handle Three bte messages**
    ///
    fn handle_byte_three(&mut self, byte: u8) -> Option<MidiEvent> {
        match self.message {
            MESSAGE_NOTE_OFF => {
                self.phase = Phase::ByteTwo;
                Some(MidiEvent::NoteOff(self.channel, self.data_byte, byte))
            }
            MESSAGE_NOTE_ON => {
                self.phase = Phase::ByteTwo;
                Some(MidiEvent::NoteOn(self.channel, self.data_byte, byte))
            }
            MESSAGE_POLY_AFTER_TOUCH => {
                self.phase = Phase::ByteTwo;
                Some(MidiEvent::PolyphonicAfterTouch(
                    self.channel,
                    self.data_byte,
                    byte,
                ))
            }
            MESSAGE_CONTROLLER_CHANGE => {
                self.phase = Phase::ByteTwo;
                // Split out mode messages
                match self.data_byte {
                    CHANNEL_MODE_ALL_SOUND_OFF => Some(MidiEvent::AllSoundOff),
                    CHANNEL_MODE_RESET_ALL => Some(MidiEvent::ResetAllControllers),
                    CHANNEL_MODE_LOCAL_CONTROL => {
                        Some(MidiEvent::LocalControl(self.channel, byte != 0))
                    }
                    CHANNEL_MODE_ALL_NOTES_OFF => Some(MidiEvent::AllNotesOff(self.channel)),
                    CHANNEL_MODE_OMNI_ON => Some(MidiEvent::OmniMode(self.channel, true)),
                    CHANNEL_MODE_OMNI_OFF => Some(MidiEvent::OmniMode(self.channel, false)),
                    CHANNEL_MODE_MONO_ON => Some(MidiEvent::MonoMode(self.channel, byte)),
                    CHANNEL_MODE_POLYPHONIC_ON => Some(MidiEvent::PolyphonicMode(self.channel)),
                    _ => Some(MidiEvent::ControllerChange(
                        self.channel,
                        self.data_byte,
                        byte,
                    )),
                }
            }
            MESSAGE_PITCH_BEND => {
                self.phase = Phase::ByteTwo;
                Some(MidiEvent::PitchBend(
                    self.channel,
                    to_i14(byte, self.data_byte),
                ))
            }
            MESSAGE_SYS_EX_START => {
                // Handles a Sys Ex Word ID (a 4 byte message)
                self.phase = Phase::SysExID;
                self.data_byte = byte;
                None
            }
            MESSAGE_SONG_POSITION_PTR => {
                self.phase = Phase::Start;
                Some(MidiEvent::SongPositionPointer(to_u14(byte, self.data_byte)))
            }
            _ => {
                self.phase = Phase::Start;
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{MidiEvent, MidiReader};
    use parameterized::parameterized;

    #[parameterized(
        bytes = {
            // Real time
            &[0xF8],
            &[0xFA],
            &[0xFB],
            &[0xFC],
            &[0xFE],
            &[0xFF],

            // Channel messages
            &[0x82, 0x40, 127],
        },
        expected = {
            // Real time
            vec![MidiEvent::Clock],
            vec![MidiEvent::Start],
            vec![MidiEvent::Continue],
            vec![MidiEvent::Stop],
            vec![MidiEvent::ActiveSensing],
            vec![MidiEvent::SystemReset],

            // Channel messages
            vec![MidiEvent::NoteOff(2, 0x40, 127)],
        }
    )]
    fn handle_byte__single_event(bytes: &[u8], expected: Vec<MidiEvent>) {
        let mut target = MidiReader::new();

        let mut actual = Vec::new();
        for byte in bytes.iter() {
            let result = target.handle_byte(*byte);
            if let Some(event) = result {
                actual.push(event);
            }
        }

        assert_eq!(actual, expected);
    }
}
