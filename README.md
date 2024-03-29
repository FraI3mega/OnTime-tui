# ontime-tui

A simple tui/cli for the OnTime transit timetable system

## Basic commands

```bash
ontime-tui # gets the data from the url configured in the config file
ontime-tui -s 012 # gets the data for the stop number 012
ontime --main-url "https://dip.mzkopole.pl/" # runs the program on the given mainpage url
```

## Config file

This is the default config

```toml
# ~/.config/ontime-tui/config.toml
main_url = "https://dip.mzkopole.pl/"
```
