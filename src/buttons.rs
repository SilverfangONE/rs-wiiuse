use std::error::Error;
use std::fmt;
use wiiuse_sys;

#[derive(Debug, Clone, Copy)]
pub enum WiimoteButton {
    One,
    Two,
    B,
    A,
    Minus,
    Home,
    Left,
    Right,
    Down,
    Up,
    Plus,
}

#[derive(Debug)]
pub struct UnknownWiimoteButtonError(u32);

impl fmt::Display for UnknownWiimoteButtonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown wiimote button code {}", self.0)
    }
}

impl Error for UnknownWiimoteButtonError {}

impl TryFrom<u32> for WiimoteButton {
    type Error = UnknownWiimoteButtonError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let res = match value {
            wiiuse_sys::WIIMOTE_BUTTON_A => WiimoteButton::A,
            wiiuse_sys::WIIMOTE_BUTTON_B => WiimoteButton::B,
            wiiuse_sys::WIIMOTE_BUTTON_ONE => WiimoteButton::One,
            wiiuse_sys::WIIMOTE_BUTTON_HOME => WiimoteButton::Home,
            wiiuse_sys::WIIMOTE_BUTTON_MINUS => WiimoteButton::Minus,
            wiiuse_sys::WIIMOTE_BUTTON_LEFT => WiimoteButton::Left,
            wiiuse_sys::WIIMOTE_BUTTON_RIGHT => WiimoteButton::Right,
            wiiuse_sys::WIIMOTE_BUTTON_DOWN => WiimoteButton::Down,
            wiiuse_sys::WIIMOTE_BUTTON_UP => WiimoteButton::Up,
            wiiuse_sys::WIIMOTE_BUTTON_PLUS => WiimoteButton::Plus,
            _ => {
                return Err(UnknownWiimoteButtonError(value));
            }
        };
        Ok(res)
    }
}
