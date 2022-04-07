use rand::Rng;

#[get("/sex")]
pub fn get_sex()-> &'static str {
    let mut rng = rand::thread_rng().gen_range(0..10);
    let result = if let 0=rng%2{"0"} else{"1"};
    return result;
}


fn read_date()-> &'static str {}

pub fn get_country()->&'static str{
    return "url";
}

//#[get("/flag")]


//#[get("/reborn")]
