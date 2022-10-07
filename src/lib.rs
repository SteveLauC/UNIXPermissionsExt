mod constants;

use constants::*;
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

    /// Is this file readable by others
    fn readable_by_other(&self) -> bool;
    /// Is this file writable by others
    fn writable_by_other(&self) -> bool;
    /// Is this file executable by others
    fn executable_by_other(&self) -> bool;
}

impl UNIXPermissionsExt for std::fs::Permissions {
    #[inline]
    fn set_uid(&self) -> bool {
        self.mode() & S_ISUID > 0
    }

    #[inline]
    fn set_gid(&self) -> bool {
        self.mode() & S_ISGID > 0
    }
    #[inline]
    fn sticky_bit(&self) -> bool {
        self.mode() & S_ISVTX > 0
    }

    #[inline]
    fn readable_by_owner(&self) -> bool {
        self.mode() & S_IRUSR > 0
    }
    #[inline]
    fn writable_by_owner(&self) -> bool {
        self.mode() & S_IWUSR > 0
    }
    #[inline]
    fn executable_by_owner(&self) -> bool {
        self.mode() & S_IXUSR > 0
    }

    #[inline]
    fn readable_by_group(&self) -> bool {
        self.mode() & S_IRGRP > 0
    }
    #[inline]
    fn writable_by_group(&self) -> bool {
        self.mode() & S_IWGRP > 0
    }
    #[inline]
    fn executable_by_group(&self) -> bool {
        self.mode() & S_IXGRP > 0
    }

    #[inline]
    fn readable_by_other(&self) -> bool {
        self.mode() & S_IROTH > 0
    }
    #[inline]
    fn writable_by_other(&self) -> bool {
        self.mode() & S_IWOTH > 0
    }
    #[inline]
    fn executable_by_other(&self) -> bool {
        self.mode() & S_IXOTH > 0
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
        if let Ok(output) =
            Command::new("bash").args(["-c", "\"which ls\""]).output()
        {
            if output.stderr.is_empty() {
                let ls_path =
                    <OsStr as OsStrExt>::from_bytes(&output.stdout[..]);

                let metadata =
                    metadata(ls_path).expect("can not get ls metadata");
                let permission = metadata.permissions();

                assert!(permission.readable_by_owner());
                assert!(permission.readable_by_group());
                assert!(permission.readable_by_other());

                assert!(permission.executable_by_owner());
                assert!(permission.executable_by_group());
                assert!(permission.executable_by_other());
            }
        }
    }

    #[test]
    // test `set_uid()`
    fn test_su() {
        if let Ok(output) =
            Command::new("bash").args(["-c", "\"which su\""]).output()
        {
            if output.stderr.is_empty() {
                let su_path =
                    <OsStr as OsStrExt>::from_bytes(&output.stdout[..]);

                let metadata =
                    metadata(su_path).expect("can not get su metadata");
                let permission = metadata.permissions();

                assert!(permission.set_uid());
            }
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
}
