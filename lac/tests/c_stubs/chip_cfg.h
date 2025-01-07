#ifndef CHIP_CFG_H
#define CHIP_CFG_H

#include "chip_sdk.h"

#ifdef __cplusplus
extern "C" {
#endif

ChipSdkError device_add_chip(const SwitchChip* chip);
const SwitchChip* device_get_chip(int chip_id);

const PhyPort* device_get_phy_port(int chip_id, int port_id);

ChipSdkError device_set_link_status(int chip_id, int port_id, LinkStatus status);
const Mac* device_get_mac_addr(int chip_id, int port_id);

#ifdef __cplusplus
}
#endif

#endif
