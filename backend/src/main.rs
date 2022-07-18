use rocket::{response::content::RawJson, tokio::sync::Mutex};
use rosu_v2::{OsuBuilder, Osu};
use sqlx::{sqlite::SqlitePoolOptions, Sqlite, Pool};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};
use rocket::http::Status;
use rocket::http::Header;

#[macro_use] extern crate rocket;

mod tournament;

#[get("/test_stage")]
fn hello() -> RawJson<&'static str> {
    RawJson(r#"{ "tournament": "DM69", "stage": "RO1337", "map": "HD2" }"#)
}

/// I am stupid and don't know how webdev works so this is here.
#[options("/<_..>")]
async fn cors_fix() -> Status {
    Status::Ok
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Attaching CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        println!("Attached Cors Headers to response");
    }
}


#[launch]
async fn rocket() -> _ {

    let client_id = std::env::var("MPSTAT_CLIENT_ID")
        .expect("MPSTAT_CLIENT_ID not set")
        .parse::<u64>()
        .expect("MPSTAT_CLIENT_ID must be an unsigned integer");
    let client_secret = std::env::var("MPSTAT_CLIENT_SECRET")
        .expect("MPSTAT_CLIENT_SECRET not set");

    let pool = SqlitePoolOptions::new()
        .max_connections(4)
        .connect("test.db").await
        .expect("Error connecting to database");

    rocket::build()
        .attach(CORS)
        .mount("/api", routes![hello, cors_fix])
        .mount("/api/tournament", routes![tournament::create_tournament, tournament::get_by_shorthand, tournament::get_all])
        .manage::<Mutex<Osu>>(Mutex::new(
            OsuBuilder::new()
                .client_id(client_id)
                .client_secret(client_secret)
                .build().await
                .expect("Could error connecting to osu api")
        ))
        .manage::<Pool<Sqlite>>(pool)
}
