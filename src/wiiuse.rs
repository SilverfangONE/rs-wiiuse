pub type WiimotePtrArr = *mut *mut wiiuse_sys::wiimote_t;
pub type WiimotePtr = *mut wiiuse_sys::wiimote_t;
pub type WiimoteId = usize;

pub struct WiiuseContext {
    wm_arr_ptr: *mut *mut wiiuse_sys::wiimote_t,
    max_wiimotes: i32,
}

pub struct Wiimote<'a> {
    ptr: *mut wiiuse_sys::wiimote_t,
    context: &'a WiiuseContext,
}

impl WiiuseContext {
    pub fn init(max_wiimotes: u32) -> Self {
        let max_wiimotes = max_wiimotes as i32;
        let wm_arr_ptr = unsafe { wiiuse_sys::wiiuse_init(max_wiimotes) };
        Self {
            wm_arr_ptr,
            max_wiimotes,
        }
    }

    pub fn get_wiimote(&self, index: usize) -> Option<Wiimote<'_>> {
        let slice =
            unsafe { std::slice::from_raw_parts(self.wm_arr_ptr, self.max_wiimotes as usize) };

        if index < slice.len() {
            let ptr = slice[index];
            if ptr.is_null() {
                return None;
            }
            Some(Wiimote { ptr, context: self })
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

    pub fn poll()
}

impl<'a> Wiimote<'a> {
    pub fn rumble(&self, status: bool) {
        unsafe {
            wiiuse_sys::wiiuse_rumble(self.ptr, if status { 1 } else { 0 });
        }
    }

    pub fn get_battery_level(&self) -> f32 {
        unsafe { (*self.ptr).battery_level }
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
