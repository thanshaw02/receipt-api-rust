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
fn process_receipt(receipt: receipt::Receipt) -> String {
  // let mut items_string = "".to_string();
  // let items = &receipt.items;
  // for item in 0..items.len() { 
  //   let foo = format!("{{ short_description: {}, price: {} }}\n", items[item].short_description, items[item].price).to_string();
  //   println!("Item: {}", foo);
  //   items_string += &foo
  // }

  // let items = &receipt.items;
  // print!("Items: {:#?}", receipt.items);

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