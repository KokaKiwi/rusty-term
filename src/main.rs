#[macro_use] extern crate clap;
extern crate config;
extern crate x11;
extern crate xdg;

use ctx::Context;

mod ctx;

fn main() {
    let matches = clap_app!(rt =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: "A simple terminal implementation in Rust for X")

        (@setting ColoredHelp)
        (@setting UnifiedHelpMessage)
    ).get_matches();

    let ctx = Context::open(matches);
}
