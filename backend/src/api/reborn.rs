use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rand::Rng;
// use redis::{AsyncCommands, RedisResult};
use rocket::State;

pub fn set_sex()->&'static str{
    let mut rng = rand::thread_rng().gen_range(0..10);
    let result = if let 0=rng%2{"0"} else{"1"};
    return result;
}

pub fn get_country()->&'static str{
    return "url";
}

pub fn get_flag(country: &str)->&'static str{
    return "url";
}

//#[get("/reborn")]
