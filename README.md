# Rust Operating System Notes

These notes track the exact path we are taking while building a tiny bare-metal Rust kernel.

## Big Picture

A normal program runs like this:

```text
hardware -> operating system -> your program
```

Our kernel will eventually run like this:

```text
hardware -> our kernel
```

That means we cannot rely on normal operating system services like printing, files, processes, or a standard program startup.

## Step 1: Create The Learning Folder

We created a main folder:

```text
Desktop/Rust Operating System
```

Then we created the actual Rust project folder inside it:

```text
Desktop/Rust Operating System/kernel
```

The outer folder is for the whole learning project. The `kernel` folder is the Rust crate.

## Step 2: Install Rust

When `cargo` was not found, we installed Rust using `rustup`.

Important tools:

```text
rustc  = Rust compiler
cargo  = Rust build tool and project manager
rustup = Rust toolchain and target manager
```

A toolchain is the set of Rust tools. A target is the kind of machine or environment we are compiling for.

## Step 3: Create The Rust Project

Inside the `kernel` folder, we ran:

```bash
cargo init --bin
```

This created:

```text
Cargo.toml
src/main.rs
```

`Cargo.toml` contains project settings. `src/main.rs` contains the starter Rust program.

## Step 4: Build The Normal Starter Program

We ran:

```bash
cargo build
```

At this point it built a normal operating-system program for the Mac. This was only to prove the Rust tools worked.

## Step 5: Configure Cargo For Bare Metal

We created:

```text
.cargo/config.toml
```

with:

```toml
[build]
target = "x86_64-unknown-none"
```

Meaning:

```text
x86_64 = 64-bit Intel/AMD CPU
unknown = no specific vendor/platform
none   = no operating system
```

After that, Cargo builds for a bare x86_64 machine instead of macOS.

## Step 6: Install The Bare-Metal Target

We ran:

```bash
rustup target add x86_64-unknown-none
```

This taught Rust how to compile core bare-metal code for a raw x86_64 machine.

## Step 7: Observe The Expected Failure

When we ran:

```bash
cargo build
```

the Hello World program failed because it needed:

```text
std      = Rust standard library
println  = operating-system-backed printing
panic runtime support
```

That failure was useful. It proved Cargo was now trying to build without an operating system.

## Step 8: Replace The Normal Program With A Kernel Entry Point

We changed `src/main.rs` to use:

```rust
#![no_std]
#![no_main]
```

Meaning:

```text
#![no_std]  = do not use Rust's operating-system standard library
#![no_main] = do not use Rust's normal main function startup
```

We added `_start` as our first entry point:

```rust
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {
        core::hint::spin_loop();
    }
}
```

Important pieces:

```text
#[unsafe(no_mangle)] = keep the function name exactly as _start
extern "C"           = use the C calling convention
-> !                 = this function never returns
```

At this stage, if the CPU reached `_start`, it would wait forever.

## Step 9: Add A Panic Handler

We added:

```rust
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
```

A panic means the program hit a situation it cannot safely continue from.

In a normal app, Rust can print an error and exit. In our kernel, there is no operating system to print or exit to, so our first panic behavior is simply to stop in a controlled wait loop.

## Step 10: Configure Panic Abort

In `Cargo.toml`, we added:

```toml
[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
```

Unwinding means walking back through the call stack and cleaning up. Abort means stop immediately. Early kernels use abort because we do not have the runtime support for unwinding.

## Step 11: Use A Real CPU Halt Instruction

We imported inline assembly:

```rust
use core::arch::asm;
```

Then added:

```rust
fn halt() {
    unsafe {
        asm!("hlt");
    }
}
```

`hlt` is an x86 CPU instruction. It tells the CPU:

```text
sleep until the next interrupt
```

Then both loops became:

```rust
loop {
    halt();
}
```

Now the kernel waits by telling the CPU to halt instead of spinning constantly.

## Current Kernel Code

```rust
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

fn halt() {
    unsafe {
        asm!("hlt");
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {
        halt();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        halt();
    }
}
```

## Useful Commands So Far

```bash
ls
```

List files in the current folder.

```bash
ls src
```

List files inside `src` without moving into it.

```bash
nano src/main.rs
```

Edit the source file.

```bash
cat src/main.rs
```

Print the source file so we can inspect what was saved.

```bash
cargo build
```

