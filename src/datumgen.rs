use rand::Rng;
use rocket::time::Date;


pub fn gen_dates(how_much: i64) -> Option<Vec<String>> {
    let mut rand = rand::thread_rng();
    let mut returnee : Vec<String> = Vec::new();
    for _ in 0..how_much {
        let year : i32 = (rand.gen::<f32>() * (2023.0-1933.0)).round() as i32 + 1933; //Jahr zwischen 1933 und 2023
        let day : u16 = (rand.gen::<f64>() * 364.0).round() as u16 + 1; //Datum zwischen 1 und 365
        let generated_day = Date::from_ordinal_date(year,day).ok()?;

        returnee.push(format!("{}.{}.{}",generated_day.day(),u8::from(generated_day.month()),generated_day.year()));
    }
    Some(returnee)
}
