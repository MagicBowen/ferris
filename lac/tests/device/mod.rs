mod bindings;
use bindings::*;

pub trait ChipFixture {
    fn new(id: ChipId) -> Self;
    fn add_port(&mut self, port: PhyPort) -> SdkResult;
}

impl ChipFixture for SwitchChip {
    fn new(id: ChipId) -> Self {
        SwitchChip {
            chip_id: id,
            ports: [PhyPort::default(); CHIP_SDK_PHY_PORT_PER_CHIP],
            numOfPorts: 0,
        }
    }

    fn add_port(&mut self, port: PhyPort) -> SdkResult {
        if self.numOfPorts as usize >= CHIP_SDK_PHY_PORT_PER_CHIP {
            return Err(ChipSdkError::CHIP_SDK_NO_RESOURCE);
        }
        self.ports[self.numOfPorts as usize] = port;
        self.numOfPorts += 1;
        Ok(())
    }
}

macro_rules! ptr_to_option {
    ($ptr:expr) => {
        if $ptr.is_null() {
            None
        } else {
            Some(unsafe { &*($ptr) })
        }
    };
}

pub struct DeviceFixture {
    pub device: Option<Device>,
}

impl DeviceFixture {
    pub fn new() -> Self {
        DeviceFixture {
            device: None,
        }
    }

    pub fn activate_device(&mut self) -> SdkResult {
        assert!(self.device.is_none(), "Device should be activated only once");
        self.device = Some(Device::new());
        SDK_OK
    }

    pub fn add_chip(&self, chip: SwitchChip) -> SdkResult {
        assert!(self.device.is_none(), "Chip should be added before device activation");
        unsafe { device_add_chip(&chip as *const SwitchChipTag).to_result() }
    }

    pub fn get_chip(&self, chip_id: ChipId) -> Option<&'static SwitchChip> {
        let chip = unsafe { device_get_chip(chip_id) };
        ptr_to_option!(chip)
    }

    pub fn get_phy_port(&self, phy_port_id: &PhyPortId) -> Option<&'static PhyPort> {
        let phy_port = unsafe { device_get_phy_port(phy_port_id.0, phy_port_id.1) };
        ptr_to_option!(phy_port)
    }

    pub fn set_link_status(&self, phy_port_id: &PhyPortId, status: LinkStatus) -> SdkResult {
        assert!(self.device.is_some(), "Link status should be set after device activation");
        unsafe { device_set_link_status(phy_port_id.0, phy_port_id.1, status).to_result() }
    }

    pub fn get_mac_addr(&self, phy_port_id: &PhyPortId) -> Option<&Mac> {
        assert!(self.device.is_some(), "Mac address should be set after device activation");
        let mac = unsafe { device_get_mac_addr(phy_port_id.0, phy_port_id.1) };
        ptr_to_option!(mac)
    }
}