use windows::Win32::Devices::DeviceAndDriverSetup::{
    SetupDiGetClassDevsW,
    SetupDiEnumDeviceInfo,
    SetupDiGetDeviceInstanceIdW,
    DIGCF_PRESENT
};
use windows::core::PCWSTR;
use std::ptr;

pub struct DeviceInfo {
    pub hwid: String,
    pub name: String,
    pub driver_status: DriverStatus,
}

#[derive(Debug)]
pub enum DriverStatus {
    UpToDate,
    NeedsUpdate,
    Missing,
}

pub fn detect_hardware() -> Vec<DeviceInfo> {
    let mut devices = Vec::new();
    unsafe {
        let hdev = SetupDiGetClassDevsW(
            None,
            PCWSTR::null(),
            None,
            DIGCF_PRESENT
        );
        
        let mut dev_info = std::mem::zeroed();
        for idx in 0.. {
            if SetupDiEnumDeviceInfo(hdev, idx, &mut dev_info).is_err() {
                break;
            }
            
            let mut buffer = [0u16; 256];
            let mut size = 0;
            SetupDiGetDeviceInstanceIdW(
                hdev,
                &dev_info,
                Some(&mut buffer),
                buffer.len() as u32,
                &mut size
            );
            
            let hwid = String::from_utf16_lossy(&buffer[..size as usize]);
            devices.push(DeviceInfo {
                hwid,
                name: String::new(),
                driver_status: DriverStatus::Missing,
            });
        }
    }
    devices
}