# Ontime CLI

This is a CLI to show the time of given Timezones.
Compare different places timezones.


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

If you want to know all available timezones, you can use the list command.
```
$ ontime list
```
