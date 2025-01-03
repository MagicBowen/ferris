#include "device.h"

namespace ChipSdkStubs {

ChipSdkError Device::AddChip(const SwitchChip& chip) {
    if (chips_.size() >= CHIP_SDK_CHIP_MAX) {
        return CHIP_SDK_NO_RESOURCE;
    }
    chips_.push_back(chip);
    return CHIP_SDK_SUCCESS;
}

ChipSdkError Device::GetChips(SwitchChip* chips, int* chip_num) const {
    if (!chip_num || !chips) {
        return CHIP_SDK_INVALID_PARAM;
    }
    if (chips) {
        for (size_t i = 0; i < chips_.size(); ++i) {
            chips[i] = chips_[i];
        }
    }
    *chip_num = chips_.size();
    return CHIP_SDK_SUCCESS;
}

const PhyPort* Device::GetPhyPort(int chip_id, int port_id) const {
    if (chip_id < 0 || (std::size_t)chip_id >= chips_.size()) {
        return nullptr;
    }
    if (port_id < 0 || port_id >= chips_[chip_id].numOfPorts) {
        return nullptr;
    }
    return &chips_[chip_id].ports[port_id];
}

ChipSdkError Device::SetLinkStatusCallback(LinkStatusCallback callback) {
    link_status_callback_ = callback;
    return CHIP_SDK_SUCCESS;
}

ChipSdkError Device::SetLinkStatus(int chip_id, int port_id, LinkStatus status) {
    if (chip_id < 0 || (std::size_t)chip_id >= chips_.size()) {
        return CHIP_SDK_INVALID_PARAM;
    }
    if (port_id < 0 || port_id >= chips_[chip_id].numOfPorts) {
        return CHIP_SDK_INVALID_PARAM;
    }
    auto original_status = chips_[chip_id].ports[port_id].status;
    chips_[chip_id].ports[port_id].status = status;
    if (original_status != status && link_status_callback_) {
        link_status_callback_(chip_id, port_id, status);
    }
    return CHIP_SDK_SUCCESS;
}

ChipSdkError Device::SetMac(int chip_id, int port_id, const Mac& mac) {
    macs_[{chip_id, port_id}] = mac;
    return CHIP_SDK_SUCCESS;
}

const Mac* Device::GetMac(int chip_id, int port_id) const {
    auto it = macs_.find({chip_id, port_id});
    if (it == macs_.end()) {
        return nullptr;
    }
    return &it->second;
}

}


ChipSdkError chip_sdk_init(SwitchChip* chips, int* chip_num) {
    return ChipSdkStubs::Device::GetInstance().GetChips(chips, chip_num);
}

ChipSdkError chip_sdk_register_link_status_callback(LinkStatusCallback callback) {
    return ChipSdkStubs::Device::GetInstance().SetLinkStatusCallback(callback);
}

ChipSdkError chip_sdk_set_mac(int chip_id, int port_id, const Mac* mac) {
    return ChipSdkStubs::Device::GetInstance().SetMac(chip_id, port_id, *mac);
}
