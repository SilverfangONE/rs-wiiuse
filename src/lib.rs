use bitflags::bitflags;
use std::{ffi::CStr, marker::PhantomData};

pub type WiimotePtrArr = *mut *mut wiiuse_sys::wiimote_t;
pub type WiimotePtr = *mut wiiuse_sys::wiimote_t;

pub const DEFAULT_EXPANSION_TIMEOUT: u8 = 100;
pub const DEFAULT_POLL_TIMEOUT: u8 = 100;

pub struct WiimoteId(pub usize);

impl std::ops::Deref for WiimoteId {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Wiiuse {
    wm_arr_ptr: *mut *mut wiiuse_sys::wiimote_t,
    max_wiimotes: i32,
}

impl Wiiuse {
    pub fn init(max_wiimotes: u32) -> Self {
        let max_wiimotes = max_wiimotes as i32;
        let wm_arr_ptr = unsafe { wiiuse_sys::wiiuse_init(max_wiimotes) };
        Self {
            wm_arr_ptr,
            max_wiimotes,
        }
    }

    pub fn version() -> String {
        let c_str_ptr = unsafe { wiiuse_sys::wiiuse_version() };
        if c_str_ptr.is_null() {
            return "unknown".to_string();
        }
        let c_str = unsafe { CStr::from_ptr(c_str_ptr) };
        let str = c_str.to_str().map(|s| s.to_owned());
        match str {
            Ok(s) => s,
            Err(_) => "utf8-error".to_string(),
        }
    }

    pub fn get_wiimote_by_id(&self, id: WiimoteId) -> Option<Wiimote<'_>> {
        let slice =
            unsafe { std::slice::from_raw_parts(self.wm_arr_ptr, self.max_wiimotes as usize) };

        let index: usize = *id;

        if index < slice.len() {
            let ptr = slice[index];
            if ptr.is_null() {
                return None;
            }
            Some(Wiimote {
                ptr,
                _marker: PhantomData,
            })
        } else {
            None
        }
    }

    pub fn find(&self, timeout_sec: u32) -> u32 {
        let found = unsafe {
            wiiuse_sys::wiiuse_find(self.wm_arr_ptr, self.max_wiimotes, timeout_sec as i32)
        };
        return if found < 0 { 0 } else { found as u32 };
    }

    pub fn connect(&self) -> u32 {
        let connected = unsafe { wiiuse_sys::wiiuse_connect(self.wm_arr_ptr, self.max_wiimotes) };
        return if connected < 0 { 0 } else { connected as u32 };
    }

    pub fn disconnect_by_id(&self, id: WiimoteId) -> Result<(), String> {
        if let Some(wiimote) = self.get_wiimote_by_id(id) {
            unsafe {
                wiiuse_sys::wiiuse_disconnect(wiimote.ptr);
            }
            Ok(())
        } else {
            Err("wiimot with id {} not found".to_string())
        }
    }

    pub fn disconnect_raw(&self, wm_ptr: WiimotePtr) {
        unsafe {
            wiiuse_sys::wiiuse_disconnect(wm_ptr);
        }
    }

    pub fn connect_all(&self, timeout_sec: u32) -> Result<u32, String> {
        let found = self.find(timeout_sec);
        if found <= 0 {
            return Err("no controller found".into());
        }
        let count = self.connect();
        if count > 0 {
            Ok(count as u32)
        } else {
            Err(format!("connection error (found {})", found))
        }
    }

    ///  return the number of wiimotes that had an event occur
    pub fn poll(&self) -> u32 {
        let wiimotes = unsafe { wiiuse_sys::wiiuse_poll(self.wm_arr_ptr, self.max_wiimotes) };
        wiimotes as u32
    }

    /// ## Arguments
    ///
    /// * `normal_timeout_ms` - used for normal polling.
    /// * `exp_timeout_ms` - used when an expansion is detected until the expansion succesfully handshakes.
    ///
    pub fn set_timeout(&self, normal_timeout_ms: u8, exp_timeout_ms: u8) {
        unsafe {
            wiiuse_sys::wiiuse_set_timeout(
                self.wm_arr_ptr,
                self.max_wiimotes,
                normal_timeout_ms,
                exp_timeout_ms,
            );
        }
    }
}

