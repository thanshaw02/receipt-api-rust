#[macro_use] extern crate rocket;
use std::io;

use rocket::tokio::time::{sleep, Duration};
use rocket::tokio::task::spawn_blocking;

#[get("/world")]
fn world() -> &'static str {
  "Hello, world!"
}

// this uses async functionality, really cool
#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
	sleep(Duration::from_secs(seconds)).await;
	format!("Waited for {} seconds", seconds)
}

// downloads the "data.txt" file
// this converts a synchronous operation into an async one, i think?
#[get("/blocking-tasks")]
async fn blocker_task() -> io::Result<Vec<u8>> {
	// In a real app, use rocket::fs::NamedFile or tokio::fs::File.
	let vec = spawn_blocking(|| std::fs::read("data.txt")).await
		.map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;

	Ok(vec)
}

#[launch]
fn rocket() -> _ {
	let my_routes = routes![
		world,
		delay,
		blocker_task
	];

  rocket::build().mount("/", my_routes)
}