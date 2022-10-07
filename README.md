## UNIXPermissionsExt

[![BUILD](https://github.com/stevelauc/UNIXPermissionsExt/workflows/Rust/badge.svg)](https://github.com/stevelauc/UNIXPermissionsExt/actions/workflows/build.yml)
[![crates.io](https://img.shields.io/crates/v/unix_permissions_ext.svg)](https://crates.io/crates/unix_permissions_ext)
[![docs.rs](https://docs.rs/unix_permissions_ext/badge.svg)](https://docs.rs/unix_permissions_ext)

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
  
    #[cfg(feature = "to-string")]
    fn to_string() -> String;
}

impl UNIXPermissionsExt for Permissions {
    ...
}
```

## Usage

1. Add it to your dependency:

   ```shell
   $ cd $YOUR_PROJECT
   $ cargo add unix_permissions_ext
   ```

2. Import this trait and use it just like you are using the standard library!

   ```rust
   use std::fs::{metadata, Permissions};
   use unix_permissions_ext::UNIXPermissionsExt;
   
   let metadata = metadata("/usr/bin/passwd").expect("can not fetch metadata");
   let permission = metadata.permissions();
   
   assert!(permission.set_uid());
   ``` 

3. If you have feature `to-string` enabled, you can also use `to_string()` to convert
   `Permissions` into a string, just like the one printed by `ls(1)`
   
   ```toml
   # Cargo.toml
   
   [dependencies]
   unix_permissions_ext = { version = "x.x.x", features = ["to-string"] }
   ```

   ```rust
   let permission_str = <Permissions as UNIXPermissionsExt>::to_string(&permission);
   println!("{}", permission_str);
   ```

## Contributing

Contributions of all forms are welcome, feel free to file an issue or make a pull request!

#### Test before your commit

1. Pass the tests

   ```shell
   $ cargo test --features "to-string"
   ``` 
2. Format your code

   ```shell
   $ cargo fmt
   ```