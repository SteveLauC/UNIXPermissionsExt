//! functions that can be used directly with `mode_t`

use crate::constants::*;

#[inline]
/// Is set-UID bit set?
pub fn set_uid(perm: u32) -> bool {
    perm & S_ISUID > 0
}

#[inline]
/// Is set-GID bit set?
pub fn set_gid(perm: u32) -> bool {
    perm & S_ISGID > 0
}

#[inline]
/// Is sticky bit set?
pub fn sticky_bit(perm: u32) -> bool {
    perm & S_ISVTX > 0
}

#[inline]
/// Is this file readable by owner?
pub fn readable_by_owner(perm: u32) -> bool {
    perm & S_IRUSR > 0
}
#[inline]
/// Is this file writable by owner?
pub fn writable_by_owner(perm: u32) -> bool {
    perm & S_IWUSR > 0
}
#[inline]
/// Is this file executable by owner?
pub fn executable_by_owner(perm: u32) -> bool {
    perm & S_IXUSR > 0
}

#[inline]
/// Is this file readable by group?
pub fn readable_by_group(perm: u32) -> bool {
    perm & S_IRGRP > 0
}
#[inline]
/// Is this file writable by group?
pub fn writable_by_group(perm: u32) -> bool {
    perm & S_IWGRP > 0
}
#[inline]
/// Is this file executable by group?
pub fn executable_by_group(perm: u32) -> bool {
    perm & S_IXGRP > 0
}

#[inline]
/// Is this file readable by others?
pub fn readable_by_other(perm: u32) -> bool {
    perm & S_IROTH > 0
}
#[inline]
/// Is this file writable by others?
pub fn writable_by_other(perm: u32) -> bool {
    perm & S_IWOTH > 0
}
#[inline]
/// Is this file executable by others?
pub fn executable_by_other(perm: u32) -> bool {
    perm & S_IXOTH > 0
}

/// Convert Permissions into a `String`, just like the one printed by
/// `ls(1)`.
pub fn stringify(perm: u32) -> String {
    format!(
        "{}{}{}{}{}{}{}{}{}",
        if readable_by_owner(perm) { 'r' } else { '-' },
        if writable_by_owner(perm) { 'w' } else { '-' },
        if executable_by_owner(perm) && set_uid(perm) {
            's'
        } else if executable_by_owner(perm) {
            'x'
        } else if set_uid(perm) {
            'S'
        } else {
            '-'
        },
        if readable_by_group(perm) { 'r' } else { '-' },
        if writable_by_group(perm) { 'w' } else { '-' },
        if executable_by_group(perm) && set_gid(perm) {
            's'
        } else if executable_by_group(perm) {
            'x'
        } else if set_gid(perm) {
            'S'
        } else {
            '-'
        },
        if readable_by_other(perm) { 'r' } else { '-' },
        if writable_by_other(perm) { 'w' } else { '-' },
        if executable_by_other(perm) && sticky_bit(perm) {
            't'
        } else if executable_by_other(perm) {
            'x'
        } else if sticky_bit(perm) {
            'T'
        } else {
            '-'
        },
    )
}
