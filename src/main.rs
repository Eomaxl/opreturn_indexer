#[macro_use] extern crate rocket;
extern crate dotenv;

mod api;
mod db;
mod bitcoin_client;
mod schema;

use rocket::fairing::AdHoc;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::on_ignite("Database Migrations", |rocket| {
            Box::pin(async move {
                let pool = db::establish_connection();
                rocket.manage(pool)
            })
        }))
        .mount("/", routes![api::get_opreturn_data])
}
