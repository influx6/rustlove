// library

#[macro_use]
extern crate actix_web;

use actix_web::{
    error::{Error, InternalError, JsonPayloadError },
    middleware, web, App, HttpServer, HttpRequest, HttpResponse, Result
};
use serde::{Serialize, Deserialize};
use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

const LOG_FORMAT: &'static str = r#""%r" %s %b "%{User-Agent}i" %D"#;

static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);

struct AppState {
    server_id: usize,
    request_count: Cell<usize>,
    messages: Arc<Mutex<Vec<String>>>,
}

pub struct MessageApp {
    port: u16,
}

impl MessageApp {
    pub fn new(port: u16) -> Self {
        MessageApp{ port }
    }

    pub fn run(&self) -> std::io::Result<()> {
        println!("Starting http server: 127.0.0.1:{}", self.port);

        let messages = Arc::new(Mutex::new(vec![]));

        // move informs the compiler to give ownership of the arguments
        // and variables/references to the closure rather than creating
        // references.
        HttpServer::new(move || {
            App::new()
                .data(AppState{
                    server_id: SERVER_COUNTER.fetch_add(1, Ordering::SeqCst),
                    request_count: Cell::new(0),
                    messages: messages.clone(),
                })
                .wrap(middleware::Logger::new(LOG_FORMAT))
                .service(index)
                .service(lookup)
                .service(
                    web::resource("/clear")
                    .route(web::post().to(clear_messages))
                )
                .service(
                    web::resource("/send")
                    .data(web::JsonConfig::default().limit(4096).error_handler(post_error))
                    .route(web::post().to(post))
                )
        })
        .bind(("127.0.0.1", self.port))?  // ? indicates that the result OK should be called for the next subsequent calls else return early
        .workers(8)
        .run()
    }
}

#[derive(Serialize)]
struct PostError {
    server_id: usize,
    request_count: usize,
    error: String,
}

fn post_error(err: JsonPayloadError, req: &HttpRequest) -> Error {
    let extns = req.extensions();
    let state = extns.get::<web::Data<AppState>>().unwrap();

    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);

    let post_error = PostError {
        server_id: state.server_id,
        request_count,
        error: format!("{}", err),
    };

    InternalError::from_response(err, HttpResponse::BadRequest().json(post_error)).into()
}

#[derive(Deserialize)]
struct PostInput {
    message: String,
}


#[derive(Serialize)]
struct PostResponse {
    message: String,
    server_id: usize,
    request_count: usize,
}

fn post(msg: web::Json<PostInput>, state: web::Data<AppState>) -> Result<web::Json<PostResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);

    let mut ms = state.messages.lock().unwrap();
    ms.push(msg.message.clone());

    Ok(web::Json(PostResponse{
        server_id: state.server_id,
        request_count: request_count,
        message: msg.message.clone(),
    }))
}

fn clear_messages(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);

    let mut ms = state.messages.lock().unwrap();
    ms.clear();

    Ok(web::Json(IndexResponse{
        server_id: state.server_id,
        request_count: request_count,
        message: vec![],
    }))
}

#[derive(Serialize)]
struct IndexResponse {
    message: Vec<String>,
    server_id: usize,
    request_count: usize,
}


#[get("/")]
fn index(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
    let msg = state.messages.lock().unwrap();

    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);


    Ok(web::Json(IndexResponse{
        server_id: state.server_id,
        request_count: request_count,
        message: msg.clone(),
    }))
}

#[derive(Serialize)]
struct LookupResponse {
    result: Option<String>,
    server_id: usize,
    request_count: usize,
}

#[get("/lookup/{index}")]
fn lookup(state: web::Data<AppState>, idx: web::Path<usize>) -> Result<web::Json<LookupResponse>> {
    let msg = state.messages.lock().unwrap();

    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);

    let result = msg.get(idx.into_inner()).cloned();

    Ok(web::Json(LookupResponse{
        server_id: state.server_id,
        request_count: request_count,
        result,
    }))
}
