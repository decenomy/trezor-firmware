#ifndef __TREZORHAL_COMMON_H__
#define __TREZORHAL_COMMON_H__

#include <stdint.h>

#define BOARDLOADER_START  0x08000000
#define BOOTLOADER_START   0x08010000
#define FIRMWARE_START     0x08020000
#define HEADER_SIZE        0x200

extern void memset_reg(volatile void *start, volatile void *stop, uint32_t val);

void clear_peripheral_local_memory(void);

void periph_init(void);

void __attribute__((noreturn)) __fatal_error(const char *msg, const char *file, int line, const char *func);

void __attribute__((noreturn)) nlr_jump_fail(void *val);

void jump_to(uint32_t address);

void hal_delay(uint32_t ms);

#endif
