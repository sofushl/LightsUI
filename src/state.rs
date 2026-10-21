use serde::Serialize;
use topcoat::runtime::record;

#[record]
#[derive(Clone)]
pub struct LedState {
    pub mode: String,
    pub brightness: String,
    pub speed: String,
    pub red: String,
    pub green: String,
    pub blue: String,
}

impl Default for LedState {
    fn default() -> Self {
        Self {
            mode: "CHASE".into(),
            brightness: "255".into(),
            speed: "100".into(),
            red: "255".into(),
            green: "0".into(),
            blue: "0".into(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct LedCommand {
    mode: String,
    brightness: u8,
    speed: u8,
    red: u8,
    green: u8,
    blue: u8,
}

impl TryFrom<LedState> for LedCommand {
    type Error = std::num::ParseIntError;

    fn try_from(s: LedState) -> Result<Self, Self::Error> {
        Ok(Self {
            mode: s.mode,
            brightness: s.brightness.parse()?,
            speed: s.speed.parse()?,
            red: s.red.parse()?,
            green: s.green.parse()?,
            blue: s.blue.parse()?,
        })
    }
}