Compile the kernel.

```bash
file target/x86_64-unknown-none/debug/kernel
```

Inspect the compiled output file.

## Next Step

Check whether QEMU is installed so we can eventually run the kernel in an emulator:

```bash
qemu-system-x86_64 --version
```

## Step 12: Install QEMU

QEMU is an emulator. It gives us a fake x86_64 computer to test the kernel inside.

This is safer than trying to boot the kernel on real hardware.

We checked for QEMU with:

```bash
qemu-system-x86_64 --version
```

At first, the command was missing. Then QEMU was installed with Homebrew:

```bash
brew install qemu
```

After installation, this command worked:

```bash
qemu-system-x86_64 --version
```

QEMU is not the bootloader. It is only the fake computer. We still need startup code that knows how to load our kernel and jump into `_start`.

## Step 13: Add The Bootloader API

We added this dependency to `Cargo.toml`:

```toml
[dependencies]
bootloader_api = "0.11.15"
```

This does not boot the kernel by itself.

It gives our kernel the Rust-side interface for working with the modern `bootloader` crate. Later the bootloader will prepare the machine, load our kernel, and pass startup information into it.

## Step 14: Use The Bootloader Entry Point

We imported:

```rust
use bootloader_api::{entry_point, BootInfo};
```

Then we replaced the manually exported `_start` function with:

```rust
entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    loop {
        halt();
    }
}
```

Meaning:

```text
entry_point!(kernel_main) = ask bootloader_api to create the correct low-level entry wrapper
kernel_main              = our Rust kernel function
BootInfo                 = startup information from the bootloader
```

The bootloader will handle the earliest CPU setup, then call `kernel_main`.

This is our first API-style contract. It is not a web API; it is an agreement between the bootloader and our kernel.

The bootloader expects our kernel function to have this shape:

```rust
fn kernel_main(_boot_info: &'static mut BootInfo) -> !
```

Meaning:

```text
bootloader calls kernel_main
bootloader passes a BootInfo pointer
kernel_main never returns
```

We use `BootInfo` because that is the data format defined by `bootloader_api`.

## Step 15: Add The Bootloader Build Dependency

We added the bootloader crate as a build dependency:

```toml
[build-dependencies]
bootloader = { version = "0.11.15", default-features = false, features = ["uefi"] }
```

Meaning:

```text
[dependencies]       = code the kernel itself uses
[build-dependencies] = code Cargo can use while building the project
```

The `bootloader` crate can create bootable disk images from our compiled kernel.

We use:

```text
default-features = false
features = ["uefi"]
```

so it only builds the modern UEFI bootloader path, not the older BIOS path.

When we built after adding it, Cargo downloaded several helper crates. That was expected.

The important distinction is:

```toml
[dependencies]
bootloader_api = "0.11.15"

[build-dependencies]
bootloader = { version = "0.11.15", default-features = false, features = ["uefi"] }
```

`bootloader_api` is used by our kernel code. `bootloader` is used by the build system.

## Step 16: Pin This Project To Nightly Rust

When we tried to build, the bootloader failed with:

```text
the `-Z` flag is only accepted on the nightly channel of Cargo
```

Meaning:

```text
stable Rust  = the normal Rust toolchain
nightly Rust = a newer toolchain that allows experimental compiler features
```

The bootloader build uses a nightly-only Cargo feature, so this kernel project needs nightly Rust.

Nightly was already installed on this machine, so we added:

```text
kernel/rust-toolchain.toml
```

with:

```toml
[toolchain]
channel = "nightly"
components = ["rust-src"]
```

This pins only this kernel project to nightly. Other Rust projects can still use stable Rust.

After that, the first build still tried to refresh package data from the internet. Since this session did not have network access, we built using the already-cached packages:

```bash
CARGO_NET_OFFLINE=true cargo build
```

That build succeeded.

At this point, our kernel compiles with the bootloader API.

## Current Kernel Code

```rust
#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
use bootloader_api::{entry_point, BootInfo};

fn halt() {
    unsafe {
        asm!("hlt");
    }
}

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    loop {
        halt();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        halt();
    }
}
```

What the CPU is doing right now:

```text
bootloader starts the machine
bootloader calls kernel_main
kernel_main repeatedly runs hlt
CPU sleeps until an interrupt happens
```

The next useful step is to write text directly to the screen, without `println!`.
