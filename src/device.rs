use libc::{c_char, c_int, c_void};
use raw;
use std::ffi::CStr;
use std::str::Utf8Error;

pub struct Device {
    raw: *mut raw::Device,
}

impl Device {
    pub unsafe fn from_raw(raw: *mut raw::Device) -> Device {
        Device { raw }
    }

    pub fn baseline(&self) -> f32 {
        unsafe { raw::lm_device_baseline(self.raw) }
    }

    pub fn horizontal_view_angle(&self) -> f32 {
        unsafe { raw::lm_device_horizontal_view_angle(self.raw) }
    }

    pub fn is_embedded(&self) -> bool {
        unsafe { raw::lm_device_is_embedded(self.raw) }
    }

    // Blocked by 3.0 release
    // pub fn is_lighting_bad(&self) -> bool {
    //     unsafe {
    //         raw::lm_device_is_lighting_bad(self.raw)
    //     }
    // }

    // Blocked by 3.0 release
    // pub fn is_smudged(&self) -> bool {
    //     unsafe {
    //         raw::lm_device_is_smudged(self.raw)
    //     }
    // }

    pub fn is_streaming(&self) -> bool {
        unsafe { raw::lm_device_is_streaming(self.raw) }
    }

    pub fn is_valid(&self) -> bool {
        unsafe { raw::lm_device_is_valid(self.raw) }
    }

    pub fn range(&self) -> f32 {
        unsafe { raw::lm_device_range(self.raw) }
    }

    pub fn vertical_view_angle(&self) -> f32 {
        unsafe { raw::lm_device_vertical_view_angle(self.raw) }
    }

    pub fn serial_number(&self) -> Result<String, Utf8Error> {
        unsafe extern "C" fn init_cb(ret: *mut c_void, data: *const c_char) {
            let ret = ret as *mut Result<String, Utf8Error>;
            *ret = CStr::from_ptr(data).to_str().map(|s| s.to_string());
        }

        unsafe {
            let mut result = Ok(String::new());

            raw::lm_device_serial_number(self.raw, &mut result as *mut _ as *mut _, init_cb);

            result
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            raw::lm_device_delete(self.raw);
        }
    }
}

pub struct DeviceList {
    raw: *mut raw::DeviceList,
}

impl DeviceList {
    pub unsafe fn from_raw(raw: *mut raw::DeviceList) -> DeviceList {
        DeviceList { raw }
    }

    pub fn len(&self) -> usize {
        unsafe { raw::lm_device_list_count(self.raw) as usize }
    }

    pub fn is_empty(&self) -> bool {
        unsafe { raw::lm_device_list_is_empty(self.raw) }
    }

    pub fn get(&self, index: usize) -> Option<Device> {
        unsafe {
            if index < self.len() {
                Some(Device::from_raw(raw::lm_device_list_at(
                    self.raw,
                    index as c_int,
                )))
            } else {
                None
            }
        }
    }

    pub fn iter(&self) -> Iter {
        Iter {
            list: self,
            index: 0,
        }
    }
}

impl Drop for DeviceList {
    fn drop(&mut self) {
        unsafe {
            raw::lm_device_list_delete(self.raw);
        }
    }
}

pub struct Iter<'a> {
    list: &'a DeviceList,
    index: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = Device;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(device) = self.list.get(self.index) {
            self.index += 1;
            Some(device)
        } else {
            None
        }
    }
}
