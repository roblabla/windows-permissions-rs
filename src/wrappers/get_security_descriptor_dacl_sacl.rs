use crate::{wrappers, Acl, SecurityDescriptor};
use std::io;
use std::ptr::{null_mut, NonNull};
use winapi::um::winnt::PACL;

macro_rules! get_security_descriptor_acl {
    ($f:ident) => {
        /// Wraps GetSecurityDescriptorDacl
        #[allow(non_snake_case)]
        pub fn $f(sd: &SecurityDescriptor) -> io::Result<Option<&Acl>> {
            let mut present = 0i32;
            let mut acl_ptr: PACL = null_mut();
            let mut defaulted = 0i32;

            let result = unsafe {
                winapi::um::securitybaseapi::$f(
                    sd.as_ptr(),
                    &mut present,
                    &mut acl_ptr,
                    &mut defaulted,
                )
            };

            if result == 0 {
                // Failed
                Err(io::Error::last_os_error())
            } else {
                if present == 0 {
                    // Not present
                    Ok(None)
                } else {
                    // Present
                    let acl = unsafe {
                        Acl::ref_from_nonnull(
                            NonNull::new(acl_ptr).expect("$f indicated success but returned NULL"),
                        )
                    };

                    debug_assert!(wrappers::IsValidAcl(acl));

                    Ok(Some(acl))
                }
            }
        }
    };
}

get_security_descriptor_acl!(GetSecurityDescriptorDacl);
get_security_descriptor_acl!(GetSecurityDescriptorSacl);
