//! UNIXPermissionsExt
//!
//! A trivial trait bringing missing functions that are not exposed by
//! PermissionsExt to Permissions on UNIX platforms.

mod constants;
pub mod raw_fn;

use raw_fn::*;
use std::os::unix::fs::PermissionsExt;

/// Missing functions that are not exposed by
/// [`std::os::unix::fs::PermisionsExt`](https://doc.rust-lang.org/std/os/unix/fs/trait.PermissionsExt.html).
#[cfg(target_family = "unix")]
pub trait UNIXPermissionsExt {
    /// Is set-UID bit set?
    fn set_uid(&self) -> bool;
    /// Is set-GID bit set?
    fn set_gid(&self) -> bool;
    /// Is sticky bit set?
    fn sticky_bit(&self) -> bool;

    /// Is this file readable by owner?
    fn readable_by_owner(&self) -> bool;
    /// Is this file writable by owner?
    fn writable_by_owner(&self) -> bool;
    /// Is this file executable by owner?
    fn executable_by_owner(&self) -> bool;

    /// Is this file readable by group?
    fn readable_by_group(&self) -> bool;
    /// Is this file writable by group?
    fn writable_by_group(&self) -> bool;
    /// Is this file executable by group?
    fn executable_by_group(&self) -> bool;

    /// Is this file readable by others?
    fn readable_by_other(&self) -> bool;
    /// Is this file writable by others?
    fn writable_by_other(&self) -> bool;
    /// Is this file executable by others?
    fn executable_by_other(&self) -> bool;

    /// Convert Permissions into a `String`, just like the one printed by
    /// `ls(1)`.
    fn stringify(&self) -> String;
}

impl UNIXPermissionsExt for std::fs::Permissions {
    #[inline]
    fn set_uid(&self) -> bool {
        set_uid(self.mode())
    }

    #[inline]
    fn set_gid(&self) -> bool {
        set_gid(self.mode())
    }
    #[inline]
    fn sticky_bit(&self) -> bool {
        sticky_bit(self.mode())
    }

    #[inline]
    fn readable_by_owner(&self) -> bool {
        readable_by_owner(self.mode())
    }
    #[inline]
    fn writable_by_owner(&self) -> bool {
        writable_by_owner(self.mode())
    }
    #[inline]
    fn executable_by_owner(&self) -> bool {
        executable_by_owner(self.mode())
    }

    #[inline]
    fn readable_by_group(&self) -> bool {
        readable_by_group(self.mode())
    }
    #[inline]
    fn writable_by_group(&self) -> bool {
        writable_by_group(self.mode())
    }
    #[inline]
    fn executable_by_group(&self) -> bool {
        executable_by_owner(self.mode())
    }

    #[inline]
    fn readable_by_other(&self) -> bool {
        readable_by_other(self.mode())
    }
    #[inline]
    fn writable_by_other(&self) -> bool {
        writable_by_other(self.mode())
    }
    #[inline]
    fn executable_by_other(&self) -> bool {
        executable_by_other(self.mode())
    }

    fn stringify(&self) -> String {
        stringify(self.mode())
    }
}

#[cfg(test)]
mod test {
    use super::UNIXPermissionsExt;
    use std::{
        ffi::OsStr,
        fs::{metadata, remove_file, File},
        os::unix::ffi::OsStrExt,
        process::Command,
    };

    #[test]
    // used for testing `readable_by_*()` and `executable_by_*()`
    fn test_ls() {
        if let Ok(output) = Command::new("which").arg("ls").output() {
            assert!(output.stderr.is_empty());
            assert!(!output.stdout.is_empty());
            let ls_path = <OsStr as OsStrExt>::from_bytes(
                &output.stdout[..output.stdout.len() - 1],
            );

            let metadata = metadata(ls_path).expect("can not get ls metadata");
            let permission = metadata.permissions();

            assert!(permission.readable_by_owner());
            assert!(permission.readable_by_group());
            assert!(permission.readable_by_other());

            assert!(permission.executable_by_owner());
            assert!(permission.executable_by_group());
            assert!(permission.executable_by_other());
        }
    }

    #[test]
    // test `set_uid()`
    fn test_su() {
        if let Ok(output) = Command::new("which").arg("su").output() {
            assert!(output.stderr.is_empty());
            assert!(!output.stdout.is_empty());
            let su_path = <OsStr as OsStrExt>::from_bytes(
                &output.stdout[..output.stdout.len() - 1],
            );

            let metadata = metadata(su_path).expect("can not get su metadata");
            let permission = metadata.permissions();

            assert!(permission.set_uid());
        }
    }

    #[test]
    // test `writable_by_*()`
    fn test_dev_null() {
        let metadata =
            metadata("/dev/null").expect("can not get /dev/null metadata");
        let permission = metadata.permissions();

        assert!(permission.writable_by_owner());
        assert!(permission.writable_by_group());
        assert!(permission.writable_by_other());
    }

    #[test]
    // test `set_gid()` and `sticky_bit()`
    fn test_set_gid_and_sticky_bit() {
        let tmp_file = File::create("/tmp/iugxacxgx")
            .expect("can not create temporary file");

        Command::new("chmod")
            .args(["g+s", "/tmp/iugxacxgx"])
            .status()
            .expect("can not execute chmod");

        Command::new("chmod")
            .args(["o+t", "/tmp/iugxacxgx"])
            .status()
            .expect("can not execute chmod");

        let metadata = tmp_file
            .metadata()
            .expect("can not get temporary file metadata");

        let permission = metadata.permissions();

        assert!(permission.set_gid());
        assert!(permission.sticky_bit());

        remove_file("/tmp/iugxacxgx").unwrap();
    }

    #[test]
    // test `to_string()`
    fn test_to_string() {
        let tmp_file = File::create("/tmp/ugxacxgx")
            .expect("can not create temporary file");

        Command::new("chmod")
            .args(["777", "/tmp/ugxacxgx"])
            .status()
            .expect("can not execute chmod");
        let mut metadata;
        let mut permission;

        Command::new("chmod")
            .args(["+s", "/tmp/ugxacxgx"])
            .status()
            .expect("can not execute chmod");
        metadata = tmp_file
            .metadata()
            .expect("can not get temporary file metadata");
        permission = metadata.permissions();
        assert_eq!(permission.stringify(), "rwsrwsrwx");

        Command::new("chmod")
            .args(["o+t", "/tmp/ugxacxgx"])
            .status()
            .expect("can not execute chmod");
        metadata = tmp_file
            .metadata()
            .expect("can not get temporary file metadata");
        permission = metadata.permissions();
        assert_eq!(permission.stringify(), "rwsrwsrwt");

        Command::new("chmod")
            .args(["-x", "/tmp/ugxacxgx"])
            .status()
            .expect("can not execute chmod");
        metadata = tmp_file
            .metadata()
            .expect("can not get temporary file metadata");
        permission = metadata.permissions();
        assert_eq!(permission.stringify(), "rwSrwSrwT");

        remove_file("/tmp/ugxacxgx").unwrap();
    }
}
