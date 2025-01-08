// ffi of c_stubs/chip_cfg.h

#![allow(unused)]

pub use lac::ffi::*;
use std::os::raw::c_int;

extern "C" {
    pub fn device_add_chip(chip: *const SwitchChipTag) -> ChipSdkError;
    pub fn device_get_chip(chip_id: c_int) -> *const SwitchChipTag;
    pub fn device_get_phy_port(chip_id: c_int, port_id: c_int) -> *const PhyPortTag;
    pub fn device_set_link_status(
        chip_id: c_int,
        port_id: c_int,
        status: LinkStatus,
    ) -> ChipSdkError;
    pub fn device_get_mac_addr(chip_id: c_int, port_id: c_int) -> *const MacTag;
}
