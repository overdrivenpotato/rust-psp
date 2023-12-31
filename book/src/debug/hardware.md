# Debugging Rust-PSP on real PSP Hardware
This section will teach you how to debug Rust-PSP programs on a real PSP over USB.

## Dependencies
You will need several dependencies from the [C pspsdk](https://github.com/pspdev/pspdev/)

Specifically you will need:
- usbhostfs_pc
- psplinkusb
- psp-gdb
- pspsh

Start by installing the C pspsdk and making these tools available on your `$PATH`.
There are release builds available in the github releases tab, or you can run the
build scripts, but compiling everything from scratch can be quite time consuming.

There is also a part of the setup required on a PSP running CFW. Ensure you have
a PSP running a recent custom firmware, then extract and install the psplink EBOOT
from the psplink.zip available [here](https://github.com/pspdev/psplinkusb/releases/tag/latest) to your memory card. 

## Compiling your project with debug symbols
In order to debug our program, we need the compiler to output information that the
debugger will use to identify our functions and variables by name. 
These are available in the non-release build profile, which you can build by running
`cargo psp` without `--release`, or if you want standard speed optimizations 
and debug symbols you can add the following to Cargo.toml and build with `--release`
```
[profile.release]
debug = true
```

## Setting up PSPLink
Back on your PC, run `usbhostfs_pc <path>` where `<path>` is your `target` directory
or somewhere above it in your folder hierarchy. Basically you will need the `target`
directory build outputs to be accessible from the path chosen. 

To follow along using the hello world from the previous chapter, ensure you've
built it with debug symbols, then run 

`usbhostfs_pc hello-world/target/mipsel-sony-psp/<release or debug>`

Now you're ready to plug in your PSP to your PC with a mini-USB cable and launch the
PSPLink EBOOT. It should appear under your memory stick games.

Back on the PC, launch `pspsh` in a separate terminal from `usbhostfs_pc`.
This will launch the "psp shell" which is what you will use to communicate with the PSP
over usb. You can read about all the things you can do with this in the [psplink
manual](https://github.com/pspdev/psplinkusb/blob/master/psplink_manual.pdf), but for now
all we need is the debug command to start the psplink gdb server.

## Debugging with PSP-GDB
In pspsh, run 

`debug ./psp-hello-world-example.prx`.

Now you'll need yet another terminal (hope you have lots of screen real estate 😉),
to run psp-gdb. Run it with the path to the ELF file with debug symbols, like so:

`psp-gdb --tui ./psp-hello-world-example`

Now connect psp-gdb to the GDB server with 

`target remote :10001`

Start by setting a breakpoint in our main function. 

`break psp_main`

Continue the flow of execution past all the init code to our main function.

`continue`

Step into the main function

`step`

Step into the `psp::enable_home_button` function call

`step`

Step over the next two calls to avoid going into their internals

`next`

`next`

Print the value of the `id` variable in the `enable_home_button` function

`print id`

Exit the `enable_home_button` function

`finish`

Get a stack trace of the function hierarchy

`backtrace`

Continue running the program

`continue`

These are the main commands you'll use to navigate and examine variables in GDB. 
Refer to [upstream documentation](https://ftp.gnu.org/old-gnu/Manuals/gdb/html_node/gdb_11.html#SEC12) for more details. 
Most of these have shortforms as well such as `b` for breakpoints,
and you can press enter to repeat the last command without typing it again.

# Additional Resources
There are tools that integrate with GDB such as [seer](https://github.com/epasveer/seer)
and vscode's debug tools to provide a more friendly UI, which are not covered here.
