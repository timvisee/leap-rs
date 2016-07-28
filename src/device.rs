use std::os::raw::c_int;
use raw;

pub struct Device {
    raw: *mut raw::Device
}

impl Device {
    pub unsafe fn from_raw(raw: *mut raw::Device) -> Device {
        Device {
            raw: raw
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
    raw: *mut raw::DeviceList
}

impl DeviceList {
    pub unsafe fn from_raw(raw: *mut raw::DeviceList) -> DeviceList {
        DeviceList {
            raw: raw
        }
    }

    pub fn len(&self) -> usize {
        unsafe {
            raw::lm_device_list_count(self.raw) as usize
        }
    }

    pub fn is_empty(&self) -> bool {
        unsafe {
            raw::lm_device_list_is_empty(self.raw)
        }
    }

    pub fn get(&self, index: usize) -> Option<Device> {
        unsafe {
            if index < self.len() {
                Some(Device::from_raw(raw::lm_device_list_at(self.raw, index as c_int)))
            }
            else {
                None
            }
        }
    }

    pub fn iter(&self) -> Iter {
        Iter {
            list: self,
            index: 0
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
    index: usize
}

impl<'a> Iterator for Iter<'a> {
    type Item = Device;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(device) = self.list.get(self.index) {
            self.index += 1;
            Some(device)
        }
        else {
            None
        }
    }
}
