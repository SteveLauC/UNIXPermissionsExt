//! constants used by this crate

pub(crate) const S_ISUID: u32 = 0o4000;
pub(crate) const S_ISGID: u32 = 0o2000;
pub(crate) const S_ISVTX: u32 = 0o1000;

pub(crate) const S_IRUSR: u32 = 0o400;
pub(crate) const S_IWUSR: u32 = 0o200;
pub(crate) const S_IXUSR: u32 = 0o100;

pub(crate) const S_IRGRP: u32 = 0o40;
pub(crate) const S_IWGRP: u32 = 0o20;
pub(crate) const S_IXGRP: u32 = 0o10;

pub(crate) const S_IROTH: u32 = 0o4;
pub(crate) const S_IWOTH: u32 = 0o2;
pub(crate) const S_IXOTH: u32 = 0o1;
