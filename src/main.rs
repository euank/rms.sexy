// SPDX-License-Identifier: AGPL-3.0-only

extern crate chrono;
extern crate iron;
extern crate mount;
extern crate router;
extern crate serde;
extern crate staticfile;
extern crate urlencoded;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate rand;
extern crate serde_json;

#[macro_use]
extern crate include_repo;

use std::path::Path;

use chrono::prelude::*;
use iron::headers;
use iron::mime;
use iron::prelude::*;
use iron::status;
use rand::Rng;
use router::Router;
use staticfile::Static;
use urlencoded::UrlEncodedQuery;

lazy_static! {
    static ref BIRTHDAY: chrono::Date<Utc> = Utc.ymd(1953, 3, 16);
    static ref IMAGES: Vec<String> = images();
    static ref IMAGE_JSON: String = serde_json::to_string(&*IMAGES).unwrap();
}

struct RMS404Handler;

impl iron::middleware::AfterMiddleware for RMS404Handler {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        // 404 image
        match err.response.status {
            Some(status::NotFound) => Ok(Response::with((
                status::NotFound,
                Path::new("./static/404.jpg"),
            ))),
            _ => Err(err),
        }
    }
}

fn main() {
    // Lazy static, make sure they don't panic
    let _ = &*BIRTHDAY;
    let _ = &*IMAGE_JSON;

    let s = Static::new(Path::new("./static/"));

    let mut router = Router::new();
    router.any("/code.tar.gz", source_code, "source code");
    router.any("/index.php", index, "backwards-compat-index");
    router.any("/", index, "index");
    router.any("/*", s, "static");
    router.get("/healthz", healthz, "healthz");

    let mut chain = iron::middleware::Chain::new(router);
    chain.link_after(RMS404Handler);

    let _server = Iron::new(chain).http("0.0.0.0:3000").unwrap();
    println!("Listening on 3000");
}

// AGPL compliance. We exclude the images to avoid increasing build and download size too much;
// people can easily enough download them from the repo or via checking the image json regardless.
include_repo_gz!(SOURCE_CODE, ".", ":!**/*.jpg", ":!**/*.JPG", ":!**/*.png");

fn source_code(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((
        headers::ContentType(mime::Mime(
            mime::TopLevel::Application,
            mime::SubLevel::Ext("gzip".to_string()),
            vec![],
        )).0,
        status::Ok,
        &SOURCE_CODE[..],
    )))
}

fn healthz(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "healthy")))
}

fn index(req: &mut Request) -> IronResult<Response> {
    let js_enabled = match req.get_ref::<UrlEncodedQuery>() {
        Ok(query) => {
            if query.contains_key("images") {
                return list_images();
            }
            query.contains_key("js")
        }
        Err(e) => {
            warn!("Could not decode query: {}", e);
            false
        }
    };
    let now = Utc::now();
    let title = if BIRTHDAY.month() == now.month() && BIRTHDAY.day() == now.day() {
        "Happy GNU/Birthday"
    } else {
        "Our GNU/Lord and GNU/Savior is 100% sexy!"
    };

    let resp = format!(
        r#"<!DOCTYPE html>
<!-- SPDX-License-Identifier: AGPL-3.0-only -->
<html>
	<head>
		<!--
This program is free software: you can redistribute it and/or  modify
it under the terms of the GNU Affero General Public License, version 3,
as published by the Free Software Foundation.
The source code, along with the full license text, is available here:
https://github.com/euank/rms.sexy

It is also available at "/code.tar.gz" on this server.
		-->
		<meta charset="utf-8">
		<title>{}</title>
		{}
		<link rel="stylesheet" href="/style.css">
		<link rel="license" href="/license.txt">
		<script async src="/script.js"></script>
	</head>
	<body>
		<a href="https://donate.fsf.org/"><img class="donate" src="/donate.png" alt="Donate!" title="Donate to the FSF!"></a>
		<a href="/"><img alt="RMS Matthew Stallman" class="stallman" src="{}" height="100%"></a>
	</body>
</html>"#,
        title,
        if js_enabled {
            ""
        } else {
            r#"<meta http-equiv="refresh" content="3;/">"#
        },
        random_image()
    );
    Ok(Response::with((
        headers::ContentType::html().0,
        status::Ok,
        resp,
    )))
}

fn list_images() -> IronResult<Response> {
    Ok(Response::with((
        headers::ContentType::json().0,
        status::Ok,
        &*IMAGE_JSON.clone(),
    )))
}

fn random_image() -> &'static str {
    rand::thread_rng().choose(&IMAGES).unwrap()
}

fn images() -> Vec<String> {
    let imgs = std::fs::read_dir("./static/img").expect("could  not read img dir");

    imgs.filter_map(|entry| {
        let entry = entry.expect("error reading item");
        match entry.file_type().expect("file type") {
            t if t.is_file() => {
                // Images can be found relative to '/' in the api, so prefix their relative paths
                // with '/' here.
                Some(
                    "/".to_string() + entry
                        .path()
                        .strip_prefix("./static/")
                        .unwrap()
                        .to_str()
                        .unwrap(),
                )
            }
            _ => None,
        }
    }).collect()
}
