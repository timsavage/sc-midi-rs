const STATUS_MASK: u8 = 0b1000_0000;
const CHANNEL_MASK: u8 = 0b0000_1111;
const SYSTEM_MASK: u8 = 0b1111_0000;
const SYSTEM_RT_MASK: u8 = 0b1111_1000;

const MESSAGE_NONE: u8                  = 0x00;
// Channel Voice Messages
const MESSAGE_NOTE_OFF: u8              = 0x80;
const MESSAGE_NOTE_ON: u8               = 0x90;
const MESSAGE_POLY_AFTER_TOUCH: u8      = 0xA0;  // Polyphonic AfterTouch
const MESSAGE_CONTROLLER_CHANGE: u8     = 0xB0;  // Controller Change / Channel Mode
const MESSAGE_PROGRAM_CHANGE: u8        = 0xC0;
const MESSAGE_CHANNEL_AFTER_TOUCH: u8   = 0xD0;
const MESSAGE_PITCH_BEND: u8            = 0xE0;
// System Common Messages
const MESSAGE_SYS_EX_START: u8          = 0xF0;  // System Exclusive Start
const MESSAGE_MTC_QUARTER_FRAME: u8     = 0xF1;  // Time Code Quarter Frame
const MESSAGE_SONG_POSITION_PTR: u8     = 0xF2;  // Song Position Pointer
const MESSAGE_SONG_SELECT: u8           = 0xF3;
const MESSAGE_TUNE_REQUEST: u8          = 0xF6;
const MESSAGE_SYS_EX_END: u8            = 0xF7;  // System Exclusive End
// System Realtime Messages
const MESSAGE_CLOCK: u8                 = 0xF8;
const MESSAGE_START: u8                 = 0xFA;
const MESSAGE_CONTINUE: u8              = 0xFB;
const MESSAGE_STOP: u8                  = 0xFC;
const MESSAGE_ACTIVE_SENSING: u8        = 0xFE;
const MESSAGE_SYSTEM_RESET: u8          = 0xFF;

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

///
/// Channel mode
///
pub enum ChannelMode {
    AllSoundOff,
    ResetAllControllers,
    LocalControl(bool),
    AllNotesOff,
    OmniModeOff,
    OmniModeOn,
    MonoModeOn(u7),
    PolyModeOn
}

///
/// Size of System Exclusive ID
///
pub enum SysExID {
    Byte(u7),
    Word(u14)
}

pub enum MidiEvent {
    // Channel events
    NoteOff(Channel, u7, u7),               // Channel, Key, Velocity
    NoteOn(Channel, u7, u7),                // Channel, Key, Velocity
    PolyphonicAfterTouch(Channel, u7, u7),  // Channel, Key, Pressure
    ControllerChange(Channel, u7, u7),      // Channel, Control, Value
    ProgramChange(Channel, u7),             // Channel, Program Num
    ChannelAfterTouch(Channel, u7),         // Channel, Pressure
    PitchBend(Channel, u14),                // Channel, Amount
    // System Common events
    TimeCodeQuarterFrame(u3, u4),           // Type, Value
    SongPositionPointer(u14),               // MIDI beats (1 beat = 6 MIDI clocks)
    SongSelect(u7),                         // Song Number
    TuneRequest,
    // System Realtime events
    Clock,
    Start,
    Continue,
    Stop,
    ActiveSensing,
    SystemReset,
    // System Exclusive events
    SystemExclusiveStart(SysExID),          // SysEx ID (Byte or Word)
    SystemExclusiveData(u7),                // Data byte
    SystemExclusiveEnd,
}

impl std::fmt::Display for MidiEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            MidiEvent::NoteOff(_, _, _) => "Note Off",
            MidiEvent::NoteOn(_, _, _) => "Note On",
            MidiEvent::PolyphonicAfterTouch(_, _, _) => "Polyphonic After-Touch",
            MidiEvent::ControllerChange(_, _, _) => "Controller Change",
            MidiEvent::ProgramChange(_, _) => "Program Change",
            MidiEvent::ChannelAfterTouch(_, _) => "Channel After-Touch",
            MidiEvent::PitchBend(_, _) => "Pitch-Bend",
            MidiEvent::TimeCodeQuarterFrame(_, _) => "MIDI Time Code Quarter Frame",
            MidiEvent::SongPositionPointer(_) => "Song Position Pointer",
            MidiEvent::SongSelect(_) => "Song Select",
            MidiEvent::TuneRequest => "Tune Request",
            MidiEvent::Clock => "Timing Clock",
            MidiEvent::Start => "Start",
            MidiEvent::Continue => "Continue",
            MidiEvent::Stop => "Stop",
            MidiEvent::ActiveSensing => "Active Sensing",
            MidiEvent::SystemReset => "System Reset",
            MidiEvent::SystemExclusiveStart(_) => "System Exclusive Start",
            MidiEvent::SystemExclusiveData(_) => "System Exclusive Data",
            MidiEvent::SystemExclusiveEnd => "System Exclusive End",
        })
    }
}

