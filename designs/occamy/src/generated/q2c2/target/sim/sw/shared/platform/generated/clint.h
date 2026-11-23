// Generated register defines for CLINT

// Copyright information found in source file:
// Copyright 2020 ETH Zurich and University of Bologna.

// Licensing information found in source file:
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

#ifndef _CLINT_REG_DEFS_
#define _CLINT_REG_DEFS_

#ifdef __cplusplus
extern "C" {
#endif
// Number of cores
#define CLINT_PARAM_NUM_CORES 37

// Register width
#define CLINT_PARAM_REG_WIDTH 32

// Machine Software Interrupt Pending  (common parameters)
#define CLINT_MSIP_P_FIELD_WIDTH 1
#define CLINT_MSIP_P_FIELDS_PER_REG 32
#define CLINT_MSIP_MULTIREG_COUNT 2

// Machine Software Interrupt Pending
#define CLINT_MSIP_0_REG_OFFSET 0x0
#define CLINT_MSIP_0_P_0_BIT 0
#define CLINT_MSIP_0_P_1_BIT 1
#define CLINT_MSIP_0_P_2_BIT 2
#define CLINT_MSIP_0_P_3_BIT 3
#define CLINT_MSIP_0_P_4_BIT 4
#define CLINT_MSIP_0_P_5_BIT 5
#define CLINT_MSIP_0_P_6_BIT 6
#define CLINT_MSIP_0_P_7_BIT 7
#define CLINT_MSIP_0_P_8_BIT 8
#define CLINT_MSIP_0_P_9_BIT 9
#define CLINT_MSIP_0_P_10_BIT 10
#define CLINT_MSIP_0_P_11_BIT 11
#define CLINT_MSIP_0_P_12_BIT 12
#define CLINT_MSIP_0_P_13_BIT 13
#define CLINT_MSIP_0_P_14_BIT 14
#define CLINT_MSIP_0_P_15_BIT 15
#define CLINT_MSIP_0_P_16_BIT 16
#define CLINT_MSIP_0_P_17_BIT 17
#define CLINT_MSIP_0_P_18_BIT 18
#define CLINT_MSIP_0_P_19_BIT 19
#define CLINT_MSIP_0_P_20_BIT 20
#define CLINT_MSIP_0_P_21_BIT 21
#define CLINT_MSIP_0_P_22_BIT 22
#define CLINT_MSIP_0_P_23_BIT 23
#define CLINT_MSIP_0_P_24_BIT 24
#define CLINT_MSIP_0_P_25_BIT 25
#define CLINT_MSIP_0_P_26_BIT 26
#define CLINT_MSIP_0_P_27_BIT 27
#define CLINT_MSIP_0_P_28_BIT 28
#define CLINT_MSIP_0_P_29_BIT 29
#define CLINT_MSIP_0_P_30_BIT 30
#define CLINT_MSIP_0_P_31_BIT 31

// Machine Software Interrupt Pending
#define CLINT_MSIP_1_REG_OFFSET 0x4
#define CLINT_MSIP_1_P_32_BIT 0
#define CLINT_MSIP_1_P_33_BIT 1
#define CLINT_MSIP_1_P_34_BIT 2
#define CLINT_MSIP_1_P_35_BIT 3
#define CLINT_MSIP_1_P_36_BIT 4

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW0_REG_OFFSET 0x4000

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH0_REG_OFFSET 0x4004

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW1_REG_OFFSET 0x4008

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH1_REG_OFFSET 0x400c

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW2_REG_OFFSET 0x4010

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH2_REG_OFFSET 0x4014

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW3_REG_OFFSET 0x4018

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH3_REG_OFFSET 0x401c

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW4_REG_OFFSET 0x4020

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH4_REG_OFFSET 0x4024

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW5_REG_OFFSET 0x4028

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH5_REG_OFFSET 0x402c

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW6_REG_OFFSET 0x4030

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH6_REG_OFFSET 0x4034

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW7_REG_OFFSET 0x4038

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH7_REG_OFFSET 0x403c

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW8_REG_OFFSET 0x4040

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH8_REG_OFFSET 0x4044

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW9_REG_OFFSET 0x4048

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH9_REG_OFFSET 0x404c

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW10_REG_OFFSET 0x4050

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH10_REG_OFFSET 0x4054

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW11_REG_OFFSET 0x4058

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH11_REG_OFFSET 0x405c

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW12_REG_OFFSET 0x4060

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH12_REG_OFFSET 0x4064

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW13_REG_OFFSET 0x4068

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH13_REG_OFFSET 0x406c

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW14_REG_OFFSET 0x4070

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH14_REG_OFFSET 0x4074

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW15_REG_OFFSET 0x4078

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH15_REG_OFFSET 0x407c

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW16_REG_OFFSET 0x4080

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH16_REG_OFFSET 0x4084

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW17_REG_OFFSET 0x4088

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH17_REG_OFFSET 0x408c

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW18_REG_OFFSET 0x4090

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH18_REG_OFFSET 0x4094

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW19_REG_OFFSET 0x4098

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH19_REG_OFFSET 0x409c

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW20_REG_OFFSET 0x40a0

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH20_REG_OFFSET 0x40a4

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW21_REG_OFFSET 0x40a8

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH21_REG_OFFSET 0x40ac

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW22_REG_OFFSET 0x40b0

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH22_REG_OFFSET 0x40b4

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW23_REG_OFFSET 0x40b8

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH23_REG_OFFSET 0x40bc

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW24_REG_OFFSET 0x40c0

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH24_REG_OFFSET 0x40c4

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW25_REG_OFFSET 0x40c8

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH25_REG_OFFSET 0x40cc

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW26_REG_OFFSET 0x40d0

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH26_REG_OFFSET 0x40d4

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW27_REG_OFFSET 0x40d8

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH27_REG_OFFSET 0x40dc

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW28_REG_OFFSET 0x40e0

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH28_REG_OFFSET 0x40e4

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW29_REG_OFFSET 0x40e8

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH29_REG_OFFSET 0x40ec

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW30_REG_OFFSET 0x40f0

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH30_REG_OFFSET 0x40f4

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW31_REG_OFFSET 0x40f8

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH31_REG_OFFSET 0x40fc

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW32_REG_OFFSET 0x4100

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH32_REG_OFFSET 0x4104

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW33_REG_OFFSET 0x4108

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH33_REG_OFFSET 0x410c

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW34_REG_OFFSET 0x4110

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH34_REG_OFFSET 0x4114

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW35_REG_OFFSET 0x4118

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH35_REG_OFFSET 0x411c

// Machine Timer Compare
#define CLINT_MTIMECMP_LOW36_REG_OFFSET 0x4120

// Machine Timer Compare
#define CLINT_MTIMECMP_HIGH36_REG_OFFSET 0x4124

// Timer Register Low
#define CLINT_MTIME_LOW_REG_OFFSET 0xbff8

// Timer Register High
#define CLINT_MTIME_HIGH_REG_OFFSET 0xbffc

#ifdef __cplusplus
}  // extern "C"
#endif
#endif  // _CLINT_REG_DEFS_
        // End generated register defines for CLINT