# Papertimed

Papaertimed is a little daemon to controll which wallpaper is shown at what time.

## Configuration

First, make sure that your current background utility does not automatically
switch wallpapers itself on a scheduled basis as this conflicts with papertimed.

The configuration file for papertimed is located at
`~/.config/papertimed/config.toml`.

Supported file formats are: `JSON`, `TOML`, `YAML`, `INI`, `RON`, `JSON5`, `CORN` 

An example of a valid configuration is in [examples/config.toml](./examples/config.toml).


### `global`

- `temp`: just a temp variable - does nothing.

### `wallpapers`

An array of wallpapers where each element has:

- `filename`: The filename of the background image
- `schedules`: All schedules that exist for this image

### `wallpapers.schedules`

An array of all schedules where the wallpaper is scheduled to be displayed, each
schedule consisting of:

- `repetition`: Over which timeperiod the schedule runs. Possible values are `Day`, `Week` and `Year`
- `rules`: All rules for this schedule which consists of Time, Days, etc. where
  at the end, all rules are combined.


### `wallpapers.schedules.rules`

An array of rules per schedule to make the schedule valid.
These rule types exist:

- `Day`: Define when on the day the schedule runs. e.g. 05:00 - 15:00
    - `from`/`to`: Daytime e.g. `05:23:50`.
    - e.g. `{ from = "05:23:50", to = "14:15:00"}`
- `Week`: Array of days when to run this schedule. e.g. `["Monday", "Tuesday"]`
- `Year`: Array of days in a year when to run this schedule. e.g. `[1, 4, 340]`


## Features

- [ ] Basic daily/weekly/yearly wallpaper schedule for `wpaperd` utility
- [ ] Support for multiple wallpaper utilities
- [ ] Per monitor schedules
- [ ] Preset profiles for different wallpaper utilities like `wpaperd`, `hyprpaper`,
  etc.
- [ ] Wallpaper schedules support custom metadata that can be forwarded to the
  wallpaper utility


