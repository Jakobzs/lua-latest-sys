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
