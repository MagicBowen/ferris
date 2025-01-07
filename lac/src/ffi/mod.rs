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

#[derive(Debug)]
pub struct SdkError(ChipSdkError);

impl SdkError {
    pub fn new(error: ChipSdkError) -> Self {
        SdkError(error)
    }
}

impl fmt::Display for SdkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SdkError: code={:?}", self.0)
    }
}

impl std::error::Error for SdkError {}

impl From<ChipSdkError> for SdkError {
    fn from(error: ChipSdkError) -> Self {
        match error {
            ChipSdkError::CHIP_SDK_SUCCESS => panic!("Invalid error code: {:?}", error),
            _ => SdkError::new(error),
        }
    }
}

pub type ChipId = i32;
type LocalPortId = i32;
pub struct PhyPortId(pub ChipId, pub LocalPortId);
pub type SwitchChip = SwitchChipTag;
pub type PhyPort = PhyPortTag;
pub type Mac = MacTag;

pub struct Device;

impl Device {
    pub fn init(&self) -> Result<Vec<SwitchChip>, SdkError> {
        let mut chips = [SwitchChip::default(); CHIP_SDK_CHIP_MAX];
        let mut chip_num = 0;
        let ret = unsafe { chip_sdk_init(chips.as_mut_ptr(), &mut chip_num) };
        if ret == ChipSdkError::CHIP_SDK_SUCCESS {
            Ok(chips[..chip_num as usize].to_vec())
        } else {
            Err(ret.into())
        }
    }
    
    pub fn register_link_status_callback(&self, cb: LinkStatusCallback) -> Result<(), SdkError> {
        let ret = unsafe { chip_sdk_register_link_status_callback(cb) };
        if ret == ChipSdkError::CHIP_SDK_SUCCESS {
            Ok(())
        } else {
            Err(ret.into())
        }
    }
    
    pub fn set_mac(&self, phy_port_id: &PhyPortId, mac: &Mac) -> Result<(), SdkError> {
        let ret = unsafe { chip_sdk_set_mac(phy_port_id.0, phy_port_id.1, mac) };
        if ret == ChipSdkError::CHIP_SDK_SUCCESS {
            Ok(())
        } else {
            Err(ret.into())
        }
    }
}