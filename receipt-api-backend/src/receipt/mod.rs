use rocket::{data::{FromData, Data, self, ToByteUnit}, http::{Status, ContentType}};
use rocket::request::Request;
use serde::Deserialize;
use serde_json::Value;


#[derive(Deserialize)]
#[derive(Debug)]
pub struct Item {
  pub short_description: String,
  pub price: String, // change this to some integer or double if i can
}

// leaving both the "id" and the "points" attributes commented out for not just to get the endpoints working
// will come back to this in issue #4 https://github.com/thanshaw02/receipt-api-rust/issues/4
#[derive(Deserialize)]
#[derive(Debug)]
pub struct Receipt {
  // pub id: String,
  pub retailer: String,
  pub purchase_date: String,
  pub purchase_time: String,
  pub items: Vec<Item>,
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

    // convert string data to JSON
    // there may be a better way tro do this, i'll make an issue related to refactoring and cleanign this up once it's working
    let str_data: &str = string_data.as_str();
    let json_data: Value = match serde_json::from_str(str_data) {
      Ok(json) => json,
      Err(_) => return Failure((Status::UnprocessableEntity, JsonParseError))
    };

    // extract all of the data (super ugly)
    let retailer = json_data["retailer"].to_string();
    let purchase_date = json_data["purchase_date"].to_string();
    let purchase_time = json_data["purchase_time"].to_string();
    let items_pointer = match json_data["items"].as_array() {
      Some(v) => v,
      None => return Failure((Status::UnprocessableEntity, JsonParseError))
    };
    let total = json_data["total"].to_string();

    let value_items = items_pointer;
    let mut items: Vec<Item> = vec!();

    // loop through items vector/array to get it looking like we need it to look
    for i in 0..value_items.len() {
      let short_description = value_items[i]["short_description"].to_string();
      let price = value_items[i]["price"].to_string();

      let item = Item {
        short_description,
        price
      };
      
      items.push(item);
    }

    let receipt: Receipt = Receipt {
      retailer,
      purchase_date,
      purchase_time,
      items,
      total,
    };

    println!("Constructed Receipt object:\n{:#?}", receipt); // debugging

    // return the Receipt object
    Success(receipt)
  }
}