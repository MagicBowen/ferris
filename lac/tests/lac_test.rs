mod device;
use device::*;
use lac::ffi::*;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_device() {
        let mut chip = SwitchChip::new(0);
        chip.add_port(PhyPort::default()).expect("Failed to add port");
        
        let mut fixture = DeviceFixture::new();
        fixture.add_chip(chip).expect("Failed to add chip");
        let chip = fixture.get_chip(0).expect("Failed to get chip");
        assert_eq!(chip.chip_id, 0);

        let phy_port = fixture.get_phy_port(&PhyPortId(0, 0)).expect("Failed to get phy port");
        assert_eq!(phy_port.port_id, 0);

        fixture.set_link_status(&PhyPortId(0, 0), LinkStatus::LINK_UP).expect("Failed to set link status");
        assert_eq!(phy_port.status, LinkStatus::LINK_UP);
        
        fixture.setup().expect("Failed to setup device");
        
        fixture.device.as_ref().unwrap().set_mac(&PhyPortId(0, 0), &Mac{addr: [0; 6]}).expect("Failed to set mac address");
        let mac = fixture.get_mac_addr(&PhyPortId(0, 0)).expect("Failed to get mac address");
        assert_eq!(mac.addr, [0; 6]);
    }
}





