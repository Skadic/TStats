use rocket::{
    fairing::{Fairing, Info, Kind},
    http::{Header, Status},
    log::private::{info, warn},
    tokio::sync::Mutex,
    Request, Response,
};
use rosu_v2::{prelude::Beatmapset, Osu, OsuBuilder};
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

#[macro_use]
extern crate rocket;

mod error;
mod map;
mod stage;
mod tournament;
mod util;

pub type DBPool = Pool<MySql>;

#[get("/ping")]
fn ping() -> (Status, &'static str) {
    (Status::Ok, "Pong!")
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
        debug!("Attached Cors Headers to response");
    }
}

#[launch]
async fn rocket() -> _ {
    match dotenvy::dotenv().ok() {
        Some(path) => info!(
            ".env file successfully loaded from path \"{}\"",
            path.to_string_lossy()
        ),
        None => warn!("No .env file found"),
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

    let redis_client = redis::Client::open(redis_url).expect("Error creating Redis client");

    let redis_conn = redis_client
        .get_tokio_connection()
        .await
        .expect("Error establishing connection to Redis Database");

    let osu = OsuBuilder::new()
        .client_id(client_id)
        .client_secret(client_secret)
        .build()
        .await
        .expect("Error connecting to osu api");

    rocket::build()
        // -- Fairings --
        .attach(CORS)
        // -- Routes --
        .mount("/api", routes![ping, cors_fix])
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
        .mount(
            "/api/tournament",
            routes![map::get_test_map, map::set_map, map::get_map_by_slot],
        )
        // -- State Management --
        .manage::<Osu>(osu)
        .manage::<Pool<MySql>>(pool)
        .manage::<Mutex<redis::aio::Connection>>(Mutex::new(redis_conn))
}

#[cfg(test)]
mod test {
    use rocket::{
        http::{ContentType, Status},
        local::asynchronous::Client,
    };

    #[rocket::async_test]
    async fn test_stage_test() {
        let client = Client::tracked(crate::rocket().await)
            .await
            .expect("Error creating client");
        let request = client.get("/api/ping").dispatch().await;
        assert_eq!(Status::Ok, request.status(), "Status not matching");
        assert_eq!(
            Some(ContentType::Plain),
            request.content_type(),
            "Content Type not matching"
        );
        assert_eq!(
            Some("Pong!".to_owned()),
            request.into_string().await,
            "Content not matching"
        );
    }
}
