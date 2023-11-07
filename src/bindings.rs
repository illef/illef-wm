use crate::extension::actions::spawn_with_args;
use penrose::{
    builtin::{
        actions::{exit, modify_with, send_layout_message, spawn},
        layout::messages::{ExpandMain, IncMain, ShrinkMain},
    },
    core::bindings::KeyEventHandler,
    map,
    x11rb::RustConn,
};
use std::collections::HashMap;

pub fn raw_key_bindings() -> HashMap<String, Box<dyn KeyEventHandler<RustConn>>> {
    let mut raw_bindings = map! {
        map_keys: |k: &str| k.to_string();

        // Client movement
        "M-j" => modify_with(|cs| cs.focus_down()),
        "M-k" => modify_with(|cs| cs.focus_up()),
        "M-S-Return" => modify_with(|cs| cs.swap_focus_and_head()),

        "M-q" => modify_with(|cs| cs.kill_focused()),

        // Layout control
        "M-space" => modify_with(|cs| cs.next_layout()),
        // TODO: layout default
        // TODO: MirrorShrink
        // TODO: MirrorExpand
        "M-comma" => send_layout_message(|| IncMain(1)),
        "M-period" => send_layout_message(|| IncMain(-1)),
        "M-l" => send_layout_message(|| ExpandMain),
        "M-h" => send_layout_message(|| ShrinkMain),

        // Launchers
        "M-Return" => spawn("urxvt"),
        "M-c" => spawn_with_args("fish", &["-c", "~/dotfiles/utilities/.local/bin/copy_to_clipboard.sh"]),
        "M-d" => spawn_with_args("fish", &["-c","app_launcher"]),
        "M-S-x" => spawn_with_args("fish", &["-c", "~/.local/bin/lock.sh"]),
        "M-a" => spawn_with_args("fish", &["-c", "open_bookmark.sh"]),
        "M-w" => spawn("brave"),
        "M-g" => spawn("gnome-screenshot -i"),
        "M-m" => spawn_with_args("urxvt", &["-e", "~/dotfiles/utilities/.local/bin/open_mail.sh"]),
        "M-S-m" => spawn_with_args("fish", &["-c", "monitor_check.sh"]),
        "M-S-e" => spawn("systemctl poweroff"),
        "M-S-r" => spawn("systemctl reboot"),
        "M-x" => spawn_with_args("fish", &["-c", "pass show bitwarden | xclip -selection clipboard && notify-send \"bitwarden password copied\""]),
        "M-z" => spawn_with_args("fish", &["-c", "~/dotfiles/utilities/.local/bin/mfa"]),

        //wm
        "M-S-w" => spawn("pkill -fi illef-wm"), // logout
        "M-S-q" => exit(),  // restart wm
    };

    // Per-workspace focusing and client throwing
    for tag in &["1", "2", "3", "4", "5", "6"] {
        raw_bindings.extend([
            (
                format!("M-{tag}"),
                modify_with(move |client_set| client_set.focus_tag(tag)),
            ),
            (
                format!("M-S-{tag}"),
                modify_with(move |client_set| client_set.move_focused_to_tag(tag)),
            ),
        ]);
    }

    raw_bindings
}

#[cfg(test)]
mod tests {
    use super::*;
    use penrose::core::bindings::parse_keybindings_with_xmodmap;

    #[test]
    fn bindings_parse_correctly_with_xmodmap() {
        let res = parse_keybindings_with_xmodmap(raw_key_bindings());

        if let Err(e) = res {
            panic!("{e}");
        }
    }
}
