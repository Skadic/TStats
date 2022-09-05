use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::http::Status;
use rocket::{response::content::RawJson, tokio::sync::Mutex};
use rocket::{Request, Response};
use rosu_v2::{Osu, OsuBuilder};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

#[macro_use]
extern crate rocket;

mod map;
mod stage;
mod tournament;

#[get("/test_stage")]
fn hello() -> RawJson<&'static str> {
    RawJson(r#"{ "id": 69, "tournament": "DM69", "stage": "RO1337", "map": "HD2" }"#)
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
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        println!("Attached Cors Headers to response");
    }
}

#[launch]
async fn rocket() -> _ {
    dotenv::dotenv().ok();

    let client_id = std::env::var("OSU_CLIENT_ID")
        .expect("OSU_CLIENT_ID not set")
        .parse::<u64>()
        .expect("OSU_CLIENT_ID must be an unsigned integer");
    let client_secret = std::env::var("OSU_CLIENT_SECRET").expect("OSU_CLIENT_SECRET not set");
    let postgres_connection_url = std::env::var("POSTGRES_URL").expect("POSTGRES_URL not set");

    //let _pool2 = SqlitePoolOptions::new()
    //    .max_connections(4)
    //    .connect("test.db")
    //    .await
    //    .expect("Error connecting to database");

    let pool = PgPoolOptions::new()
        .max_connections(4)
        .connect(&postgres_connection_url)
        .await
        .expect("Error connecting to database");

    let redis_client =
        redis::Client::open("redis://127.0.0.1/").expect("Error creating Redis client");

    let redis_conn = redis_client
        .get_tokio_connection()
        .await
        .expect("Error establishing connection to Redis Database");

    rocket::build()
        // -- Fairings --
        .attach(CORS)
        // -- Routes --
        .mount("/api", routes![hello, cors_fix])
        .mount(
            "/api/tournament",
            routes![
                tournament::create_tournament,
                tournament::get_by_shorthand,
                tournament::get,
                tournament::get_all
            ],
        )
        .mount(
            "/api/stage",
            routes![stage::create, stage::get_all, stage::get],
        )
        .mount("/api/map", routes![map::get_test_map])
        // -- State Management --
        .manage::<Osu>(
            OsuBuilder::new()
                .client_id(client_id)
                .client_secret(client_secret)
                .build()
                .await
                .expect("Could error connecting to osu api"),
        )
        .manage::<Pool<Postgres>>(pool)
        .manage::<Mutex<redis::aio::Connection>>(Mutex::new(redis_conn))
}
