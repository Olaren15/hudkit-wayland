# OverWay

Create overlays for wayland using web technologies.

## Usage

1. Generate a configuration file and edit it to your liking

```bash
cat >> config.json << END
{
    "title": "FFXIV parsing overlay",
    "url": "http://proxy.iinact.com/overlay/skyline/?OVERLAY_WS=ws://127.0.0.1:10501/ws",
    "x": 0,
    "y": 0,
    "width": 1920,
    "height": 1080,
    "monitor": 0,
    "zoom": 1.0
}
END
```

2. Run the overlay

```bash
cargo run -- -c config.json
```

## Credits

This was built on the work of theses awesome people:

anko: [anko/hudkit](https://github.com/anko/hudkit)
valarnin: [valarnin/hudkit](https://github.com/valarnin/hudkit)
SparxySys: [SparxySys/hudkit-wayland](https://github.com/SparxySys/hudkit-wayland)

## License

GNU GPLv3 Only
