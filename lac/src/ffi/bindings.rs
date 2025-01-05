#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused)]

use std::os::raw::{c_int, c_uchar};

pub const CHIP_SDK_CHIP_MAX: usize = 4;
pub const CHIP_SDK_PHY_PORT_PER_CHIP: usize = 8;
pub const CHIP_SDK_PHY_PORT_PER_GROUP_MAX: usize = 4;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ChipSdkError {
    CHIP_SDK_SUCCESS = 0,
    CHIP_SDK_ERROR,
    CHIP_SDK_INVALID_PARAM,
    CHIP_SDK_NO_MEMORY,
    CHIP_SDK_NO_RESOURCE,
    CHIP_SDK_NOT_FOUND,
    CHIP_SDK_NOT_SUPPORTED,
    CHIP_SDK_BUSY,
    CHIP_SDK_TIMEOUT,
    CHIP_SDK_NO_CHANGE,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum LinkStatus {
    #[default]
    LINK_DOWN = 0,
    LINK_UP = 1,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct PhyPortTag {
    pub port_id: c_int,
    pub speed: c_int,
    pub status: LinkStatus,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct SwitchChipTag {
    pub chip_id: c_int,
    pub numOfPorts: c_int,
    pub ports: [PhyPortTag; CHIP_SDK_PHY_PORT_PER_CHIP],
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct MacTag {
    pub addr: [c_uchar; 6],
}

pub type LinkStatusCallback = extern "C" fn(chip_id: c_int, port_id: c_int, status: LinkStatus);

extern "C" {
    pub fn chip_sdk_init(chips: *mut SwitchChipTag, chip_num: *mut c_int) -> ChipSdkError;
    pub fn chip_sdk_register_link_status_callback(cb: LinkStatusCallback) -> ChipSdkError;
    pub fn chip_sdk_set_mac(chip_id: c_int, port_id: c_int, mac: *const MacTag) -> ChipSdkError;
}
