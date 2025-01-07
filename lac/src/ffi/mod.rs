#![allow(unused)]

#[cfg(feature = "use_bindgen")]
mod bindings_gen;
#[cfg(feature = "use_bindgen")]
pub use bindings_gen::*;

#[cfg(not(feature = "use_bindgen"))]
mod bindings;
#[cfg(not(feature = "use_bindgen"))]
pub use bindings::*;

use std::fmt;
use std::error::Error;

pub type SdkResult = Result<(), ChipSdkError>;
pub const SDK_OK: SdkResult = Ok(());

impl fmt::Display for ChipSdkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match self {
            ChipSdkError::CHIP_SDK_SUCCESS => "Success",
            ChipSdkError::CHIP_SDK_ERROR => "General error",
            ChipSdkError::CHIP_SDK_INVALID_PARAM => "Invalid parameter",
            ChipSdkError::CHIP_SDK_NO_MEMORY => "No memory",
            ChipSdkError::CHIP_SDK_NO_RESOURCE => "No resource",
            ChipSdkError::CHIP_SDK_NOT_FOUND => "Not found",
            ChipSdkError::CHIP_SDK_NOT_SUPPORTED => "Not supported",
            ChipSdkError::CHIP_SDK_BUSY => "Busy",
            ChipSdkError::CHIP_SDK_TIMEOUT => "Timeout",
            ChipSdkError::CHIP_SDK_NO_CHANGE => "No change",
        };
        write!(f, "{}", description)
    }
}

impl Error for ChipSdkError {}

impl ChipSdkError {
    pub fn to_result(self) -> SdkResult {
        if self == ChipSdkError::CHIP_SDK_SUCCESS {
            SDK_OK
        } else {
            Err(self)
        }
    }
}

pub type ChipId = i32;
type LocalPortId = i32;
pub struct PhyPortId(pub ChipId, pub LocalPortId);
pub type SwitchChip = SwitchChipTag;
pub type PhyPort = PhyPortTag;
pub type Mac = MacTag;

pub struct Device {
    chips: [SwitchChip; CHIP_SDK_CHIP_MAX],
    chip_num: i32,
}

impl Device {
    pub fn new() -> Self {
        let mut chips = [SwitchChip::default(); CHIP_SDK_CHIP_MAX];
        let mut chip_num = 0;
        let ret = unsafe { chip_sdk_init(chips.as_mut_ptr(), &mut chip_num)};
        if ret != ChipSdkError::CHIP_SDK_SUCCESS {
            panic!("Failed to initialize device: {}", ret);
        }
        Device { chips, chip_num, }
    }
    
    pub fn register_link_status_callback(&self, cb: LinkStatusCallback) -> SdkResult {
        unsafe { chip_sdk_register_link_status_callback(cb).to_result() }
    }
    
    pub fn set_mac(&self, phy_port_id: &PhyPortId, mac: &Mac) -> SdkResult {
        unsafe { chip_sdk_set_mac(phy_port_id.0, phy_port_id.1, mac).to_result() }
    }
}