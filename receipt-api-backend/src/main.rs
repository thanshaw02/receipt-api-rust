#[macro_use] extern crate rocket;
use rocket::tokio::time::{sleep, Duration};

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

#[launch]
fn rocket() -> _ {
  rocket::build().mount("/", routes![world, delay])
}