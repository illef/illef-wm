//! penrose from scratch
use illef_wm::{
    bindings::raw_key_bindings,
    extension::hooks::{FocusTag, InsertPositionBelowNewer},
    layouts::layouts,
    STARTUP_SCRIPT,
};
use penrose::{
    core::{bindings::parse_keybindings_with_xmodmap, hooks::ManageHook, Config, WindowManager},
    extensions::hooks::{
        add_ewmh_hooks,
        manage::{FloatingCentered, SetWorkspace},
        startup::SpawnOnStartup,
    },
    x::query::ClassName,
    x11rb::RustConn,
};
use std::collections::HashMap;
use tracing_subscriber::{self, prelude::*};

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .finish()
        .init();

    let wm = WindowManager::new(
        add_ewmh_hooks(config()),
        parse_keybindings_with_xmodmap(raw_key_bindings())?,
        HashMap::new(),
        RustConn::new()?,
    )?;

    wm.run()?;

    Ok(())
}

fn config() -> Config<RustConn> {
    let startup_hook = SpawnOnStartup::boxed(STARTUP_SCRIPT);
    let manage_hook = (ClassName("Slack"), SetWorkspace("1"))
        .then((ClassName("Slack"), FocusTag("1")))
        .then((ClassName("Youtube"), SetWorkspace("4")))
        .then((ClassName("Youtube"), FocusTag("4")))
        .then((ClassName("Sabaki"), SetWorkspace("6")))
        .then((ClassName("Sabaki"), FocusTag("6")))
        .then((
            ClassName("Gnome-screenshot"),
            FloatingCentered::new(0.5, 0.5),
        ))
        .then(InsertPositionBelowNewer)
        .boxed();

    Config {
        default_layouts: layouts(),
        startup_hook: Some(startup_hook),
        manage_hook: Some(manage_hook),
        border_width: 0,
        ..Config::default()
    }
}
