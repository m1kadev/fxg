use std::{convert::Infallible, fs, path::PathBuf, sync::Arc};

use colored::Colorize;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};

pub fn start_server(path: PathBuf) -> Result<(), crate::Error> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { start_server_facaded(path).await })
}

async fn start_server_facaded(path: PathBuf) -> Result<(), crate::Error> {
    let addr = "127.0.0.1:6969".parse()?;
    let counter = Arc::new(path);

    let make_service = make_service_fn(move |_conn| {
        let counter = counter.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                let counter = counter.clone();
                async move { Ok::<_, Infallible>(use_counter(counter, req)) }
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_service);
    println!("{} new server on 127.0.0.1:6969", "Started".bold().green());

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
        Err(crate::Error::Hyper(e))
    } else {
        Ok(())
    }
}

fn use_counter(path_am: Arc<PathBuf>, req: Request<Body>) -> Response<Body> {
    let mut path = (*path_am).clone();
    path.push(&req.uri().path()[1..]);
    if path.is_dir() {
        path.push("index.html");
    }
    dbg!(&path);
    if let Ok(contents) = fs::read(path) {
        Response::new(Body::from(contents))
    } else {
        Response::new(Body::from("not found"))
    }
}
