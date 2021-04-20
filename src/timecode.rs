use crate::{u3, u4, MidiEvent};
use std::fmt::{Display, Formatter};

const MSN: u8 = 0b1111_0000;
const LSN: u8 = 0b0000_1111;

///
/// Time-code framerate
///
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Rate {
    Film,
    PAL,
    NTSCDropFrame,
    NTSC,
}

impl Rate {
    ///
    /// **From a raw MTC Quarter frame**
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
}

///
/// MIDI Time code struct.
///
/// Default rate is PAL
///
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
            hour: hour & 0b0001_1111,
            minute: minute & 0b0011_1111,
            second: second & 0b0011_1111,
            frame: frame & 0b0001_1111,
            rate: Rate::Film,
        }
    }

    ///
    /// Set the frame rate
    ///
    pub fn at_rate(mut self, rate: Rate) -> Self {
        self.rate = rate;
        self
    }

    ///
    /// Update a time-code field from a MTC Quarter frame
    ///
    pub fn update_from_event(&mut self, piece: u3, data: u4) {
        match piece {
            0 => self.frame = (self.frame & MSN) | (data & 0b1111),
            1 => self.frame = (self.frame & LSN) | ((data & 0b0001) << 4),
            2 => self.second = (self.second & MSN) | (data & 0b1111),
            3 => self.second = (self.second & LSN) | ((data & 0b0011) << 4),
            4 => self.minute = (self.minute & MSN) | (data & 0b1111),
            5 => self.minute = (self.minute & LSN) | ((data & 0b0011) << 4),
            6 => self.hour = (self.hour & MSN) | (data & 0b1111),
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
            rate: Rate::Film,
        }
    }
}

impl Display for TimeCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}:{:02}:{:02}{}{:02}",
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

#[cfg(test)]
mod tests {
    use crate::timecode::{Rate, TimeCode};
    use std::sync::mpsc::RecvTimeoutError::Timeout;

    #[test]
    fn test_display__where_default_is_created() {
        let target = TimeCode::default();

        let actual = format!("{}", target);

        assert_eq!(actual, "00:00:00:00")
    }

    #[test]
    fn test_display__where_new_with_film() {
        let target = TimeCode::new(24, 59, 59, 23);

        let actual = format!("{}", target);

        assert_eq!(actual, "24:59:59:23")
    }

    #[test]
    fn test_display__where_is_drop_frame() {
        let target = TimeCode::new(9, 8, 7, 6).at_rate(Rate::NTSCDropFrame);

        let actual = format!("{}", target);

        assert_eq!(actual, "09:08:07;06")
    }

    #[test]
    fn test_update_from_event() {
        let mut target = TimeCode::default();

        target.update_from_event(0, 15);
        target.update_from_event(1, 1);
        target.update_from_event(2, 15);
        target.update_from_event(3, 3);
        target.update_from_event(4, 15);
        target.update_from_event(5, 3);
        target.update_from_event(6, 15);
        target.update_from_event(7, 7);

        let expected = TimeCode::new(31, 63, 63, 31).at_rate(Rate::NTSC);
        assert_eq!(target, expected)
    }
}
