#ifndef CHIP_SDK_H
#define CHIP_SDK_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * 常量定义
 */
#define CHIP_SDK_CHIP_MAX 4                 /* 最多支持的芯片数 */
#define CHIP_SDK_PHY_PORT_PER_CHIP 8        /* 每个芯片上的物理端口上限 */
#define CHIP_SDK_PHY_PORT_PER_GROUP_MAX 4   /* 最多允许聚合的物理端口数 */

/**
 * 错误码
 */
typedef enum {
    CHIP_SDK_SUCCESS = 0,   /* 成功 */
    CHIP_SDK_ERROR,         /* 通用错误 */
    CHIP_SDK_INVALID_PARAM, /* 无效参数 */
    CHIP_SDK_NO_MEMORY,     /* 内存不足 */
    CHIP_SDK_NO_RESOURCE,   /* 资源不足 */
    CHIP_SDK_NOT_FOUND,     /* 未找到 */
    CHIP_SDK_NOT_SUPPORTED, /* 不支持 */
    CHIP_SDK_BUSY,          /* 忙 */
    CHIP_SDK_TIMEOUT,       /* 超时 */
    CHIP_SDK_NO_CHANGE,     /* 无变化 */
} ChipSdkError;

/**
 * PhyPort 的 Link 状态
 */
typedef enum {
    LINK_DOWN,
    LINK_UP,
} LinkStatus;

/**
 * 物理端口
 */
typedef struct PhyPortTag {
    int port_id;       /* 端口ID，每个芯片内部唯一，从 0 开始 */
    int speed;         /* 端口速率，单位为 MB */
    LinkStatus status; /* 端口状态 */
} PhyPort;

/**
 * 交换芯片
 */
typedef struct SwitchChipTag {
    int chip_id;       /* 芯片ID，设备内部唯一，从 0 开始 */
    int numOfPorts;    /* 物理端口数 */
    PhyPort ports[CHIP_SDK_PHY_PORT_PER_CHIP]; /* 物理端口 */
} SwitchChip;

/**
 * MAC 地址
 */
typedef struct MacTag {
    unsigned char addr[6]; /* MAC 地址 */
} Mac;

/****************************************************************
 * 驱动接口
 ****************************************************************/

/**
 * @brief 初始化 SDK，获得芯片信息
 * @param SwitchChip** chips 芯片信息，出参
 * @param int* chip_num 芯片数目，出参
 * @return 错误码
 */
ChipSdkError chip_sdk_init(SwitchChip* chips, int* chip_num);

/**
 * @brief 注册 PhyPort 的 Link 状态回调
 * @param LinkStatusCallback callback 回调函数 (chip_id, port_id, status)
 * @return 错误码
 */
typedef void (*LinkStatusCallback)(int chip_id, int port_id, LinkStatus status);
ChipSdkError chip_sdk_register_link_status_callback(LinkStatusCallback callback);

/**
 * @brief 设置 PhyPort 的 MAC 地址
 * @param int chip_id 芯片ID
 * @param int port_id 端口ID
 * @param Mac mac     MAC 地址
 * @return 错误码
 */
ChipSdkError chip_sdk_set_mac(int chip_id, int port_id, const Mac* mac);

#ifdef __cplusplus
}
#endif

#endif
