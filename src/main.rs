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

#[macro_use] extern crate rocket;

#[get("/datum/<how_much>")]
fn datum_gen(how_much : i64) -> Option<Json<Vec<String>>> {
    return Some(Json(datumgen::gen_dates(how_much)?));
}

#[get("/telnr/<how_much>")]
fn tel(how_much : i64) -> Json<Vec<String>>{
    return Json(telnr::gen(how_much))
}

#[get("/email/<how_much>")]
fn mail(how_much : usize) -> Option<Json<Vec<String>>> {
    return Some(Json(email::gen_email(how_much)?));
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/namen", routes![tel,datum_gen,mail])
}
