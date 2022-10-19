# lua-latest-sys

Unsafe Rust bindings for the latest Lua version (5.4.4).

## Motivation

Lua is not a very backwards compatible friendly language[[1]](https://lua-l.lua.narkive.com/TMcd90Jc/a-rant-about-backward-incompatible-changes)[[2]](https://www.reddit.com/r/ProgrammingLanguages/comments/gugxw6/comment/fsige7x/?utm_source=share&utm_medium=web2x&context=3).

The goal of this repository is to support the latest and only the latest version of the Lua language.

## Using the library

TODO

## Creating the bindings

The bindings are generated with `bindgen` using the following command.

```sh
bindgen lua.hpp -o bindings.rs --no-layout-tests --size_t-is-usize --default-macro-constant-type signed
```
