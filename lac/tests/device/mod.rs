mod bindings;
use bindings::*;

pub trait ChipTest {
    fn new(id: ChipId) -> Self;
    fn add_port(&mut self, port: PhyPort) -> Result<(), SdkError>;
}

impl ChipTest for SwitchChip {
    fn new(id: ChipId) -> Self {
        SwitchChip {
            chip_id: id,
            ports: [PhyPort::default(); CHIP_SDK_PHY_PORT_PER_CHIP],
            numOfPorts: 0,
        }
    }

    fn add_port(&mut self, port: PhyPort) -> Result<(), SdkError> {
        if self.numOfPorts as usize >= CHIP_SDK_PHY_PORT_PER_CHIP {
            return Err(SdkError::new(1, "No more port can be added".to_string()));
        }
        self.ports[self.numOfPorts as usize] = port;
        self.numOfPorts += 1;
        Ok(())
    }
}

pub trait DeviceTest {
    fn add_chip(&self, chip: SwitchChip) -> Result<(), SdkError>;
    fn get_chip(&self, chip_id: ChipId) -> Option<&SwitchChip>;
    fn get_phy_port(&self, phy_port_id: &PhyPortId) -> Option<&PhyPort>;
    fn set_link_status(&self, phy_port_id: &PhyPortId, status: LinkStatus) -> Result<(), SdkError>;
    fn get_mac_addr(&self, phy_port_id: &PhyPortId) -> Option<&Mac>;
}

impl DeviceTest for Device {
    fn add_chip(&self, chip: SwitchChip) -> Result<(), SdkError> {
        let ret = unsafe { device_add_chip(&chip as *const SwitchChipTag) };
        match ret {
            ChipSdkError::CHIP_SDK_SUCCESS => Ok(()),
            _ => Err(ret.into()),
        }
    }

    fn get_chip(&self, chip_id: ChipId) -> Option<&SwitchChip> {
        let chip = unsafe { device_get_chip(chip_id) };
        if chip.is_null() {
            None
        } else {
            Some(unsafe { &*chip })
        }
    }

    fn get_phy_port(&self, phy_port_id: &PhyPortId) -> Option<&PhyPort> {
        let port = unsafe { device_get_phy_port(phy_port_id.0, phy_port_id.1) };
        if port.is_null() {
            None
        } else {
            Some(unsafe { &*port })
        }
    }

    fn set_link_status(&self, phy_port_id: &PhyPortId, status: LinkStatus) -> Result<(), SdkError> {
        let ret = unsafe { device_set_link_status(phy_port_id.0, phy_port_id.1, status) };
        match ret {
            ChipSdkError::CHIP_SDK_SUCCESS => Ok(()),
            _ => Err(ret.into()),
        }
    }

    fn get_mac_addr(&self, phy_port_id: &PhyPortId) -> Option<&Mac> {
        let mac = unsafe { device_get_mac_addr(phy_port_id.0, phy_port_id.1) };
        if mac.is_null() {
            None
        } else {
            Some(unsafe { &*mac })
        }
    }
}