impl Drop for Wiiuse {
    fn drop(&mut self) {
        unsafe {
            wiiuse_sys::wiiuse_cleanup(self.wm_arr_ptr, self.max_wiimotes);
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct WiimoteLeds(pub bool, pub bool, pub bool, pub bool);

impl WiimoteLeds {
    pub fn new() -> Self {
        WiimoteLeds::default()
    }

    pub fn on_1(mut self) -> Self {
        self.0 = true;
        self
    }
    pub fn on_2(mut self) -> Self {
        self.1 = true;
        self
    }
    pub fn on_3(mut self) -> Self {
        self.2 = true;
        self
    }
    pub fn on_4(mut self) -> Self {
        self.3 = true;
        self
    }

    pub fn bitmask(&self) -> i32 {
        let l1 = if self.0 { 0x10 } else { 0 };
        let l2 = if self.1 { 0x20 } else { 0 };
        let l3 = if self.2 { 0x30 } else { 0 };
        let l4 = if self.3 { 0x40 } else { 0 };
        l1 | l2 | l3 | l4
    }
}

pub struct Wiimote<'a> {
    ptr: *mut wiiuse_sys::wiimote_t,
    _marker: PhantomData<&'a Wiiuse>,
}

impl<'a> Wiimote<'a> {
    pub fn rumble(&self, status: bool) {
        unsafe {
            wiiuse_sys::wiiuse_rumble(self.ptr, if status { 1 } else { 0 });
        }
    }

    pub fn toggle_rumble(&self) {
        unsafe {
            wiiuse_sys::wiiuse_toggle_rumble(self.ptr);
        }
    }

    pub fn set_leds(&self, config: WiimoteLeds) {
        unsafe {
            wiiuse_sys::wiiuse_set_leds(self.ptr, config.bitmask());
        }
    }

    pub fn get_battery_level(&self) -> f32 {
        unsafe { (*self.ptr).battery_level }
    }
}

bitflags! {
    pub struct WiimoteButton: u16 {
        const LEFT      = 0x0001;
        const RIGHT     = 0x0002;
        const DOWN      = 0x0004;
        const UP        = 0x0008;
        const PLUS      = 0x0010;
        const TWO       = 0x0100;
        const ONE       = 0x0200;
        const B         = 0x0400;
        const A         = 0x0800;
        const MINUS     = 0x1000;
        const HOME      = 0x8000;
    }
}

impl<'a> Wiimote<'a> {
    pub fn is_button_pressed(&self, buttons: WiimoteButton) -> bool {
        unsafe {
            let current_btns = (*self.ptr).btns;
            (current_btns & buttons.bits()) != 0
        }
    }
}

// pub trait Wiiuse {
//     pub fn init(wiimotes: i32) -> Self;
//     pub fn cleanup(self);
//     pub fn version(&self) -> String;
//     fn find(&self) -> i32;
//     fn connect(&self) -> i32;
//     fn disconnect(&self) -> i32;
//     fn poll(&self) -> i32;
//     fn rumble(&self, id: WiimoteId);
//     fn toggle_rumble(&self, id: WiimoteId);
//     fn set_leds(&self, id: WiimoteId);
//     fn motion_sensing(&self, id: WiimoteId);
//     fn read_data(&self, id: WiimoteId);
//     fn write_data(&self, id: WiimoteId);
//     fn status(&self, id: WiimoteId);
//     fn get_by_id(&self, id: WiimoteId);
//     fn set_flags(&self, id: WiimoteId);
//     fn set_smooth_alpha(&self, id: WiimoteId);
//     fn set_bluetooth_stack();
//     fn set_orient_threshold();
//     fn set_accel_threshold();
//     fn set_nunchuk_orient_threshold();
//     fn set_nunchuk_accel_threshold();
//     fn resync();
//     fn set_timeout();
//     fn set_ir();
//     fn set_ir_vres();
//     fn set_ir_position();
//     fn set_ir_sensitivity();
//     fn set_aspect_ratio();
// }
