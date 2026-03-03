## Gamepad2Key

This is an insanely simple CLI program which seeks to read gamepad inputs using SDL2 and injects them into a given X11 session display.

## Usage

```bash
gamepad2key padconfig.cfg --display :1
```

Where `padconfig.cfg` looks like

```ini
[gamepad2key]

# Supports comments
a = z
b = x

LB = Left
RB = Right

RS = BackSpace

dpadup = Up
dpadleft = Left
dpadright = Right
dpaddown = Down

paddle1 = 1
paddle2 = 2
paddle3 = 3
paddle4 = 4
```

And it will then listen to events from ANY SDL2 compatible gamepad and inject X11 events according the config.
See xdotool for the given keycodes references.

## Goals and Limitations

- X11 Only (No wayland)
- Insanely Simple and lightweight.
- No mouse remapping. (Although could be very easily added)
- No gamepad filtering / hiding.
- No chording.

## Building

Just ensure to have dependencies:

- `xdotool` which includes `libxdo`
- SDL2

And build using `cargo build --release`.

## Why?

This project is a small odd-requirement I needed as part of a bigger project. Basically I've been enjoying playing with https://github.com/wunnr/partydeck lately - but I wanted to have some sort of steam-input remapping with the instances so I could make use of extra buttons playing some slightly advanced games.

It might have odd-requirements but I intend it to be deadsimple.

Other more robust solutions usually include creating a virtual input - like https://github.com/AntiMicroX/antimicrox does - but I wanted to specifically avoid this approach and inject the events directly.