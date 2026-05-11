# pioctl

A CLI tool for applying and restoring output devices profiles (monitors and audio sinks).

### Supported monitors provider:
- Hyprland

### Supported audio outputs provider:
- PipeWire

## Usage
1. Profiles are `.json` files saved in `~/.congif/pioctl/` directory, the filename is a unique profile id you can use to apply it.

2. A current profile is saved in `~/.local/share/pioctl/current_profile` file.

3. Profile example:
```
{
  "name": "PC",
  "monitors_config": {
    "delay_before_ms": 0,
    "disabled_to_enabled_delay_ms": null,
    "monitors": [
      {
        "name": "HDMI-A-1",
        "scale": 2.0,
        "transformation": 0,
        "resolution": {
          "width": 4096,
          "height": 2160
        },
        "refresh_rate": 29.97,
        "is_enabled": false,
        "mirror_of_name": null,
        "current_position": {
          "width": 0,
          "height": 0
        }
      },
      {
        "name": "DP-2",
        "scale": 1.0,
        "transformation": 0,
        "resolution": {
          "width": 1920,
          "height": 1080
        },
        "refresh_rate": 60.0,
        "is_enabled": true,
        "mirror_of_name": null,
        "current_position": {
          "width": 0,
          "height": 0
        }
      },
      {
        "name": "DP-1",
        "scale": 1.0,
        "transformation": 0,
        "resolution": {
          "width": 2560,
          "height": 1440
        },
        "refresh_rate": 59.951,
        "is_enabled": true,
        "mirror_of_name": null,
        "current_position": {
          "width": 1920,
          "height": 0
        }
      }
    ]
  },
  "audio_sinks_config": {
    "delay_before_ms": 0,
    "audio_sinks": [
      {
        "sink_name": "alsa_output.usb-Focusrite_Scarlett_2i2_USB-00",
        "volume": 75,
        "default": true
      }
    ]
  },
  "desktop_config": {
    "delay_before_ms": 500,
    "workspaces": [
      {
        "index": "1",
        "monitor_name": "DP-2",
        "active": true
      },
      {
        "index": "2",
        "monitor_name": "DP-1",
        "active": true,
        "focused": true
      },
      {
        "index": "3",
        "monitor_name": "DP-1"
      },
      {
        "index": "4",
        "monitor_name": "DP-1"
      },
      {
        "index": "5",
        "monitor_name": "DP-1"
      },
      {
        "index": "6",
        "monitor_name": "DP-1"
      },
      {
        "index": "7",
        "monitor_name": "DP-1"
      }
    ]
  }
}
```

4. Commands:
- **monitors** - List all connected monitors and their current configuration
- **audio-sinks** - List all available audio output sinks
- **profiles** - List all profiles defined in the config directory
- **current** - Show the currently active profile
- **restore** - Re-apply the current profile, with an optional delay before starting
- **apply** - Apply a specific profile by its ID (config filename stem)
- **apply-next** - Apply the next profile in alphabetical order, cycling back to the first
- **help** - Print this message or the help of the given subcommand(s)
