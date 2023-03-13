use rocket::{data::{FromData, Data, self, ToByteUnit}, http::{Status, ContentType}};
use rocket::request::Request;
use serde::Deserialize;
use serde_json::Value;


#[derive(Deserialize)]
pub struct Item {
  pub short_description: String,
  pub price: String, // change this to some integer or double if i can
}

// leaving both the "id" and the "points" attributes commented out for not just to get the endpoints working
// will come back to this in issue #4 https://github.com/thanshaw02/receipt-api-rust/issues/4
#[derive(Deserialize)]
pub struct Receipt {
  // pub id: String,
  pub retailer: String,
  pub purchase_date: String,
  pub purchase_time: String,
  // pub items: Vec<Item>,
  pub total: String,
  // pub points: String
}

// i want to add more errors here specific to the errors that can happen on a Receipt object (missing data, invalid data, etc.)
#[derive(Debug)]
pub enum Error {
  JsonParseError,
  TooLarge,
  Io(std::io::Error),
}

#[rocket::async_trait]
impl<'r> FromData<'r> for Receipt {
  type Error = Error;

  async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
    use Error::*;
    use rocket::outcome::Outcome::*;
    
    // content type of POST is application/json
    let person_ct = ContentType::new("application", "json");
    if req.content_type() != Some(&person_ct) {
      return Forward(data);
    }

    // use a configured limit with name 'receipt' or fallback to default
    let limit = req.limits().get("receipt").unwrap_or(256.bytes());

    // read data into a String
    let string_data = match data.open(limit).into_string().await {
      Ok(string) if string.is_complete() => string.into_inner(),
      Ok(_) => return Failure((Status::PayloadTooLarge, TooLarge)),
      Err(e) => return Failure((Status::InternalServerError, Io(e)))
    };

    println!("Receipt object as string:\n{}", string_data);

    // convert string data to JSON
    // there may be a better way tro do this, i'll make an issue related to refactoring and cleanign this up once it's working
    let str_data: &str = string_data.as_str();
    let json_data: Value = match serde_json::from_str(str_data) {
      Ok(json) => json,
      Err(_) => return Failure((Status::UnprocessableEntity, JsonParseError))
    };

    println!("Receipt object:");
    println!(
      "Retailer: {}\nPurchase Date: {}\nPurchase Time: {}\nItems: {}\nTotal: {}", 
      json_data["retailer"].to_string(), 
      json_data["purchase_date"].to_string(),
      json_data["purchase_time"].to_string(),
      json_data["items"], // may not work
      json_data["total"].to_string()
    );

    let retailer = json_data["retailer"].to_string();
    let purchase_date = json_data["purchase_date"].to_string();
    let purchase_time = json_data["purchase_time"].to_string();
    // let items = json_data["items"] as Vec<Item>;
    let total = json_data["total"].to_string();

    // temporary
    // let short_description: String = "Redbull".to_string();
    // let price: String = "3.29".to_string();
    // let item: Item = Item { short_description, price };
    // let v = vec![item, item];

    let receipt: Receipt = Receipt {
      retailer,
      purchase_date,
      purchase_time,
      // v,
      total,
    };

    // return the Receipt object
    Success(receipt)
  }
}