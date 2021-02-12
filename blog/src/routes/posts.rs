
use crate::errors::AppError;
use crate::routes::convert;
use crate::{models, Pool};
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use futures::Future;

#[derive(Debug, Serialize, Deserialize)]
struct PostInput {
    body: String,
	title: String,
}


fn create_post(
	user_id: web::Path<i32>,
    post: web::Json<PostInput>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = AppError> {
    web::block(move || {
        let conn = &pool.get().unwrap();
		let target_id = models::UserKey::ID(user_id.into_inner());
		models::find_user(conn, target_id).and_then(|user| {
			let post = post.into_inner();
			let title = post.title;
			let body = post.body;
			models::create_post(conn, &user, title.as_str(), body.as_str())
		})
    }).then(convert)
}


fn get_post(
	post_id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = AppError> {
    web::block(move || {
        let conn = &pool.get().unwrap();
		let target_id = post_id.into_inner();
		models::get_post(conn, target_id)
    }).then(convert)
}

fn publish_post(
	post_id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = AppError> {
    web::block(move || {
        let conn = &pool.get().unwrap();
		let target_id = post_id.into_inner();
		models::publish_post(conn, target_id)
    }).then(convert)
}


fn user_posts(
	user_id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = AppError> {
    web::block(move || {
        let conn = &pool.get().unwrap();
		let target_id = user_id.into_inner();
		models::user_posts(conn, target_id)
    }).then(convert)
}

fn all_posts(
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = AppError> {
    web::block(move || {
        let conn = &pool.get().unwrap();
		models::all_posts(conn)
    }).then(convert)
}

pub fn configure(cfg: &mut web::ServiceConfig) {
	cfg.service(
		web::resource("/users/{id}/posts")
		.route(web::post().to_async(create_post))
		.route(web::get().to_async(get_post)),
	)
	.service(web::resource("/posts").route(web::get().to_async(all_posts)))
	.service(web::resource("/posts/{id}/publish").route(web::post().to_async(publish_post)));
}
