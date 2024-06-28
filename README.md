# dshw
A dead simple CLI program to query information about system and hardware. Basically a CLI wrapper
over the [sysinfo](https://crates.io/crates/sysinfo) crate.

Initially it was written to configure a Wayland bar (yes, reinventing the wheel), but playing with
the sysinfo crate has gone too far...

Written just for fun.

## Features
- Query about:
    - CPU
    - Sensors
    - Memory
    - Swap memory
    - Drives
    - OS
    - Network
- Multiple queries can be issued (`dshw memory total usage available free`)
- Custom delimiter is supported (`dshw -d ', ' drive /dev/sda3 total available usage fs mount-point`)
- Command-specific string formatting (`dshw -f 'I use %release-id% btw, my total cpu usage is %total-cpu-usage% %%' os`
may yield `I use arch btw, my total cpu usage is 2.15 %`)
- Specify how many times to run and set interval between commands (`dshw -I 1s -n 5 os total-cpu-usage`).
- Specify the unit of information for memory, swap, drive, and network commands (`dshw -u gib memory total`)

## Usage
To print total and available memory:
```
~ $ dshw memory total available
16689270784
10336894976
```

Some commands like `cpu`, `sensor` or `drive` take a name/id as the first **required** argument:
```
~ $ dshw drive /dev/sda3 fs usage total
ext4
259652198400
474853687296
```

Some commands take zero arguments:
```
~ $ dshw list-sensors
acpitz temp1
acpitz temp2
coretemp Core 1
coretemp Core 5
coretemp Package id 0
coretemp Core 2
coretemp Core 4
coretemp Core 7
coretemp Core 0
coretemp Core 6
coretemp Core 3
amdgpu edge
```

You can also specify a desired delimiter:
```
~ $ dshw -d ', ' list-cpus
cpu0, cpu1, cpu2, cpu3, cpu4, cpu5, cpu6, cpu7, cpu8, cpu9, cpu10, cpu11, cpu12, cpu13, cpu14, cpu15
```

You can also format a string with desired parameters:
```
~ $ dshw -f '%usage%/%total% bytes' memory
8163627008/16689266688 bytes
```

Formatting is supported even by commands, that require a name/id argument:
```
~ % dshw -f '%frequency%, %vendor-id%' cpu cpu7
2719, GenuineIntel
```

Format specifiers can be entered in any case (`%FReQUEnCy%` = `%frequency%`).

Type `dshw help` to see all commands. Type, for example, `dshw help os` to see all OS related subcommands.

## Installation
You'll need the Rust toolchain ([rustup](https://rustup.rs/) or from system package repo) and make
sure it's up to date.

When the toolchain will be prepared, type `cargo install dshw`.

If you have installed successfully dshw, you can now run the it simply by typing `dshw`. If
the shell says that the command does not exists, make sure that `$HOME/.cargo/bin` (or whatever the
default cargo dir will be) is in the PATH environment variable.

To see all available options, pass `-h`, `--help` or `help`.

## Contribution
If you have found a problem or have a suggestion, feel free to open an issue or send a pull request.
I'd appreciate it.

## License
dshw is licensed under the [MIT license](LICENSE.md).
