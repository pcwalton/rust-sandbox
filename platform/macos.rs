// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use SandboxMode;

use std::c_str::CString;
use std::libc::{c_char, c_int};
use std::ptr;

static SANDBOX_TEMPLATE: &'static str = "(version 1)

(deny default)

(allow file*
    (literal \"/dev/dtracehelper\")
    (literal \"/dev/urandom\")
    (literal \"/dev/null\"))

; FIXME(pcwalton): Needs to wait for resource messages to become serializable.
(allow file-read*
    (subpath \"/\"))

(allow file-write*
    (subpath \"/private/var\"))

(allow sysctl-read)
(allow signal (target self))
(allow ipc-posix-shm)
(allow mach-lookup (global-name \"com.apple.FontServer\"))

; TODO(pcwalton): This may give too much power to the client. It seems Mac has a way of securely
; transferring ownership of IOSurfaces and we should use that instead.
(allow iokit-open
    (iokit-user-client-class \"IOSurfaceRootUserClient\"))

; FIXME(pcwalton): Needs to wait for resource messages to be serializable.
(allow network-outbound)

(debug deny)";

static SANDBOX_GL_TEMPLATE: &'static str = "
(allow iokit-open
    (iokit-connection \"IOAccelerator\")
    (iokit-user-client-class \"AGPMClient\")
    (iokit-user-client-class \"AppleGraphicsControlClient\")
    (iokit-user-client-class \"IOAccelerationUserClient\")
    (iokit-user-client-class \"IOFramebufferSharedUserClient\")
    (iokit-user-client-class \"IOHIDParamUserClient\")
    (iokit-user-client-class \"IOSurfaceSendRight\")
    (iokit-user-client-class \"RootDomainUserClient\"))";

extern {
    fn sandbox_init(profile: *c_char, flags: u64, error_buf: *mut *c_char) -> c_int;
}

#[fixed_stack_segment]
pub fn enter(mode: SandboxMode) {
    unsafe {
        let mut sandbox_descriptor = SANDBOX_TEMPLATE.to_str();
        match mode {
            RestrictedMode => {}
            OpenGLMode => sandbox_descriptor.push_str(SANDBOX_GL_TEMPLATE),
        }

        let c_str = sandbox_descriptor.to_c_str();
        let mut error_buf = ptr::null();
        c_str.with_ref(|c_buffer| {
            if sandbox_init(c_buffer, 0, &mut error_buf) != 0 {
                let c_str = CString::new(error_buf, false);
                let string = c_str.as_str().unwrap();
                fail!("sandbox creation failed: {}", string);
            }
        })
    }
}

