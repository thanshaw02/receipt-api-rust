#[macro_use] extern crate rocket;
use rocket::fs::NamedFile;
use rocket::response::Redirect;
use rocket::tokio::time::{sleep, Duration};
use rocket::tokio::task::spawn_blocking;
use std::path::{PathBuf, Path};
use std::io;
use rocket::serde::{Deserialize, json::Json};

mod receipt;

// POST endpoint for processing Receipt objects
#[post("/receipts/process", format = "application/json", data = "<receipt>")]
fn process_receipt(receipt: Json<receipt::Receipt>) -> String {
  format!("\nRetailer: {}\nPurchase Date: {}\nPurchase Time: {}\nTotal: {}", receipt.retailer, receipt.purchase_date, receipt.purchase_time, receipt.total)
} 

// GET endpoint for fetching Receipt points by id
#[get("/receipts/<id>/points")]
fn get_receipt_points(id: String) -> String {
  format!("Receipt ID: {}", id)
}

#[launch]
fn rocket() -> _ {
	let my_routes = routes![
    get_receipt_points,
    process_receipt,
	];

  rocket::build().mount("/", my_routes)
}