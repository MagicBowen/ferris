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
        
        let device: Device = Device;
        device.add_chip(chip).expect("Failed to add chip");
        let chip = device.get_chip(0).expect("Failed to get chip");
        assert_eq!(chip.chip_id, 0);

        let phy_port = device.get_phy_port(&PhyPortId(0, 0)).expect("Failed to get phy port");
        assert_eq!(phy_port.port_id, 0);

        device.set_link_status(&PhyPortId(0, 0), LinkStatus::LINK_UP).expect("Failed to set link status");
        assert_eq!(phy_port.status, LinkStatus::LINK_UP);

        device.set_mac(&PhyPortId(0, 0), &Mac{addr: [0; 6]}).expect("Failed to set mac address");
        let mac = device.get_mac_addr(&PhyPortId(0, 0)).expect("Failed to get mac address");
        assert_eq!(mac.addr, [0; 6]);
    }
}





