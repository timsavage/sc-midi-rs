use std::fmt::Formatter;

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

impl std::fmt::Display for ChannelMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ChannelMode::AllSoundOff => "All sound off",
            ChannelMode::ResetAllControllers => "Reset all controllers",
            ChannelMode::LocalControl(_) => "Local Control",
            ChannelMode::AllNotesOff => "All notes off",
            ChannelMode::OmniModeOff => "Omni mode off",
            ChannelMode::OmniModeOn => "Omni mode on",
            ChannelMode::MonoModeOn(channels) => "Mono mode on",
            ChannelMode::PolyModeOn => "Poly mode on",
        })
    }
}

///
/// Channel Message
///
pub enum ChannelMessage {
    NoteOff(Channel, u7, u7),  // Channel, Key, Velocity
    NoteOn(Channel, u7, u7),  // Channel, Key, Velocity
    AfterTouchPoly(Channel, u7, u7),  // Channel, Key, Pressure
    ControllerChange(Channel, u7, u7),  // Channel, Control, Value
    ProgramChange(Channel, u7),  // Channel, Program Num
    AfterTouchChannel(Channel, u7),   // Channel, Key, Pressure
    PitchBend(Channel, i14),  // Channel, Amount
}

impl std::fmt::Display for ChannelMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ChannelMessage::NoteOff(channel, key, velocity) => "Note Off",
            ChannelMessage::NoteOn(channel, key, velocity) => "Note On",
            ChannelMessage::AfterTouchPoly(channel, key, pressure) => "Polyphonic After Touch",
            ChannelMessage::ControllerChange(channel, control, value) => "Controller Change",
            ChannelMessage::ProgramChange(channel, program_num) => "Program Change",
            ChannelMessage::AfterTouchChannel(channel, control) => "Channel After Touch",
            ChannelMessage::PitchBend(channel, amount) => "Pitch Bend",
        })
    }
}

///
/// System messages
///
pub enum SystemMessage {
    SysExStart,
    TimeCodeQuarterFrame(u7, u7),
    SongPositionPointer(u14),
    SongSelect(u7),
    TuneRequest,
}

impl std::fmt::Display for SystemMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            SystemMessage::SysExStart => "System Ex Start",
            SystemMessage::TimeCodeQuarterFrame(_, _) => "Midi Time-code Quarter Frame",
            SystemMessage::SongPositionPointer(_) => "Song Position Pointer",
            SystemMessage::SongSelect(_) => "Song Select",
            SystemMessage::TuneRequest => "Tune Request",
        })
    }
}

///
/// System Realtime Messages
///
pub enum SystemRTMessage {
    Clock,
    Start,
    Continue,
    Stop,
    ActiveSensing,
    SystemReset
}

impl std::fmt::Display for SystemRTMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            SystemRTMessage::Clock => "System Real Time - Timing Clock",
            SystemRTMessage::Start => "System Real Time - Start",
            SystemRTMessage::Continue => "System Real Time - Continue",
            SystemRTMessage::Stop => "System Real Time - Stop",
            SystemRTMessage::ActiveSensing => "System Real Time - Active Sensing",
            SystemRTMessage::SystemReset => "System Real Time - System Reset",
        })
    }
}

pub enum Message {
    ChannelMessage
}