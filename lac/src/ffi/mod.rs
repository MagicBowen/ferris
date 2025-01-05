#![allow(unused)]

#[cfg(feature = "use_bindgen")]
mod bindings_gen;
#[cfg(feature = "use_bindgen")]
use bindings_gen::*;

#[cfg(not(feature = "use_bindgen"))]
mod bindings;
#[cfg(not(feature = "use_bindgen"))]
use bindings::*;

pub type SwitchChip = SwitchChipTag;
pub type PhyPort = PhyPortTag;
pub type Mac = MacTag;

pub struct PhyPortId(i32, i32);

pub fn init_sdk() -> Result<Vec<SwitchChip>, ChipSdkError> {
    let mut chips = [SwitchChip::default(); CHIP_SDK_CHIP_MAX];
    let mut chip_num = 0;
    let ret = unsafe { chip_sdk_init(chips.as_mut_ptr(), &mut chip_num) };
    if ret == ChipSdkError::CHIP_SDK_SUCCESS {
        Ok(chips[..chip_num as usize].to_vec())
    } else {
        Err(ret)
    }
}

pub fn register_link_status_callback(cb: LinkStatusCallback) -> Result<(), ChipSdkError> {
    let ret = unsafe { chip_sdk_register_link_status_callback(cb) };
    if ret == ChipSdkError::CHIP_SDK_SUCCESS {
        Ok(())
    } else {
        Err(ret)
    }
}

pub fn set_mac(phy_port_id: &PhyPortId, mac: &Mac) -> Result<(), ChipSdkError> {
    let ret = unsafe { chip_sdk_set_mac(phy_port_id.0, phy_port_id.1, mac) };
    if ret == ChipSdkError::CHIP_SDK_SUCCESS {
        Ok(())
    } else {
        Err(ret)
    }
}