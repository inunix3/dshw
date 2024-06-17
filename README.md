# dshw
A dead simple CLI program to query information about system and hardware. Basically a CLI wrapper
over the [sysinfo](https://crates.io/crates/sysinfo) crate.

Initially it was written to configure a Wayland bar (yes, reinventing the wheel), but playing with
the sysinfo crate has gone too far...

Written just for fun.

## Installation
You'll need the Rust toolchain ([rustup](https://rustup.rs/) or from system package repo) and make
sure it's up to date.

When the toolchain will be prepared, type `cargo install dshw`.

If you have installed successfully dshw, you can now run the it simply by typing `dshw`. If
the shell says that the command does not exists, make sure that `$HOME/.cargo/bin` (or whatever the
default cargo dir will be) is in the PATH environment variable.

To see all available options, pass `-h`, `--help` or `help`.

## TODO
- [ ] Add extra functionality like network.
- [ ] Timing: measure stats within specified interval.
- [ ] Add format option: something like `dshw drive '/dev/sda3' fmt '{usage}/{total} GiB'`
...

## Contribution
If you have found a problem or have a suggestion, feel free to open an issue or send a pull request.
I'd appreciate it.

## License
dshw is licensed under the [MIT license](LICENSE.md).
