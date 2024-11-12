## elgato-rs
Simple utility to control brightness & colour temperature of an elgato light.

Intended to be bound to a keyboard shortcut.

Tested with the keylight model.

```shell
cargo build --release
Error: "Usage: elgato-rs http://elgato.lan:9123/elgato/lights <bright|dim|warm|cold>"
```

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
