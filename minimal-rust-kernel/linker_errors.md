# Linker errors

When trying to build with all done (panic, unwindling disabled, no_main), we get this:

```sh
➜  minimal-rust-kernel git:(main) ✗ cargo build
   Compiling minimal-rust-kernel v0.1.0 (/home/adityag/os_projects/rust_os/minimal-rust-kernel)
error: linking with `cc` failed: exit status: 1
  |
  = note: "cc" "-m64" "/home/adityag/os_projects/rust_os/minimal-rust-kernel/target/debug/deps/minimal_rust_kernel-3dccdf89839de9ee.4mad7su22b0htxh.rcgu.o" "-Wl,--as-needed" "-L" "/home/adityag/os_projects/rust_os/minimal-rust-kernel/target/debug/deps" "-L" "/home/adityag/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/home/adityag/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-8063eea38dcc5e62.rlib" "/home/adityag/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-5b228734afae15ee.rlib" "/home/adityag/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-d6c7a399d95d173f.rlib" "-Wl,-Bdynamic" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/home/adityag/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/home/adityag/os_projects/rust_os/minimal-rust-kernel/target/debug/deps/minimal_rust_kernel-3dccdf89839de9ee" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-nodefaultlibs"
  = note: ld: error: duplicate symbol: _start
          >>> defined at init.c
          >>>            /usr/lib/gcc/x86_64-pc-linux-gnu/11.1.0/../../../../lib/Scrt1.o:(_start)
          >>> defined at main.rs:29 (src/main.rs:29)
          >>>            /home/adityag/os_projects/rust_os/minimal-rust-kernel/target/debug/deps/minimal_rust_kernel-3dccdf89839de9ee.4mad7su22b0htxh.rcgu.o:(.text._start+0x0)
          collect2: error: ld returned 1 exit status
          

error: aborting due to previous error

error: could not compile `minimal-rust-kernel`
```

To fix this, there are two ways:
1. Build for a bare metal target
  * rustup target add thumbv7em-none-eabihf
  * cargo build --target thumbv7em-none-eabihf
2. By passing OS specific linker arguments (basically telling use our startpoint, don't add yours)
  * Can add these in Cargo.toml
  > [target.'cfg(target_os = "linux")']
  > rustflags = ["-C", "link-arg=-nostartfiles"]
  >
  > [target.'cfg(target_os = "windows")']
  > rustflags = ["-C", "link-args=/ENTRY:_start /SUBSYSTEM:console"]
  >
  > [target.'cfg(target_os = "macos")']
  > rustflags = ["-C", "link-args=-e __start -static -nostartfiles"]

