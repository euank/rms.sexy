// SPDX-License-Identifier: AGPL-3.0-only

use netns::NetNS;
use pnetlink::packet::netlink::NetlinkConnection;
use pnetlink::packet::route::link::Links;
use reqwest;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Child;
use std::process::Command;
use std::thread;
use std::time::Duration;

// This test function allows for easily running tests in parallel by isolating them each to their
// own network namespace.
pub fn run<F>(f: F)
where
    F: Fn() + Send + 'static,
{
    let thread = thread::spawn(move || {
        let _netns = NetNS::new().unwrap();
        // new network namespaces don't get a working lo, fix that up first...
        set_lo_up();

        let mut child = setup();
        wait_ready();
        f();
        child.kill().unwrap();
        child.wait().unwrap();
    });

    thread.join().unwrap();
}

fn set_lo_up() {
    let mut conn = NetlinkConnection::new();
    let link = conn.get_link_by_name("lo").unwrap().unwrap();
    conn.link_set_up(link.get_index()).unwrap();
}

pub fn root_dir() -> PathBuf {
    Path::new(&env::var("CARGO_MANIFEST_DIR").expect("build with cargo")).into()
}

fn setup() -> Child {
    let exe = root_dir().join("target/release/rms-sexy");

    let child = Command::new(exe)
        .spawn()
        .expect("could not run release binary; is it built?");
    child
}

fn wait_ready() {
    for _ in 0..100 {
        let res = reqwest::get("http://127.0.0.1:3000/healthz");
        match res {
            Ok(r) => {
                if r.status() == reqwest::StatusCode::Ok {
                    return;
                }
                println!("status: {}", r.status());
            }
            Err(e) => {
                println!("err: {}", e);
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
}
