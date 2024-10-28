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
_________________________ | _________________________________
America/New_York          | 2024-10-27 20:45:35.801121855 EDT
Europe/London             | 2024-10-28 00:45:35.801121855 GMT
Europe/Berlin             | 2024-10-28 01:45:35.801121855 CET
```

You can set the `local` flag to show your local timezone.

```
$ ontime compare --timezone America/New_York --timezone Europe/London --timezone Europe/Berlin --local
```
And it'll bring:
```
Timezone                  | Time
_________________________ | _________________________________
America/Sao_Paulo (Local) | 2024-10-27 21:46:16.800451191 -03
America/New_York          | 2024-10-27 20:46:16.800451191 EDT
Europe/London             | 2024-10-28 00:46:16.800451191 GMT
Europe/Berlin             | 2024-10-28 01:46:16.800451191 CET
```

If you want to know all available timezones, you can use the list command.
```
$ ontime list
```
