use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::http::Status;
use rocket::log::private::{info, warn};
use rocket::{response::content::RawJson, tokio::sync::Mutex};
use rocket::{Request, Response};
use rosu_v2::{Osu, OsuBuilder};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};

#[macro_use]
extern crate rocket;

mod error;
mod map;
mod stage;
mod tournament;
mod util;

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
    match dotenvy::dotenv().ok() {
        Some(path) => info!(".env file successfully loaded from path \"{}\"", path.to_string_lossy()),
        None => warn!("No .env file found")
    };

    let client_id = std::env::var("OSU_CLIENT_ID")
        .expect("OSU_CLIENT_ID not set")
        .parse::<u64>()
        .expect("OSU_CLIENT_ID must be an unsigned integer");
    let client_secret = std::env::var("OSU_CLIENT_SECRET").expect("OSU_CLIENT_SECRET not set");
    let db_connection_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL not set");

    //let _pool2 = SqlitePoolOptions::new()
    //    .max_connections(4)
    //    .connect("test.db")
    //    .await
    //    .expect("Error connecting to database");

    //let pool = PgPoolOptions::new()
    //    .max_connections(4)
    //    .connect(&postgres_connection_url)
    //    .await
    //    .expect("Error connecting to database");

    let pool = MySqlPoolOptions::new()
        .max_connections(4)
        .connect(&db_connection_url)
        .await
        .expect("Error connecting to database");

    let redis_client =
        redis::Client::open(redis_url).expect("Error creating Redis client");

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
                tournament::get,
                tournament::get_all
            ],
        )
        // -- Stages --
        .mount(
            "/api/tournament",
            routes![
                stage::create,
                stage::get_all,
                stage::get,
                stage::set_pool_format,
                stage::get_pool_format
            ],
        )
        // -- Maps --
        .mount("/api/tournament", routes![map::get_test_map, map::set_map])
        // -- State Management --
        .manage::<Osu>(
            OsuBuilder::new()
                .client_id(client_id)
                .client_secret(client_secret)
                .build()
                .await
                .expect("Could error connecting to osu api"),
        )
        .manage::<Pool<MySql>>(pool)
        .manage::<Mutex<redis::aio::Connection>>(Mutex::new(redis_conn))
}
