## Gamepad2Key

-- WIP --


This is an insanely simple CLI program which seeks to read gamepad inputs using SDL2 and injects them into a given X11 session display.

## (Intended) Usage

```bash
gamepad2key padconfig.json --display 1
```

And it will then listen to events from ANY SDL2 compatible gamepad and inject X11 events according the config.

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