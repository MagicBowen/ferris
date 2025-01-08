use crate::ffi::*;

pub struct Device {
    chips: [SwitchChip; CHIP_SDK_CHIP_MAX],
    chip_num: i32,
}

impl Device {
    pub fn new() -> Self {
        Device {
            chips: [SwitchChip::default(); CHIP_SDK_CHIP_MAX],
            chip_num: 0,
        }
    }

    pub fn activate(&mut self) -> SdkResult {
        sdk_init(&mut self.chips, &mut self.chip_num)
    }

    pub fn register_link_status_callback(&self, cb: LinkStatusCallback) -> SdkResult {
        sdk_register_link_status_callback(cb)
    }

    pub fn set_mac(&self, phy_port_id: &PhyPortId, mac: &Mac) -> SdkResult {
        sdk_set_mac(phy_port_id, mac)
    }
}
