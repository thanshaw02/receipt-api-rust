#[macro_use] extern crate rocket;

/* Keeping this imports for now, but they will be removed if I end up not using any of them */
// use rocket::fs::NamedFile;
// use rocket::response::Redirect;
// use std::path::{PathBuf, Path};
// use std::io;
// use rocket::serde::{Deserialize, json::Json};

// Receipt struct module with custom Request Guard implementation
mod receipt;

// POST endpoint for processing Receipt objects
#[post("/receipts/process", format = "application/json", data = "<receipt>")]
fn process_receipt(receipt: receipt::Receipt) -> String {
  // NOTE: Just returning a string for now until I add in the receipt processing logic
  format!(
    "\nRetailer: {}\nPurchase Date: {}\nPurchase Time: {}\nItems: {:?}\nTotal: {}", 
    receipt.retailer, 
    receipt.purchase_date, 
    receipt.purchase_time,
    receipt.items,
    receipt.total
  )
} 

// GET endpoint for fetching Receipt points by id
#[get("/receipts/<id>/points")]
fn get_receipt_points(id: String) -> String {
  // NOTE: Just returning a string for now until I add in the receipt processing logic and store the points
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