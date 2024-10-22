# Ontime CLI

This is a CLI to show the time of given Timezones.
Compare different places timezones.


## Usage

Just say the timezones you want to compare and you're ready to go.
```
$ ontime --timezone America/New_York --timezone Europe/London --timezone Europe/Berlin
```
And it'll bring:
```
City                 | Time
____________________ | ____________________________________
America/New_York     | 2024-10-21 21:08:30.713163133 EDT
Europe/London        | 2024-10-22 02:08:30.713163133 BST
Europe/Berlin        | 2024-10-22 03:08:30.713163133 CEST
```

You can set the `local` flag to show your local timezone.

```
$ ontime --timezone America/New_York --timezone Europe/London --timezone Europe/Berlin --local
```
And it'll bring:
```
City                 | Time
____________________ | ____________________________________
Local                | 2024-10-21 22:08:30.713163133 -03:00
America/New_York     | 2024-10-21 21:08:30.713163133 EDT
Europe/London        | 2024-10-22 02:08:30.713163133 BST
Europe/Berlin        | 2024-10-22 03:08:30.713163133 CEST
```

If you want to know all available timezones, you can use the list command.
```
$ ontime list
```
