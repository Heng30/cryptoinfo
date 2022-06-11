use tokio;
use webserver::controller::frontend;

pub fn init() {
    tokio::spawn(async move {
        rocket::ignite()
            .mount("/", routes![frontend::index::index, frontend::index::css])
            .launch();
    });
}
