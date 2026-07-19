pub mod buttons;
pub mod error;
pub mod events;
pub mod wiiuse;

pub use buttons::*;
pub use events::*;

use std::ffi::CStr;
use wiiuse_sys::*;

pub fn get_version() -> String {
    unsafe {
        let c_str_ptr = wiiuse_version();
        if c_str_ptr.is_null() {
            return "unknown".to_string();
        }
        let c_str = CStr::from_ptr(c_str_ptr);
        let str = c_str.to_str().map(|s| s.to_owned());
        match str {
            Ok(s) => s,
            Err(_) => "utf8-error".to_string(),
        }
    }
}
