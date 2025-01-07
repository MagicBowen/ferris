#include "chip_sdk.h"
#include "device.h"

ChipSdkError chip_sdk_init(SwitchChip* chips, int* chip_num) {
    return ChipSdkStubs::Device::GetInstance().GetChips(chips, chip_num);
}

ChipSdkError chip_sdk_register_link_status_callback(LinkStatusCallback callback) {
    return ChipSdkStubs::Device::GetInstance().SetLinkStatusCallback(callback);
}

ChipSdkError chip_sdk_set_mac(int chip_id, int port_id, const Mac* mac) {
    return ChipSdkStubs::Device::GetInstance().SetMac(chip_id, port_id, *mac);
}
