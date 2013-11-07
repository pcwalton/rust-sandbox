// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[link(name = "sandbox",
       vers = "0.1")];
#[crate_type = "lib"];

extern mod std;

#[cfg(target_os="macos")]
pub use enter = platform::macos::enter;

pub mod platform {
    #[cfg(target_os="macos")]
    pub mod macos;
}

#[deriving(Clone, Eq)]
pub enum SandboxMode {
    RestrictedMode,
    OpenGLMode,
}

#[cfg(not(target_os="macos"))]
pub fn enter(_: SandboxMode) {
    // No-op.
    //
    // FIXME(pcwalton): Sandbox on Windows and Linux.
}

