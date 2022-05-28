#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct User {
    name: String,
    surname: String,
    age: usize,
}

#[get("/", format = "json", data = "<user>")]
fn merhaba(user: Json<User>) -> String {
    let user_name = &user.name;
    let user_surname = &user.surname;
    let dogum_yili = 2022 - user.age;
    format!("Merhaba {user_name} {user_surname}, {dogum_yili}")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![merhaba])
}


