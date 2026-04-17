# Papertimed

Papaertimed is a little daemon to controll which wallpaper is shown at what time.

## Configuration

First, make sure that your current background utility does not automatically
switch wallpapers itself on a scheduled basis as this conflicts with papertimed.

The configuration file for papertimed is located at
`~/.config/papertimed/config.toml`.

Supported file formats are: `JSON`, `TOML`, `YAML`, `INI`, `RON`, `JSON5`, `CORN` 

An example of a valid configuration is in [examples/config.toml](./examples/config.toml).

### Supported Adapters

- `wpaperd`: Overwrites the configuration at `~/.config/wpaperd/wallpaper.toml`
- `hyprpaper`: Required hyprpaper ipc to be turned on. [More info on
  hyprpaper](https://wiki.hypr.land/Hypr-Ecosystem/hyprpaper/)

### `global`

- `adapter`: The background ulility e.g. `wpaperd`. see at [Supported
  Adapters](#supported-adapters)

### `wallpapers`

An array of wallpapers where each element has:

- `filename`: The filename of the background image
- `schedules`: All schedules that exist for this image

### `wallpapers.schedules`

An array of all schedules where the wallpaper is scheduled to be displayed, each
schedule consisting of:

- `rules`: All rules of must be true for the schedule to take effect. You can
  define rules for daytime, weekday or day of the year, etc.

### `wallpapers.schedules.rules`

- `day_time`: Define when on the day the schedule runs. e.g. 05:00 - 15:00
    - `from`/`to`: Daytime e.g. `05:23:50`.
    - e.g. `{ from = "05:23:50", to = "14:15:00"}`
- `week_days`: Array of days when to run this schedule. e.g. `["monday", "tuesday", "wed"]`
- `year_days`: Array of days in a year when to run this schedule. e.g. `[1, 4, 340]`


## Features

- [X] Basic daily/weekly/yearly wallpaper schedule for `wpaperd` utility
- [x] Support for multiple wallpaper utilities
- [ ] Per monitor schedules
- [X] Preset profiles for different wallpaper utilities like `wpaperd`, `hyprpaper`,
  etc.
- [ ] Wallpaper schedules support custom metadata that can be forwarded to the
  wallpaper utility
- [ ] Manual and automatic timezone selection
- [ ] Custom schedule definition with a shell command

