#![allow(unused)]

mod bindings;
pub use bindings::*;

use std::error::Error;
use std::fmt;

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

pub fn sdk_init(chips: &mut [SwitchChip], chip_num: &mut i32) -> SdkResult {
    unsafe { chip_sdk_init(chips.as_mut_ptr(), chip_num as *mut i32).to_result() }
}

pub fn sdk_register_link_status_callback(cb: LinkStatusCallback) -> SdkResult {
    unsafe { chip_sdk_register_link_status_callback(cb).to_result() }
}

pub fn sdk_set_mac(phy_port_id: &PhyPortId, mac: &Mac) -> SdkResult {
    unsafe { chip_sdk_set_mac(phy_port_id.0, phy_port_id.1, mac).to_result() }
}
