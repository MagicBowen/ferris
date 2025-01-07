#include "chip_cfg.h"
#include "device.h"

ChipSdkError device_add_chip(const SwitchChip* chip) {
    if (!chip) {
        return CHIP_SDK_INVALID_PARAM;
    }
    return ChipSdkStubs::Device::GetInstance().AddChip(*chip);
}

const SwitchChip* device_get_chip(int chip_id) {
    return ChipSdkStubs::Device::GetInstance().GetChip(chip_id);
}

const PhyPort* device_get_phy_port(int chip_id, int port_id) {
    return ChipSdkStubs::Device::GetInstance().GetPhyPort(chip_id, port_id);
}

ChipSdkError device_set_link_status(int chip_id, int port_id, LinkStatus status) {
    return ChipSdkStubs::Device::GetInstance().SetLinkStatus(chip_id, port_id, status);
}

const Mac* device_get_mac_addr(int chip_id, int port_id) {
    return ChipSdkStubs::Device::GetInstance().GetMac(chip_id, port_id);
}
