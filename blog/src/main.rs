use dotenv::dotenv;
use std::env;

fn main() {
	dotenv().ok();

	env::set_var("RUST_LOG", "actix_web=info");
	env_logger.init();

	let database_url = env::var("DATABASE_URL").expect("DATABSE_URL must be set");

	let app = blog::Blog::new(9889);
	app.run(database_url)
}
