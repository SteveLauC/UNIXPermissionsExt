# UNIXPermissionsExt

[![BUILD](https://github.com/stevelauc/UNIXPermissionsExt/workflows/Rust/badge.svg)](https://github.com/stevelauc/UNIXPermissionsExt/actions/workflows/build.yml)

A trivial trait bringing missing functions that are not exposed by
[`std::os::unix::fs::PermissionsExt`](https://doc.rust-lang.org/std/os/unix/fs/trait.PermissionsExt.html)
to [`std::fs::Permissions`](https://doc.rust-lang.org/std/fs/struct.Permissions.html)
on UNIX platforms.

```rust
pub trait UNIXPermissionsExt {
    fn set_uid(&self) -> bool;
    fn set_gid(&self) -> bool;
    fn sticky_bit(&self) -> bool;

    fn readable_by_owner(&self) -> bool;
    fn writable_by_owner(&self) -> bool;
    fn executable_by_owner(&self) -> bool;

    fn readable_by_group(&self) -> bool;
    fn writable_by_group(&self) -> bool;
    fn executable_by_group(&self) -> bool;

    fn readable_by_other(&self) -> bool;
    fn writable_by_other(&self) -> bool;
    fn executable_by_other(&self) -> bool;
}
```

# How to use

1. Add it to your dependency:

   ```shell
   $ cd $YOUR_PROJECT
   $ cargo add unix_permissions_ext
   ```

2. Import this trait and use it just like you are using the standard library!

   ```rust
   use std::fs;
   use unix_permissions_ext::UNIXPermissionsExt;
   
   let metadata = fs::metadata("/usr/bin/passwd").expect("can not fetch metadata");
   let permission = metadata.permissions();
   
   assert!(permission.set_uid());
   ``` 
