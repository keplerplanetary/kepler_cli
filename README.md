# CLI for Kepler

## Development

If you get this error on Linux while trying to build the project or (in my case, rust-analyzer in VSCode gave the error)...

```
Thread 'main' panicked at ~/.cargo/registry/src/index.crates.io-6f17d22bba15001f/yeslogic-fontconfig-sys-3.2.0/build.rs:8:48:
  called `Result::unwrap()` on an `Err` value: "\npkg-config exited with status code 1\n> PKG_CONFIG_ALLOW_SYSTEM_LIBS=1 PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1 pkg-config --libs --cflags fontconfig\n\nThe system library `fontconfig` required by crate `yeslogic-fontconfig-sys` was not found.\nThe file `fontconfig.pc` needs to be installed and the PKG_CONFIG_PATH environment variable must contain its parent directory.\nThe PKG_CONFIG_PATH environment variable is not set.\n\nHINT: if you have installed the library, try setting PKG_CONFIG_PATH to the directory containing `fontconfig.pc`.\n"
```

You may need to install libfontconfig1-dev.