#[inline]
fn to_u14(high_byte: u7, low_byte: u7) -> u14 {
    ((high_byte as u14) << 7) | (low_byte as u14)
}

// #[inline]
// fn to_i14(high_byte: u7, low_byte: u7) -> i14 {
//     ((high_byte as u14) << 7) | (low_byte as u14)
// }

///
/// State-machine message phase
///
enum Phase {
    Start,
    HighByte,
    LowByte,
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
    message: u8,    // Current message
    channel: u8,    // Current channel (for channel messages)
    high_byte: u8,  // High data byte
}

impl MidiReader {
    pub fn new() -> Self {
        Self {
            phase: Phase::Start,
            message: MESSAGE_NONE,
            channel: 0,
            high_byte: 0,
        }
    }

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
            self.high_byte = 0;
        }

        match self.phase {
            Phase::Start => {
                // On reset ignore bytes until a status byte is received.
                if is_status_byte {
                    self.phase = Phase::HighByte;

                    if (byte & SYSTEM_MASK) == SYSTEM_MASK {
                        // Start byte is the message for system messages
                        self.message = byte;
                        return self.handle_system_message();
                    } else {
                        // Split start byte for channel messages
                        self.message = byte & SYSTEM_MASK;
                        self.channel = byte & CHANNEL_MASK;
                    }
                }
                None
            },
            Phase::HighByte => self.handle_high_byte(byte),
            Phase::LowByte => self.handle_low_byte(byte),
            Phase::SysExID => {
                self.phase = Phase::SysExData;
                Some(MidiEvent::SystemExclusiveStart(SysExID::Word(to_u14(self.high_byte, byte))))
            }
            Phase::SysExData => {
                Some(MidiEvent::SystemExclusiveData(byte))
            }
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
            _ => None
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
            },
            MESSAGE_SYS_EX_END => {
                self.phase = Phase::Start;
                Some(MidiEvent::SystemExclusiveEnd)
            }
            _ => None
        }
    }

    ///
    /// **Handle Two byte messages**
    ///
    fn handle_high_byte(&mut self, byte: u8) -> Option<MidiEvent> {
        match self.message {
            MESSAGE_PROGRAM_CHANGE => {
                self.phase = Phase::HighByte;
                Some(MidiEvent::ProgramChange(self.channel, byte))
            }
            MESSAGE_CHANNEL_AFTER_TOUCH => {
                self.phase = Phase::HighByte;
                Some(MidiEvent::ChannelAfterTouch(self.channel, byte))
            }
            MESSAGE_MTC_QUARTER_FRAME => {
                self.phase = Phase::HighByte;
                None
            },
            MESSAGE_SONG_SELECT => {
                self.phase = Phase::Start;
                Some(MidiEvent::SongSelect(byte))
            }
            MESSAGE_SYS_EX_START => {
                if byte == 0 {
                    self.phase = Phase::LowByte;
                    None
                } else {
                    // Sys Ex uses ID Byte
                    self.phase = Phase::SysExData;
                    Some(MidiEvent::SystemExclusiveStart(SysExID::Byte(byte)))
                }
            }
            _ => {
                self.phase = Phase::LowByte;
                self.high_byte = byte;
                None
            }
        }
    }

    ///
    /// **Handle Three bte messages**
    ///
    fn handle_low_byte(&mut self, byte: u8) -> Option<MidiEvent> {
        // Most messages allow for continuous events


        match self.message {
            MESSAGE_NOTE_OFF => {
                self.phase = Phase::HighByte;
                Some(MidiEvent::NoteOff(self.channel, self.high_byte, byte))
            },
            MESSAGE_NOTE_ON => {
                self.phase = Phase::HighByte;
                Some(MidiEvent::NoteOn(self.channel, self.high_byte, byte))
            },
            MESSAGE_POLY_AFTER_TOUCH => {
                self.phase = Phase::HighByte;
                Some(MidiEvent::PolyphonicAfterTouch(self.channel, self.high_byte, byte))
            },
            MESSAGE_CONTROLLER_CHANGE => {
                self.phase = Phase::HighByte;
                // TODO: Split out into mode vs controller events
                Some(MidiEvent::ControllerChange(self.channel, self.high_byte, byte))
            }
            MESSAGE_PITCH_BEND => {
                self.phase = Phase::HighByte;
                Some(MidiEvent::PitchBend(self.channel, to_u14(self.high_byte, byte)))
            },
            MESSAGE_SYS_EX_START => {
                // Handles a Sys Ex Word ID (a 4 byte message)
                self.phase = Phase::SysExID;
                self.high_byte = byte;
                None
            }
            MESSAGE_SONG_POSITION_PTR => {
                self.phase = Phase::Start;
                Some(MidiEvent::SongPositionPointer(to_u14(self.high_byte, byte)))
            }
            _ => {
                self.phase = Phase::Start;
                None
            }
        }
    }
}
