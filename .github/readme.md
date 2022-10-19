# lua-latest-sys

Unsafe Rust bindings for the latest Lua version (5.4.4).

## Motivation

Lua is not a very backwards compatible friendly language[[1]](https://lua-l.lua.narkive.com/TMcd90Jc/a-rant-about-backward-incompatible-changes)[[2]](https://www.reddit.com/r/ProgrammingLanguages/comments/gugxw6/comment/fsige7x/?utm_source=share&utm_medium=web2x&context=3).

The goal of this repository is to support the latest and only the latest version of the Lua language.

## Example

Add the following line to your Cargo.toml file.
```
lua-latest-sys = "0.0.1"
```

Now you can utilize `lua-latest-sys` in your crate.

An example follows which grabs and prints the global Lua `_VERSION` variable to Rust console output.

```rust
use lua_latest_sys::{luaL_newstate, luaL_openlibs, lua_getglobal, lua_tostring};
use std::{
    error::Error,
    ffi::{CStr, CString},
};

fn main() -> Result<(), Box<dyn Error>> {
    // Create the new Lua state and open related libaries
    let l = unsafe { luaL_newstate() };
    unsafe { luaL_openlibs(l) };

    // Get the global _VERSION field
    let version = CString::new("_VERSION")?;
    unsafe { lua_getglobal(l, version.as_ptr()) };

    // Convert the _VERSION field into a Rust string
    let version_string_ptr = unsafe { lua_tostring(l, -1) };
    let version_string = unsafe { CStr::from_ptr(version_string_ptr) }.to_str()?;

    // Print the version string to output
    println!("{}", version_string);

    Ok(())
}
```

## Creating the bindings

The bindings are generated with `bindgen` using the following command.

```sh
bindgen lua.hpp -o bindings.rs --no-layout-tests --size_t-is-usize --default-macro-constant-type signed
```
