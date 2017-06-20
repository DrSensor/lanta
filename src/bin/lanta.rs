#[macro_use]
extern crate log;
extern crate x11;


extern crate lanta;

use std::process::Command;

use lanta::cmd;
use lanta::RustWindowManager;
use lanta::groups::GroupBuilder;
use lanta::keys::ModKey;
use lanta::layout::{StackLayout, TiledLayout};
use x11::keysym;


fn main() {
    lanta::intiailize_logger();

    let modkey = ModKey::Mod4;
    let shift = ModKey::Shift;
    let mut keys = vec![
        (
            vec![modkey],
            keysym::XK_w,
            cmd::lazy::close_focused_window()
        ),
        (vec![modkey], keysym::XK_j, cmd::lazy::focus_next()),
        (vec![modkey], keysym::XK_k, cmd::lazy::focus_previous()),
        (vec![modkey, shift], keysym::XK_j, cmd::lazy::shuffle_next()),
        (
            vec![modkey, shift],
            keysym::XK_k,
            cmd::lazy::shuffle_previous()
        ),
        (vec![modkey], keysym::XK_Tab, cmd::lazy::layout_next()),
        (
            vec![modkey],
            keysym::XK_q,
            cmd::lazy::spawn(Command::new("change-wallpaper"))
        ),
        (
            vec![modkey],
            keysym::XK_Return,
            cmd::lazy::spawn(Command::new("urxvt"))
        ),
        (
            vec![modkey],
            keysym::XK_c,
            cmd::lazy::spawn(Command::new("chrome"))
        ),
        (
            vec![modkey],
            keysym::XK_v,
            cmd::lazy::spawn(Command::new("code"))
        ),
    ];

    let padding = 20;
    let layouts = vec![
        StackLayout::new("stack-padded", padding),
        StackLayout::new("stack", 0),
        TiledLayout::new("tiled", padding),
    ];

    let group_metadata = vec![
        (keysym::XK_a, "chrome", "stack"),
        (keysym::XK_s, "code", "stack"),
        (keysym::XK_d, "term", "tiled"),
        (keysym::XK_f, "misc", "tiled"),
    ];
    let groups: Vec<GroupBuilder> = group_metadata
        .into_iter()
        .map(|(key, name, default_layout_name)| {
            keys.push((vec![modkey], key, cmd::lazy::switch_group(name)));
            keys.push((
                vec![modkey, ModKey::Shift],
                key,
                cmd::lazy::move_window_to_group(name),
            ));

            GroupBuilder::new(name.to_owned(), default_layout_name.to_owned())
        })
        .collect();

    let mut wm = RustWindowManager::new(keys, groups, layouts).unwrap();
    info!("Started WM, entering event loop.");
    wm.run_event_loop();
}