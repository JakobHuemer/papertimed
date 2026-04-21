# Papertimed

Papaertimed is a little daemon to controll which wallpaper is shown at what time.

## Configuration

First, make sure that your current background utility does not automatically
switch wallpapers itself on a scheduled basis as this conflicts with papertimed.

The configuration file for papertimed is located at
`~/.config/papertimed/config.yaml`.

Supported file formats are: `JSON`, `TOML`, `YAML`, `INI`, `RON`, `JSON5`, `CORN` 

An example of a valid configuration is in [examples/config.yaml](./examples/config.yaml).

### Supported Adapters

- `wpaperd`: Overwrites the configuration at `~/.config/wpaperd/wallpaper.toml`
- `hyprpaper`: Required hyprpaper ipc to be turned on. [More info on
  hyprpaper](https://wiki.hypr.land/Hypr-Ecosystem/hyprpaper/)
- `custom`: Define a custom shell command that is executed for every image on
  every monitor with monitor and image injected using 
  [MiniJinja](https://github.com/mitsuhiko/minijinja) for templating.
  e.g.: `awww -o {{ monitor }} {{ image }}`

### `global`

- `adapter`: The background ulility e.g. `wpaperd`. see at [Supported
  Adapters](#supported-adapters)

### `wallpapers`

An array of wallpapers where each element has:

- `filename`: The filename of the background image
- `schedules`: An array of schedule `id`s when the immage should be shown

### `schedules`

An array of all schedules that can then be assigned to wallpapers:

- `id`: A string identifying the schedule
- `rules`: All rules of must be true for the schedule to take effect. You can
  define rules for daytime, weekday or day of the year, etc.

### `schedules.rules`

- `day_time`: Define when on the day the schedule runs. e.g. 05:00 - 15:00
    - `from`/`to`: Daytime e.g. `05:23:50`.
    - e.g. from = 05:23:50, to = 14:15:00
- `week_days`: Array of days when to run this schedule. e.g. `["monday", "tuesday", "wed"]`
- `year_days`: Array of days in a year when to run this schedule. e.g. `[1, 4, 340]`


## Todos

- [X] Basic daily/weekly/yearly wallpaper schedule for `wpaperd` utility
- [x] Support for multiple wallpaper utilities
- [X] Preset profiles for different wallpaper utilities like `wpaperd`, `hyprpaper`,
- [X] Per monitor schedules/wallpapers
  etc.
- [ ] Wallpaper schedules support custom metadata that can be forwarded to the
  wallpaper utility
- [ ] Manual and automatic timezone selection
- [x] Custom schedule definition with a shell command
- [ ] Scheduled sleep instead of polling
- [X] Restructure config to allow reusing wallpapers and have a binding allowing
- [X] Home Manager Module + overlay
  config structure
- [ ] Add support for schedule types
  - [ ] cron
  - [ ] rrules
