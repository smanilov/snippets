use actix_web::{
    post,
    web::{self, Data},
    App, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use std::ops::DerefMut;
use std::sync::Mutex;
use tracing::{error, info};

#[derive(Debug, Serialize, Deserialize)]
struct GetSessionRequest {
    access_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct StorageData {
    counter: u32,
}

#[post("/get_session")]
async fn get_session(
    app_data: Data<Mutex<StorageData>>,
    request: web::Json<GetSessionRequest>,
) -> impl Responder {
    info!("Reached handler app_data={app_data:?} request={request:?}");
    match app_data.lock() {
        Ok(mut guard) => guard.deref_mut().counter += 1,
        Err(err) => error!("Could not lock mutex: {err}"),
    };
    "all good"
}

async fn read_data() -> StorageData {
    StorageData { counter: 0 }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Enable tracing
    tracing_subscriber::fmt().init();

    let address = "0.0.0.0";
    let port = 11840;

    let data = Data::new(Mutex::new(read_data().await));

    info!("Starting server on address {address} with port {port}");

    HttpServer::new(move || {
        App::new().app_data(Data::clone(&data)).service(get_session) // Register the handler
    })
    .bind((address, port))?
    .run()
    .await
}
