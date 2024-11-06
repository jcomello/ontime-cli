# Ontime CLI

This is a CLI to show the time of given Timezones.
Compare different places timezones.

## Installation

First install `rustup` using your system package manager, and then run:

```sh
rustup toolchain install nightly-2024-09-01
cargo install ontime
```

**Note** that, in order to use `cargo` and `ontime` on Posix systems, you need
to put `$CARGO_HOME/bin` (usually `$HOME/.cargo/bin`) into `PATH`.

## Usage

Just say the timezones you want to compare and you're ready to go.
```
$ ontime compare --timezone America/New_York --timezone Europe/London --timezone Europe/Berlin
```
And it'll bring:
```
Timezone                  | Time
_________________________ | ___________________________
America/New_York          | 2024-10-27 21:10 EDT -04:00
Europe/London             | 2024-10-28 01:10 GMT +00:00
Europe/Berlin             | 2024-10-28 02:10 CET +01:00
```

You can set the `local` flag to show your local timezone.

```
$ ontime compare --timezone America/New_York --timezone Europe/London --timezone Europe/Berlin --local
```
And it'll bring:
```
Timezone                  | Time
_________________________ | ___________________________
America/Sao_Paulo (Local) | 2024-10-27 22:10 -03 -03:00
America/New_York          | 2024-10-27 21:10 EDT -04:00
Europe/London             | 2024-10-28 01:10 GMT +00:00
Europe/Berlin             | 2024-10-28 02:10 CET +01:00
```
Also, it is possible add and subtract hours for your local time.
The follow example, we're comparing Date Time 3 hours from current time.

```
$ ontime compare --timezone america/new_york --timezone europe/london --timezone europe/berlin --from-now 3
```
The next example, 3 hours ago.

```
$ ontime compare --timezone america/new_york --timezone europe/london --timezone europe/berlin --ago 3
```

If you want to know all available timezones, you can use the list command.
```
$ ontime list
```
You can filter the available timezones if your need:
```
$ ontime list --filter Brazil
```

```
AVAILABLE TIMEZONES
-------------------
Brazil/Acre
Brazil/DeNoronha
Brazil/East
Brazil/West
```

## Known bugs

Check [open issues][] for known bugs and feature requests.

## License

This software is licensed under the [GPL-3.0 License][].

- Copyright 2024 João Mello &lt;joao.mello@a-nobody.dev&gt;
- [COPYING][]

[GPL-3.0 License]: https://opensource.org/license/gpl-3-0
[COPYING]: https://github.com/jcomello/ontime-cli?tab=GPL-3.0-1-ov-file
[open issues]: https://github.com/jcomello/ontime-cli/issues
[Please Installer]: https://crates.io/crates/ontime
