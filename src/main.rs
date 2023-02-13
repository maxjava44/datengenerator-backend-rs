#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::restriction)]
#![warn(clippy::nursery)]
#![warn(clippy::perf)]
#![allow(clippy::missing_docs_in_private_items)]

mod test;
mod telnr;
mod datumgen;
mod email;

use rocket::serde::json::Json;
use rocket::http::{Cookie, CookieJar};
use rocket::tokio::task;

#[macro_use] extern crate rocket;

#[get("/datum/<how_much>")]
async fn datum_gen(how_much : i64, cookies: &CookieJar<'_>) -> Option<Json<Vec<String>>> {
    let result = task::spawn_blocking(move || {
        Some(Json(datumgen::gen_dates(how_much)?))
    }).await.ok()?;
    let cookie = cookies.get_private("Test");
    match cookie {
        Some(c) => {println!("{}",c.value())},
        None => cookies.add_private(Cookie::new("Test", format!("{}",how_much)))
    }
    return result;
}

#[get("/telnr/<how_much>")]
async fn tel(how_much : i64) -> Option<Json<Vec<String>>>{
    let result = task::spawn_blocking(move || {
        Json(telnr::gen(how_much))
    }).await.ok();
    return result
}

#[get("/email/<how_much>")]
async fn mail(how_much : usize) -> Option<Json<Vec<String>>> {
    let result = task::spawn_blocking(move || {
        Some(Json(email::gen_email(how_much)?))
    }).await.ok()?;
    return result;
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/namen", routes![tel,datum_gen,mail])
}
