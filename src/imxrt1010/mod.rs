//! Pads for the i.MX RT 1010 processor family
//!
//! The module exports all of the i.MX RT 1010 processor's pads. Pads that can support peripheral
//! functions are tagged with `imxrt-iomuxc` traits.
//!
//! # Example
//!
//! In the example below, we implement a hypothetical `uart_new` function, which is responsible
//! for preparing a UART peripheral. To properly configure the peripheral, we need the two
//! pads that represent a peripheral's TX and RX pins. The implementation will use the
//! `imxrt_iomuxc::lpuart::prepare()` function to prepare the pins.
//!
//! Note the trait bounds on `uart_new`. The usage requires that
//!
//! - the user provides one TX and one RX pin
//! - the modules for each pin match
//!
//! ```no_run
//! use imxrt_iomuxc as iomuxc;
//! use iomuxc::lpuart::{Pin, Tx, Rx};
//!
//! # struct UART;
//! /// Creates a UART peripheral from the TX and RX pads, and a baud rate
//! fn uart_new<T, R>(mut tx: T, mut rx: R, baud: u32) -> UART
//! where
//!     T: Pin<Direction = Tx>,
//!     R: Pin<Direction = Rx, Module = <T as Pin>::Module>,
//! {
//!     // Check the imxrt-iomuxc documentation to understand why
//!     // this is unsafe.
//!     unsafe {
//!         iomuxc::lpuart::prepare(&mut tx);
//!         iomuxc::lpuart::prepare(&mut rx);
//!     }
//!     // Prepare the rest of the UART peripheral, and return it...
//!     # UART
//! }
//!
//! # let gpio_sd_12 = unsafe { imxrt_iomuxc::imxrt1010::gpio_sd::GPIO_SD_12::new() };
//! # let gpio_sd_11 = unsafe { imxrt_iomuxc::imxrt1010::gpio_sd::GPIO_SD_11::new() };
//! // GPIO_SD_12 and GPIO_SD_11 are a suitable pair of UART pins
//! uart_new(gpio_sd_12, gpio_sd_11, 115_200);
//! ```
//!
//! Specifically, the trait bounds guard against non-UART pins:
//!
//! ```compile_fail
//! # use imxrt_iomuxc as iomuxc;
//! # use iomuxc::lpuart::{Pin, Tx, Rx};
//! # struct UART;
//! # fn uart_new<T, R>(mut tx: T, mut rx: R, baud: u32) -> UART
//! # where
//! #     T: Pin<Direction = Tx>,
//! #     R: Pin<Direction = Rx, Module = <T as Pin>::Module>,
//! # {
//! #     unsafe {
//! #         iomuxc::lpuart::prepare(&mut tx);
//! #         iomuxc::lpuart::prepare(&mut rx);
//! #     }
//! #     UART
//! # }
//! # let gpio_ad_10 = unsafe { imxrt_iomuxc::imxrt1010::gpio_ad::GPIO_AD_10::new() };
//! # let gpio_ad_11 = unsafe { imxrt_iomuxc::imxrt1010::gpio_ad::GPIO_AD_11::new() };
//! // Neither pad is a valid UART pin
//! uart_new(gpio_ad_10, gpio_ad_11, 115_200);
//! ```
//!
//! It also guards against mismatched UART pins:
//!
//! ```compile_fail
//! # use imxrt_iomuxc as iomuxc;
//! # use iomuxc::lpuart::{Pin, Tx, Rx};
//! # struct UART;
//! # fn uart_new<T, R>(mut tx: T, mut rx: R, baud: u32) -> UART
//! # where
//! #     T: Pin<Direction = Tx>,
//! #     R: Pin<Direction = Rx, Module = <T as Pin>::Module>,
//! # {
//! #     unsafe {
//! #         iomuxc::lpuart::prepare(&mut tx);
//! #         iomuxc::lpuart::prepare(&mut rx);
//! #     }
//! #     UART
//! # }
//! # let gpio_09 = unsafe { imxrt_iomuxc::imxrt1010::gpio::GPIO_09::new() };
//! # let gpio_13 = unsafe { imxrt_iomuxc::imxrt1010::gpio::GPIO_13::new() };
//! // GPIO_10 is a UART1 TX pin, and GPIO_13 is a UART2 RX pin
//! uart_new(gpio_10, gpio_13, 115_200);
//! ```

mod lpi2c;
mod lpspi;
mod lpuart;

include!(concat!(env!("OUT_DIR"), "/imxrt1010.rs"));
pub use pads::*;

mod bases {
    define_base!(GPIO_AD, 0x401F_8010, 0x401F_80C0);
    define_base!(GPIO_SD, 0x401F_804C, 0x401F_80FC);
    define_base!(GPIO, 0x401F_8088, 0x401F_8138);
}
