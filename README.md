# Ontime CLI

This is a CLI to show the time of given Timezones.
Compare different places timezones.

## Installation

First install `rustup` using your system package manager, and then run:

```sh
rustup toolchain install nightly-2024-09-01
cargo install ontime-cli
```

**Note** that, in order to use `cargo` and `ontime` on Posix systems, you need
to put `$CARGO_HOME/bin` (usually `$HOME/.cargo/bin`) into `PATH`.

## Usage

### Compare subcommand
Just say the timezones you want to compare and you're ready to go.
```
$ ontime-cli compare --timezone America/New_York --timezone Europe/London --timezone Europe/Berlin
```
And it'll bring:
```
Timezone                  | Time
_________________________ | ___________________________
America/New_York          | 2024-10-27 21:10 EDT -04:00
Europe/London             | 2024-10-28 01:10 GMT +00:00
Europe/Berlin             | 2024-10-28 02:10 CET +01:00
```
It's possible to pass datetime argument to compare timezones:

```
$ ontime-cli compare --timezone America/New_York --timezone Europe/London --timezone Europe/Berlin --datetime '2025-04-14 17:34'
```
And it'll bring:
```
Timezone                  | Time
_________________________ | ___________________________
America/New_York          | 2025-04-14 16:34 EDT -04:00
Europe/London             | 2025-04-14 21:34 BST +01:00
Europe/Berlin             | 2025-04-14 22:34 CEST +02:00
```
The `--datetime` option works with many date formats follow all of them:
```
// unix timestamp
"1511648546",
"1620021848429",
"1620024872717915000",
// rfc3339
"2021-05-01T01:17:02.604456Z",
"2017-11-25T22:34:50Z",
// rfc2822
"Wed, 02 Jun 2021 06:31:39 GMT",
// postgres timestamp yyyy-mm-dd hh:mm:ss z
"2019-11-29 08:08-08",
"2019-11-29 08:08:05-08",
"2021-05-02 23:31:36.0741-07",
"2021-05-02 23:31:39.12689-07",
"2019-11-29 08:15:47.624504-08",
"2017-07-19 03:21:51+00:00",
// yyyy-mm-dd hh:mm:ss
"2014-04-26 05:24:37 PM",
"2021-04-30 21:14",
"2021-04-30 21:14:10",
"2021-04-30 21:14:10.052282",
"2014-04-26 17:24:37.123",
"2014-04-26 17:24:37.3186369",
"2012-08-03 18:31:59.257000000",
// yyyy-mm-dd hh:mm:ss z
"2017-11-25 13:31:15 PST",
"2017-11-25 13:31 PST",
"2014-12-16 06:20:00 UTC",
"2014-12-16 06:20:00 GMT",
"2014-04-26 13:13:43 +0800",
"2014-04-26 13:13:44 +09:00",
"2012-08-03 18:31:59.257000000 +0000",
"2015-09-30 18:48:56.35272715 UTC",
// yyyy-mm-dd
"2021-02-21",
// yyyy-mm-dd z
"2021-02-21 PST",
"2021-02-21 UTC",
"2020-07-20+08:00",
// hh:mm:ss
"01:06:06",
"4:00pm",
"6:00 AM",
// hh:mm:ss z
"01:06:06 PST",
"4:00pm PST",
"6:00 AM PST",
"6:00pm UTC",
// Mon dd hh:mm:ss
"May 6 at 9:24 PM",
"May 27 02:45:27",
// Mon dd, yyyy, hh:mm:ss
"May 8, 2009 5:57:51 PM",
"September 17, 2012 10:09am",
"September 17, 2012, 10:10:09",
// Mon dd, yyyy hh:mm:ss z
"May 02, 2021 15:51:31 UTC",
"May 02, 2021 15:51 UTC",
"May 26, 2021, 12:49 AM PDT",
"September 17, 2012 at 10:09am PST",
// yyyy-mon-dd
"2021-Feb-21",
// Mon dd, yyyy
"May 25, 2021",
"oct 7, 1970",
"oct 7, 70",
"oct. 7, 1970",
"oct. 7, 70",
"October 7, 1970",
// dd Mon yyyy hh:mm:ss
"12 Feb 2006, 19:17",
"12 Feb 2006 19:17",
"14 May 2019 19:11:40.164",
// dd Mon yyyy
"7 oct 70",
"7 oct 1970",
"03 February 2013",
"1 July 2013",
// mm/dd/yyyy hh:mm:ss
"4/8/2014 22:05",
"04/08/2014 22:05",
"4/8/14 22:05",
"04/2/2014 03:00:51",
"8/8/1965 12:00:00 AM",
"8/8/1965 01:00:01 PM",
"8/8/1965 01:00 PM",
"8/8/1965 1:00 PM",
"8/8/1965 12:00 AM",
"4/02/2014 03:00:51",
"03/19/2012 10:11:59",
"03/19/2012 10:11:59.3186369",
// mm/dd/yyyy
"3/31/2014",
"03/31/2014",
"08/21/71",
"8/1/71",
// yyyy/mm/dd hh:mm:ss
"2014/4/8 22:05",
"2014/04/08 22:05",
"2014/04/2 03:00:51",
"2014/4/02 03:00:51",
"2012/03/19 10:11:59",
"2012/03/19 10:11:59.3186369",
// yyyy/mm/dd
"2014/3/31",
"2014/03/31",
// mm.dd.yyyy
"3.31.2014",
"03.31.2014",
"08.21.71",
// yyyy.mm.dd
"2014.03.30",
"2014.03",
// yymmdd hh:mm:ss mysql log
"171113 14:14:20",
// chinese yyyy mm dd hh mm ss
"2014年04月08日11时25分18秒",
// chinese yyyy mm dd
"2014年04月08日",
```

You can set the `local` flag to show your local timezone.

```
$ ontime-cli compare --timezone America/New_York --timezone Europe/London --timezone Europe/Berlin --local
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
$ ontime-cli compare --timezone America/New_York --timezone  Europe/London --timezone Europe/Berlin --from-now 3
```
The next example, 3 hours ago.

```
$ ontime-cli compare --timezone America/New_York --timezone Europe/London --timezone Europe/Berlin --ago 3
```
Compare subcommand also accepts STDIN values. You can pipe values for comparing like this:
```
$ echo "Europe/Berlin" ontime-cli compare
```
The response will be:
```
Timezone                  | Time
_________________________ | ___________________________
Europe/Berlin             | 2025-03-30 17:37 CEST +02:00
```

### List subcommand
If you want to know all available timezones, you can use the list command.
```
$ ontime-cli list
```
You can filter the available timezones if your need:
```
$ ontime-cli list --filter Brazil
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
