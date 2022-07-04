# Data Cargo 2022
New rewritten version of the old Data Cargo project made on demand. 

## config
To configure the build options change content of files located in: `"options/"`.
- **key**: encryption key used to hide the string literals
- **out_channel_id**: id of the discord channel where logs will be send
- **owner**: id of the discord user that can execute DataCargo commands
- **token**: the discord bot token

## building
This crate provides 2 ways of compiling.

**To build the executable run the command**: <br>
- `cargo build --package data-cargo-22 --bin data-cargo-22 --release`

**To build the spoofed _xmllite_ library**: <br>
Make sure the desired versions of the original _xmllite_ library are located in `"xmllite/bin/"` and 
their prefixes match the specific architecture. <br> 
_(`_32` = 32 bit) (`_64` = 64 bit)_ <br>
If everything is correct, run command: <br>
- `cargo build --package xmllite --release`

_To build for 32 bit architecture add `--target i686-pc-windows-msvc` at the end of the command._

## commands
only the **_Selector_** and **_Test_** commands are implemented.

The first one is used to select the desired target on which the commands will be executed. <br>
Example: `s UUID-11111111-2222-3333-4444-555555555555`

The second one is used to test if the selected target is responsive <br>
Example: `test`

To add new commands, modify `message()` function in the `handler.rs` file and/or you can use imported **_CommandEngine_** functionality. <br> 
(see `engine.rs` in the `"(...)/utils/"` directory). 