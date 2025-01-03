/**
* Copyright (c) wangbo@joycode.art 2024
*/

#ifndef DEVICE_H
#define DEVICE_H

#include "chip_sdk.h"
#include <unordered_map>
#include <vector>

namespace ChipSdkStubs {

struct PhyPortId {
    PhyPortId(int chip_id, int port_id) : chip_id(chip_id), port_id(port_id) {}

    bool operator==(const PhyPortId& other) const {
        return chip_id == other.chip_id && port_id == other.port_id;
    }

private:    
    friend struct PhyPortIdHash;
    int chip_id;
    int port_id;
};

struct PhyPortIdHash {
    size_t operator()(const PhyPortId& phy_port_id) const {
        return std::hash<int>()(phy_port_id.chip_id) ^ std::hash<int>()(phy_port_id.port_id);
    }
};
    

struct Device {
    static Device& GetInstance() {
        static Device instance;
        return instance;
    }

    ChipSdkError AddChip(const SwitchChip& chip);
    ChipSdkError GetChips(SwitchChip* chips, int* chip_num) const;
    const PhyPort* GetPhyPort(int chip_id, int port_id) const;
    ChipSdkError SetLinkStatusCallback(LinkStatusCallback callback);
    ChipSdkError SetLinkStatus(int chip_id, int port_id, LinkStatus status);
    ChipSdkError SetMac(int chip_id, int port_id, const Mac& mac);
    const Mac* GetMac(int chip_id, int port_id) const;

private:
    Device() {
        chips_.reserve(CHIP_SDK_CHIP_MAX);
    }

private:
    std::vector<SwitchChip> chips_;
    std::unordered_map<PhyPortId, Mac, PhyPortIdHash> macs_;
    LinkStatusCallback link_status_callback_;
};

}

#endif
