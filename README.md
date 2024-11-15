# elgato-rs
Simple utility to control brightness & colour temperature of an elgato light.

Intended to be bound to a keyboard shortcut.

Tested with the keylight model.

## Nix Flake
```shell
$ nix run github:hughobrien/elgato-rs                                            
Error: "Usage: elgato-rs http://keylight.lan <bright+|bright-|temp+|temp-|on|off|max|min>"

$ nix run github:hughobrien/elgato-rs http://keylight.lan max
```

## Build
```shell
$ cargo build --release
$ ./target/release/elgato-rs
Error: "Usage: elgato-rs http://keylight.lan: <bright+|bright-|temp+|temp-|on|off|max|min>"

$ ./target/release/elgato-rs http://keylight.lan max
```

## Use
My light has a static DHCP lease and a hosts file entry, swap in your own endpoint as needed.

Then in sway config file
```config
set $elgato-rs /home/hugh/.bin/elgato-rs http://keylight.lan
bindsym --no-repeat $mod+Shift+Prior exec $elgato-rs bright+
bindsym --no-repeat $mod+Shift+Next exec $elgato-rs bright-
bindsym --no-repeat $mod+Shift+Home exec $elgato-rs temp+
bindsym --no-repeat $mod+Shift+End exec $elgato-rs temp-
bindsym --no-repeat $mod+Shift+Alt+Prior exec $elgato-rs on
bindsym --no-repeat $mod+Shift+Alt+Next exec $elgato-rs off
```

Prior==PgUp and Next==PgDown.

Now I hold down two modifier keys, and four of the keys in the island above the arrow keys become light controls.

## Behaviour
|    |     Light On        |     Light Off      |
|----|---------------------|--------------------|
| B+ | Increase Brightness | Restore Brightness |
| B- | Decrease Brightness | Min Brightness     |
| T+ | Increase Warmth     | Max Brightness     |
| T- | Decrease Warmth     | Mid Brightness     |

## FAQ
### Does this support dark mode?
Stop it.

### Does this support multiple lights?
Not presently, you can probably just loop the command over the different endpoints in shell for now.

### How big is the binary?
```
$ eza -l ./target/release/elgato-rs
.rwxr-xr-x 2.6M hugh 11 Nov 20:36 ./target/release/elgato-rs

$ strip ./target/release/elgato-rs 
$ eza -l ./target/release/elgato-rs
.rwxr-xr-x 2.2M hugh 11 Nov 21:35 ./target/release/elgato-rs
```

### Have you benchmarked this?
That would be insane.
```
$ hyperfine "./elgato-rs http://keylight.lan bright+"
Benchmark 1: ./elgato-rs http://keylight.lan bright+
  Time (mean ± σ):     121.8 ms ±  21.3 ms    [User: 29.4 ms, System: 9.0 ms]
  Range (min … max):   100.6 ms … 171.0 ms    24 runs
```

### Couldn't this have been a script?
Yes, but it's intolerably slower.
```
$ hyperfine "./elgato.py bright+"
Benchmark 1: python elgato.py bright+
Time (mean ± σ):     330.7 ms ±  16.3 ms    [User: 198.4 ms, System: 22.5 ms]
Range (min … max):   314.2 ms … 364.0 ms    10 runs
```

### Has this been tested?
Not in any way that someone asking that question would find acceptable.

### How many software developers does it take to change a lightbulb?
None, it's a hardware problem.

### How does this compare to the state of the art in Elgato Control?
* It has 6% of the code (and therefore features) as [this Python library](https://github.com/frenck/python-elgato).
* It has no UI it just yells at you unlike [this nice system utility](https://github.com/mschneider82/keylight-control).
* It has no support for advanced timeseries DB metrics like [this one](https://github.com/mdlayher/keylight_exporter).

### Could this have been a shell function?
Using [HTTPie](https://httpie.io/docs/cli).
```bash
keylight() {
        # Usage: keylight x y, both 0-100, x=brightness
        KEYLIGHT=(http PUT "http://keylight.lan:9123/elgato/lights")
        if [[ "$1" = "0" ]]; then
                "${KEYLIGHT[@]}" "lights[0][on]:=0"
                return
        fi
        # 1:bright: 1-100
        # 2:temp: 143-344
        set -x
        "${KEYLIGHT[@]}" \
                "lights[0][on]:=1" \
                "lights[0][brightness]:=$1" \
                "lights[0][temperature]:=$((144 + ($2 * 2)))"
}
```

### Could this have been a shell oneliner?
Ick.
```bash
curl http://keylight.lan:9123/elgato/lights \
    | jq -cr '.lights[] |= (.brightness = (.brightness - 3 | if . < 0 then 0 else . end) | .on = if .brightness == 0 then 0 else .on end)' \
    | curl -X PUT -H "Content-Type: application/json" http://keylight.lan:9123/elgato/lights --data @-
```
