//! This build script detects target platforms that lack proper support for
//! atomics and sets `cfg` flags accordingly.

use std::env;

fn main() {
    let target = match rustc_target() {
        Some(target) => target,
        None => return,
    };

    // If the target isn't thumbv6 then we can use atomic CAS
    if !target.starts_with("thumbv6") {
        println!("cargo:rustc-cfg=atomic_cas");
    }
}

fn rustc_target() -> Option<String> {
    env::var("TARGET").ok()
}
