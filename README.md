# SAVR - minimal x lockscreen written in rust [![Crates.io](https://img.shields.io/crates/v/savr)](https://crates.io/crates/savr)

## Motivation

`savr` is a minimal lockscreen written with rust-lang. I took the inspiration from `slock`, but I wanted the ability to customize
the `locked` message. In `slock` this comes with community patches, with `savr` it is built in. It also uses `pam` for authentication as 
opposed to manual password hash lookup.

## Usage

See `savr -h` to see up to date options.

```
savr 0.1.0
Lukasz Gmys <lgmys@pm.me>
Simple x lockscreen

USAGE:
    savr [OPTIONS]

OPTIONS:
    -h, --help                 Print help information
    -m, --message <MESSAGE>    Locked message [default: locked]
    -V, --version              Print version information
```

## TODO
[ ] Add tests

