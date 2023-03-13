#[macro_use] extern crate rocket;
use rocket::fs::NamedFile;
use rocket::response::Redirect;
use rocket::tokio::time::{sleep, Duration};
use rocket::tokio::task::spawn_blocking;
use std::path::{PathBuf, Path};
use std::io;

// json
use rocket::serde::{Deserialize, json::Json};

// needed for cookie access
use rocket::http::{CookieJar, Cookie};

// my own imports
mod user;

/************ Body Data *************/

#[post("/user-test", format = "json", data = "<user>")]
fn user_test(user: Json<user::User>) -> String {
	format!("User's name: {}\nUser's age: {}", user.name, user.age)
}

/********* End of Body Data *********/

/************ Formats *************/

// NOTE: using 'format = "some/format"' in a GET request specifies what the endpoint expects for the "Accept" header
// NOTE: using 'format = "some/format"' in a POST request specifies what the endpoint expects for the "Content-Type" header

// this is for media types accepted, stuff like that (headers, etc.)

// NOTE: "data = ''" specifies the body data sent via a POST or PUT endpoint
#[post("/user", format = "application/json", data = "<user>")]
fn new_user(user: user::User) -> String {
	format!("User's name: {}\nUser's age: {}", user.name, user.age)
}

/********* End of Formats *********/

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
fn user_top(id: usize) -> String { 
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

/********* End of Forwarding *********/

/*********** Cookies ***********/

/* Public Cookies */
// CookieJar::add() sets cookies that are accessable by the client (public cookies)

#[get("/")]
fn index(cookies: &CookieJar<'_>) -> Option<String> {
	cookies.get("message").map(|crumb| format!("Message: {}", crumb.value()))
}

#[get("/set-cookie")]
fn set_cookie(cookies: &CookieJar<'_>) -> Redirect {
	cookies.add(Cookie::new("message", "Hello there!"));
	Redirect::to(uri!(index))
}

/* Private Cookies */
// these cookies are similar to public cookies except these are encrypted using authenticated encryption
// these cookies cannot be inspected or tampered with by the client
// support for private cookies must manually be added by importing the optional Rocket feature "secrets" in your Cargo.toml dependancy list

#[get("/get-private-cookie")]
fn get_private_cookie(cookies: &CookieJar<'_>) -> Option<String> {
	cookies.get_private("private-message").map(|crumb| format!("Private cookie: {}", crumb.value()))
}

#[get("/set-private-cookie")]
fn set_private_cookie(cookies: &CookieJar<'_>) -> Redirect {
	cookies.add_private(Cookie::new("private-message", "Super secret private cookie message!"));
	Redirect::to(uri!(get_private_cookie))
}


/******* End of Cookies ********/

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
		index, // cookie work
		// everything,
		foo_bar,
		get_private_cookie,
		hello,
		new_user,
		set_cookie, // setting a cookie to use in the "index" route
		set_private_cookie,
		user_int,
		user_str,
		user_test, // json stuff
		user_top,
		world,
	];

  rocket::build().mount("/", my_routes)
}