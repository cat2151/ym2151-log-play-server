#!/usr/bin/env rust-script
// cargo-deps: duct
use duct::cmd;
fn main() {
    cmd!("cat-play-mml", "--shutdown").run().ok();
    for tool in ["ym2151-log-play-server", "cat-play-mml", "cat-edit-mml", "ym2151-tone-editor"] {
        println!("Installing {}...", tool);
        cmd!("cargo", "install", "--git", format!("https://github.com/cat2151/{}", tool)).run().unwrap();
    }
}
