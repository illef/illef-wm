#! /usr/bin/env bash
# ----------------------------------------
# Bootstrap the start of a penrose session
# >> This get's run on restart as well!
# ----------------------------------------

# Make sure we only run once
pid=$$
pgrep -fi illef-wm-startup.sh | grep -v "^$pid$" | xargs -I{} kill {}

pkill -fi polybar; "$HOME/.config/polybar/launch.sh"
"$HOME/.local/bin/update_background.sh"
pkill -fi nm-applet; nm-applet &
