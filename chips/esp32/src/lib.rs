// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2022.

// // https://www.espressif.com/sites/default/files/documentation/esp32_technical_reference_manual_en.pdf

//! Drivers and chip support for Espressif ESP32 boards.

#![no_std]
#![crate_name = "esp32"]
#![crate_type = "rlib"]

pub mod chip;
pub mod gpio;
pub mod intc;
pub mod interrupts;
pub mod rng;
pub mod rtc_cntl;
pub mod sysreg;
pub mod timg;
pub mod uart;
