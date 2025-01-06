#![allow(unused)]

#[cfg(feature = "use_bindgen")]
mod bindings_gen;
#[cfg(feature = "use_bindgen")]
use bindings_gen::*;

#[cfg(not(feature = "use_bindgen"))]
mod bindings;
#[cfg(not(feature = "use_bindgen"))]
use bindings::*;

use std::fmt;

pub type SwitchChip = SwitchChipTag;
pub type PhyPort = PhyPortTag;
pub type Mac = MacTag;

#[derive(Debug)]
pub struct SdkError {
    code: i32,
    message: String,
}

impl SdkError {
    pub fn new(code: i32, message: String) -> Self {
        SdkError { code, message }
    }
}

impl fmt::Display for SdkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SdkError: code={}, message={}", self.code, self.message)
    }
}

impl From<ChipSdkError> for SdkError {
    fn from(error: ChipSdkError) -> Self {
        match error {
            ChipSdkError::CHIP_SDK_ERROR => SdkError::new(1, "Error".to_string()),
            ChipSdkError::CHIP_SDK_INVALID_PARAM => SdkError::new(2, "Invalid parameter".to_string()),
            ChipSdkError::CHIP_SDK_NO_MEMORY => SdkError::new(3, "No memory".to_string()),
            ChipSdkError::CHIP_SDK_NO_RESOURCE => SdkError::new(4, "No resource".to_string()),
            ChipSdkError::CHIP_SDK_NOT_FOUND => SdkError::new(5, "Not found".to_string()),
            ChipSdkError::CHIP_SDK_NOT_SUPPORTED => SdkError::new(6, "Not supported".to_string()),
            ChipSdkError::CHIP_SDK_BUSY => SdkError::new(7, "Busy".to_string()),
            ChipSdkError::CHIP_SDK_TIMEOUT => SdkError::new(8, "Timeout".to_string()),
            ChipSdkError::CHIP_SDK_NO_CHANGE => SdkError::new(9, "No change".to_string()),
            _ => SdkError::new(error as i32, "Unknown error".to_string()),
        }
    }
}

pub struct PhyPortId(i32, i32);

pub fn init_sdk() -> Result<Vec<SwitchChip>, SdkError> {
    let mut chips = [SwitchChip::default(); CHIP_SDK_CHIP_MAX];
    let mut chip_num = 0;
    let ret = unsafe { chip_sdk_init(chips.as_mut_ptr(), &mut chip_num) };
    if ret == ChipSdkError::CHIP_SDK_SUCCESS {
        Ok(chips[..chip_num as usize].to_vec())
    } else {
        Err(ret.into())
    }
}

pub fn register_link_status_callback(cb: LinkStatusCallback) -> Result<(), SdkError> {
    let ret = unsafe { chip_sdk_register_link_status_callback(cb) };
    if ret == ChipSdkError::CHIP_SDK_SUCCESS {
        Ok(())
    } else {
        Err(ret.into())
    }
}

pub fn set_mac(phy_port_id: &PhyPortId, mac: &Mac) -> Result<(), SdkError> {
    let ret = unsafe { chip_sdk_set_mac(phy_port_id.0, phy_port_id.1, mac) };
    if ret == ChipSdkError::CHIP_SDK_SUCCESS {
        Ok(())
    } else {
        Err(ret.into())
    }
}