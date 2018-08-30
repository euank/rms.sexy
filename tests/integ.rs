// SPDX-License-Identifier: AGPL-3.0-only

extern crate serde_json;
extern crate netns;
extern crate pnetlink;
extern crate reqwest;

mod shared;
use shared::*;

#[test]
#[cfg_attr(not(feature = "integ"), ignore)]
fn it_serves_all_images() {
    run(|| {
	let json_images = get_all_images();
	assert!(json_images.len() > 0);
	for img in json_images {
	    assert!(img.starts_with("/img/"));
	    let resp = reqwest::get(&format!("http://127.0.0.1:3000{}", img)).unwrap();
	    assert!(resp.status().is_success());
	    let contenttype = resp.headers().get_raw("Content-Type");
	    let type_value = String::from_utf8_lossy(contenttype.unwrap().one().unwrap());
	    assert!(type_value.starts_with("image/"));
	}
    })
}

fn get_all_images() -> Vec<String> {
    let imgs_res = reqwest::get("http://127.0.0.1:3000/?images=pls").unwrap();
    serde_json::from_reader(imgs_res).unwrap()
}

