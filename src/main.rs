extern crate curl;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
use curl::easy::Easy;
use curl::easy::List;
use serde_json::{Value, Error};
#[derive(Serialize, Deserialize)]
struct Entry {
  user: String,
  greeting: String,
}

fn readEntries() {
 let mut data = Vec::new();

 let url ="https://j8dkjo4657.execute-api.us-east-1.amazonaws.com/prod/entries";
 let mut handle = Easy::new();

 handle.url(url).unwrap();
 {
 let mut transfer = handle.transfer();
 transfer.write_function(|new_data| {
        data.extend_from_slice(new_data);
        Ok(new_data.len())
  }).unwrap();
  transfer.perform().unwrap();
 }
  println!("{:?}", data);
  let body= String::from_utf8(data).expect("Found invalid UTF-8");
  println!("{}", body);

  let json:Value;  
   match serde_json::from_str(&body) {
     Ok(v) => {
           json=v;
           println!("Items: {}", json["Items"]);
           println!("Item[0]: {}", json["Items"][0]);
           println!("Item[0][date]: {}", json["Items"][0]["date"]);
           println!("Item[0][greeting]: {}", json["Items"][0]["greeting"]);
           println!("Item[0][user]: {}", json["Items"][0]["user"]);
        },
     Err(_) => println!("json con formato incorrecto"),
   } 
}

fn writeEntry() -> Result<(), Error> {
 let url ="https://j8dkjo4657.execute-api.us-east-1.amazonaws.com/prod/entries";
 let mut handle = Easy::new();
 let  entry =  Entry {
   user: "John Doe".to_string(),
   greeting: "Simple!".to_string(),
 };

 let json = serde_json::to_string(&entry)?;
 println!("{}",json);

 let mut list = List::new();
 list.append("content-type: application/json").unwrap();
 handle.url(url).unwrap();
 handle.post(true).unwrap();
 handle.post_fields_copy(json.as_bytes());
 handle.http_headers(list).unwrap();
 let mut data = Vec::new(); 
 {
    let mut transfer = handle.transfer();
    transfer.write_function(|new_data| {
           data.extend_from_slice(new_data);
           Ok(new_data.len())
     }).unwrap();

    transfer.perform().unwrap();
 }
 println!("{:?}", data);
 let body= String::from_utf8(data).expect("Found invalid UTF-8");
 println!("{}", body);
 let json:Value;  
 match serde_json::from_str(&body) {
     Ok(v) => {
           json=v;
           println!("date: {}", json["date"]);
     },
     Err(_) => println!("json con formato incorrecto"),
 }
 Ok(())
}

fn main() {
 readEntries() ;
 writeEntry(); 
}  
