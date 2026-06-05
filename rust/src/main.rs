// SPDX-FileCopyrightText: 2026 seL4 Project a Series of LF Projects, LLC
// SPDX-License-Identifier: BSD-2-Clause

//! seL4 Kernel Binary
//!
//! Entry point for the seL4 microkernel executable.
//! This is a placeholder for the kernel entry point during the Rust rewrite process.

#![no_std]
#![no_main]

use sel4_kernel::kernel;

/// Kernel entry point
/// 
/// This function serves as the entry point for the seL4 kernel.
/// Currently a placeholder during the C to Rust migration process.
#[no_mangle]
pub extern "C" fn kernel_entry() {
    kernel::init();
}

/// Panic handler for the kernel
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
