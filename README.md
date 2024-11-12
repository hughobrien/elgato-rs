# elgato-rs
Simple utility to control brightness & colour temperature of an elgato light.

Intended to be bound to a keyboard shortcut.

Tested with the keylight model.

## Nix Flake
```shell
$ nix run github:hughobrien/elgato-rs                                            
Error: "Usage: elgato-rs http://elgato.lan:9123/elgato/lights <bright|dim|warm|cold>"

$ nix run github:hughobrien/elgato-rs http://elgato.lan:9123/elgato/lights bright
```

## Build
```shell
$ cargo build --release
$ ./target/release/elgato-rs
Error: "Usage: elgato-rs http://elgato.lan:9123/elgato/lights <bright|dim|warm|cold>"

$ ./target/release/elgato-rs http://elgato.lan:9123/elgato/lights bright
```

## Use
My light has a static DHCP lease and a hosts file entry, sub in your own endpoint as needed.

Then in sway config file
```config
set $elgato-rs /home/hugh/.bin/elgato-rs
set $elgato-url http://elgato.lan:9123/elgato/lights
bindsym --no-repeat $mod+Shift+Prior exec $elgato-rs $elgato-url bright
bindsym --no-repeat $mod+Shift+Next exec $elgato-rs $elgato-url dim
bindsym --no-repeat $mod+Shift+Home exec $elgato-rs $elgato-url warm
bindsym --no-repeat $mod+Shift+End exec $elgato-rs $elgato-url cold
```

Prior==PgUp and Next==PgDown.

Now I hold down two modifier keys, and four of the keys in the island above the arrow keys become light controls.


## FAQ
### How big is the binary?
```
$ eza -l ./target/release/elgato-rs
.rwxr-xr-x 2.6M hugh 11 Nov 20:36 ./target/release/elgato-rs

$ strip ./target/release/elgato-rs 
$ eza -l ./target/release/elgato-rs
.rwxr-xr-x 2.2M hugh 11 Nov 21:35 ./target/release/elgato-rs
```

### Have you benchmarked this?
That would be insane
```
$ hyperfine "./elgato-rs http://elgato.lan:9123/elgato/lights bright"
Benchmark 1: ./elgato-rs http://elgato.lan:9123/elgato/lights bright
  Time (mean ± σ):     121.8 ms ±  21.3 ms    [User: 29.4 ms, System: 9.0 ms]
  Range (min … max):   100.6 ms … 171.0 ms    24 runs
```

### Couldn't this have been a script?
Yes, but it's intolerably slower.
```
$ hyperfine "./elgato.py bright"
Benchmark 1: python elgato.py up
Time (mean ± σ):     330.7 ms ±  16.3 ms    [User: 198.4 ms, System: 22.5 ms]
Range (min … max):   314.2 ms … 364.0 ms    10 runs
```

### How does this compare to the state of the art in Elgato Control?
* It has 0.1% of the code (and therefore features) as [this Python library](https://github.com/frenck/python-elgato)
* It has no UI it just yells at you unlike [this nice system utility](https://github.com/mschneider82/keylight-control)
* It has no support for advanced timeseries DB metrics like [this one](https://github.com/mdlayher/keylight_exporter)
