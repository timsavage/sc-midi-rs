use crate::{u3, u4, MidiEvent};
use std::fmt::{Display, Formatter};

const MSN: u8 = 0b1111_0000;
const LSN: u8 = 0b0000_1111;

///
/// Time-code framerate
///
#[derive(Debug, Copy, Clone)]
pub enum Rate {
    Film,
    PAL,
    NTSCDropFrame,
    NTSC,
}

impl Rate {
    ///
    /// From a raw MTC Quarter frame
    ///
    /// Method will handle bit shifting
    ///
    fn from_event(rate: u4) -> Option<Self> {
        match rate & 0b0110 {
            0b0000 => Some(Self::Film),
            0b0010 => Some(Self::PAL),
            0b0100 => Some(Self::NTSCDropFrame),
            0b0110 => Some(Self::NTSC),
            _ => None,
        }
    }

    ///
    /// Value for a MTC Quarter frame
    ///
    fn to_event(&self) -> u4 {
        match self {
            Self::Film => 0b0000,
            Self::PAL => 0b0010,
            Self::NTSCDropFrame => 0b0100,
            Self::NTSC => 0b0110,
        }
    }

    ///
    /// **Frame rate in Frames/s**
    ///
    /// Drop frame is rounded to 30FPS
    ///
    fn frame_rate(&self) -> u8 {
        match self {
            Self::Film => 24,
            Self::PAL => 25,
            Self::NTSCDropFrame => 30,
            Self::NTSC => 30,
        }
    }
}

///
/// MIDI Time code struct.
///
/// Default rate is PAL
///
#[derive(Debug, Copy, Clone)]
pub struct TimeCode {
    frame: u8,
    second: u8,
    minute: u8,
    hour: u8,
    rate: Rate,
}

impl TimeCode {
    pub fn new(hour: u8, minute: u8, second: u8, frame: u8) -> Self {
        Self {
            hour,
            minute,
            second,
            frame,
            rate: Rate::PAL,
        }
    }

    pub fn set_rate(&mut self, rate: Rate) {
        self.rate = rate;
    }

    ///
    /// Update a time-code field from a MTC Quarter frame
    ///
    pub fn update_from_event(&mut self, piece: u3, data: u4) {
        match piece {
            0 => self.frame = (self.frame & MSN) | (data & LSN),
            1 => self.frame = (self.frame & LSN) | ((data & LSN) << 4),
            2 => self.second = (self.second & MSN) | (data & LSN),
            3 => self.second = (self.second & LSN) | ((data & LSN) << 4),
            4 => self.minute = (self.minute & MSN) | (data & LSN),
            5 => self.minute = (self.minute & LSN) | ((data & LSN) << 4),
            6 => self.hour = (self.hour & MSN) | (data & LSN),
            7 => {
                self.hour = (self.hour & LSN) | ((data & 0b0001) << 4);
                self.rate = Rate::from_event(data).unwrap()
            }
            _ => (),
        }
    }
}

impl Default for TimeCode {
    fn default() -> Self {
        Self {
            frame: 0,
            second: 0,
            minute: 0,
            hour: 0,
            rate: Rate::PAL,
        }
    }
}

impl Display for TimeCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}{}{}",
            self.hour,
            self.minute,
            self.second,
            match self.rate {
                Rate::NTSCDropFrame => ";",
                _ => ":",
            },
            self.frame
        )
    }
}
