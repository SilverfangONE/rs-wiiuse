use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct UnknownEventError(u32);

impl fmt::Display for UnknownEventError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unbekannter C-Event-Code empfangen: {}", self.0)
    }
}

impl Error for UnknownEventError {}

#[allow(non_camel_case_types)]
pub enum WiiuseEvent {
    None,
    Event,
    Status,
    Disconnect,
    ReadData,
}

impl From<u32> for WiiuseEvent {
    fn from(value: u32) -> Self {
        match value {
            wiiuse_sys::WIIUSE_EVENT_TYPE_WIIUSE_EVENT => WiiuseEvent::Event,
            wiiuse_sys::WIIUSE_EVENT_TYPE_WIIUSE_STATUS => WiiuseEvent::Status,
            wiiuse_sys::WIIUSE_EVENT_TYPE_WIIUSE_DISCONNECT => WiiuseEvent::Disconnect,
            wiiuse_sys::WIIUSE_EVENT_TYPE_WIIUSE_READ_DATA => WiiuseEvent::ReadData,
            wiiuse_sys::WIIUSE_EVENT_TYPE_WIIUSE_NONE | _ => WiiuseEvent::None,
        }
    }
}
