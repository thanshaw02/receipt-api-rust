#[macro_use] extern crate rocket;
use rocket::fs::NamedFile;
use rocket::tokio::time::{sleep, Duration};
use rocket::tokio::task::spawn_blocking;
use std::path::{PathBuf, Path};
use std::io;

/**
 * Request Guard:
 * 	- appear as inputs to router handlers (in the handler's argument list)
 * 	- Rocket automatically invokes the "FromRequest implementation for a requet guard before invoking the handler itself
 * 		- Handlers are only invoked if ALL request guards pass
 * 	- request guards only found in the handlers argument list and NOT in the route's path
 * 	- request guards fire from left to right in the handler's argument list
 * - request guards centralize policies, resulting in a simpler, safer, and more secure applications
 * 
 * Implementing custom Request Guards:
 * 	- you can create a custom type that implements "FromRequest", this will allow your custom type to be used as a request guard for endpoints
 * 		- for example if you require an API key in the route's path
 */

/************ Forwarding *************/
// Routes are executed in order by their rank ranging from -12 to -1, highest ranking being -1 and lowest being -12
// if you have multiple routes with colliding paths then you must manually rank them, if you don't then an error is thrown when starting the server

#[get("/user/<id>")]
fn user(id: usize) -> String { 
	format!("User endpoint with highest rank: {}", id)
}

#[get("/user/<id>", rank = 2)]
fn user_int(id: isize) -> String { 
	format!("User endpoint with second highest rank (integer): {}", id)
}

#[get("/user/<id>", rank = 3)]
fn user_str(id: &str) -> String { 
	format!("User endpoint with third highest rank (string): {}", id)
}

/********* End of forwarding *********/

// to ignore a route segment you can simply use <_> within the route to ignore a single segment
// to ignore multiple segments in a route you can use <_..>
// an ignored paramater must not appear in the handlers argument list however
#[get("/foo/<_>/bar")]
fn foo_bar() -> &'static str {
	// ex. /foo/hiiii/bar --> the "hiiii" segment will be ignored and can be anything
	"Foo ______ bar!"
}

#[get("/<_..>")]
fn everything() -> &'static str {
	"Hey, you're here."
}

// we can also match against multiple segments in a path using <param..> in the routing path
// this paramater type is known as "segment guards" and MUST implement "FromSegments"
// segment guards but be the final component of a route's path, any text or single segments after a segment guard will throw a compile-time error
#[get("/page/<path..>")] // note the name given to the segment guard can be anything, but must match the argument name in the corrosponding handler
fn get_page(path: PathBuf) { /* ... */ }

// thjis example serves a safe and secure static file server
// although it's recommended to do this instead if i want/need to server a static file server in the #[launch] handler:
/**
 *  rocket.mount("/public", FileServer::from("static/"))
 */
#[get("/serve/<file..>")]
async fn serve_page(file: PathBuf) -> Option<NamedFile> {
	NamedFile::open(Path::new("static/").join(file)).await.ok()
}

// by default utilizes "FromParam"'s which is also known as a "paramater guard"
// Rocket implements "paramater guards" for many of the standard library types, including a few special Rocket types
#[get("/hello/<name>/<age>/<cool>")]
fn hello(name: &str, age: u8, cool: bool) -> String {
	if cool {
		format!("You're a cool {} year old, {}!", age, name)
	} else {
		format!("{}, we need to talk about your coolness.", name)
	}
}

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
		blocker_task,
		delay,
		// everything,
		foo_bar,
		hello,
		user,
		user_int,
		user_str,
		world,
	];

  rocket::build().mount("/", my_routes)
}