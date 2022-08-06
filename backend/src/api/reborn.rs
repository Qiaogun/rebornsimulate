use rand::prelude::*;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use lazy_static::lazy_static;

#[derive(Deserialize, Debug)]
struct CountryData {
    #[serde(rename = "Country Name")]
    country_name: String,
    //#[serde(rename = "2020")]
    //population: f64,
    #[serde(rename = "Value")]
    weight_value: f64,
}

lazy_static! {
    static ref COUNTRIES: Vec<CountryData> = {
        let mut file = File::open(r"Z:\data.json").unwrap();
        let mut json = String::new();
        file.read_to_string(&mut json).unwrap();
        
        serde_json::from_str(json.as_str()).unwrap()
    };

    static ref SUMS: Vec<f64> = {
        let mut sums = vec![0f64; COUNTRIES.len() + 1];
        for i in 1..sums.len() {
            sums[i] = sums[i - 1] + COUNTRIES[i - 1].weight_value;
        }

        sums
    };
}

#[get("/sex")]
pub fn get_sex()-> &'static str {
    let mut rng = rand::thread_rng().gen_range(0..10);
    let result = if let 0=rng%2{"0"} else{"1"};
    return result;
}

#[get("/reborn")]
pub fn reborn() -> &'static str {
    let mut rng = rand::thread_rng();
    let rand_value = rng.gen::<f64>() * SUMS.last().unwrap();
    
    let index = match SUMS.binary_search_by(|x| x.partial_cmp(&rand_value).unwrap()) {
        Ok(index) => index,
        Err(index) => index - 1,
    };

    COUNTRIES[index].country_name.as_str()
}

