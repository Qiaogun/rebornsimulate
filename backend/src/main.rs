mod api;
use crate::api::reborn::*;
#[macro_use] 
extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/api", routes![hello,get_sex,reborn])
}