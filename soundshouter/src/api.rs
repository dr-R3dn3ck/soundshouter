use std::path::PathBuf;
use crate::db::models::{Category, Sound, SubCategory};
use crate::error::AppError;
use rust_embed::{RustEmbed};

#[derive(RustEmbed)]
#[folder = "../soundshouter-vite-webapp/dist"]
struct Asset;

use log::{debug};

use rocket;
use rocket::{get, post, routes};
use rocket::serde::json::Json;
use rocket::serde::{Serialize};
use rocket::tokio::{task, time};

use rumqttc::{AsyncClient, Event, MqttOptions, QoS, Outgoing};
use std::time::Duration;

use utoipa::{OpenApi};
use utoipa_swagger_ui::SwaggerUi;

use crate::db::{Db, load_sounds, load_categories, load_subcategories};

#[derive(Serialize)]
struct Health {
    status: String
}

/// ensures a valid limit
/// returns min if the value is smaller than min
/// returns max if the value is larger than max
/// Values below zero will be passed as None by rocket, since it is not a valid unsigned integer.
/// The default value is the used, as if no value was passed.
fn get_limit(value: Option<u32>, default: u32, min: u32, max: u32) -> i64 {
    let mut _limit: u32 = value.unwrap_or(default);
    match _limit {
        n if n < min => min.into(),
        v if v >= min && v <= max => v.into(),
        m if m > max => max.into(),
        _ => default.into()
    }
}

#[get("/")]
fn index() -> Result<Json<Health>, AppError> {
    Ok(Json(Health{ status: "ok".to_string() }))
}

#[utoipa::path(
    get,
    path="/api/v1/sounds",
    params(
        ("limit" = Option<u32>, Query, minimum = 1, maximum = 1000,
        description = "limit list length (default 10 if not set)"),
        ("skip" = Option<u32>, Query, minimum = 0, description = "offset list start")
    ),
    responses(
        (status = OK, description = "List of sounds", body = [Sound])
    )
)]
#[get("/sounds?<limit>&<skip>")]
async fn sounds(db: Db, limit: Option<u32>, skip: Option<u32>) -> Result<Json<Vec<Sound>>, AppError> {
    let _limit = get_limit(limit, 10, 1, 1000);
    let sound_list = load_sounds(
        db, _limit, skip.unwrap_or(0).into()).await;

    match sound_list {
        Ok(sounds) => Ok(Json(sounds)),
        Err(_err) => Err(AppError::DBError("could not load sounds".to_string()))
    }
}

#[utoipa::path(
    get,
    path="/api/v1/categories",
    params(
        ("limit" = Option<u32>, Query, minimum = 1, maximum = 1000,
        description = "limit list length (default 10 if not set)"),
        ("skip" = Option<u32>, Query, minimum = 0, description = "offset list start")
    ),
    responses(
        (status = OK, description = "List categories", body = [Category])
    )
)]
#[get("/categories?<limit>&<skip>")]
async fn categories(db: Db, limit: Option<u32>, skip: Option<u32>) -> Result<Json<Vec<Category>>, AppError> {
    let _limit = get_limit(limit, 10, 1, 1000);
    let category_list = load_categories(
        db, _limit, skip.unwrap_or(0).into()).await;

    match category_list {
        Ok(categories) => Ok(Json(categories)),
        Err(_err) => Err(AppError::DBError("could not load categories".to_string()))
    }
}

#[utoipa::path(
    get,
    path="/api/v1/subcategories",
    params(
        ("limit" = Option<u32>, Query, minimum = 1, maximum = 1000,
        description = "limit list length (default 10 if not set)"),
        ("skip" = Option<u32>, Query, minimum = 0, description = "offset list start")
    ),
    responses(
        (status = OK, description = "List subcategories", body = [SubCategory])
    )
)]
#[get("/subcategories?<limit>&<skip>")]
async fn subcategories(db: Db, limit: Option<u32>, skip: Option<u32>) -> Result<Json<Vec<SubCategory>>, AppError> {
    let _limit = get_limit(limit, 10, 1, 1000);
    let subcategory_list = load_subcategories(
        db, _limit, skip.unwrap_or(0).into()).await;

    match subcategory_list {
        Ok(subcategories) => Ok(Json(subcategories)),
        Err(_err) => Err(AppError::DBError("could not load categories".to_string()))
    }
}

#[utoipa::path(
    post,
    path="/api/v1/play/{id}",
    params(
        ("id" = u32, description="sound id", minimum = 1, example = json!(1))
    ),
    responses(
        (status = OK, description = "Put a sound into the play queue"),
        // TODO: stimmt 503? use error.rs
        (status = 503, description = "Couldn't put the sound in queue")
    )
)]
#[post("/play/<id>")]
async fn play(id: u32, config: &rocket::State<Config>) -> Result<Json<u32>, AppError> {

    let mut mqttoptions = MqttOptions::new(
        "soundshouter-api", &config.queue.ip.to_string(), config.queue.port);
    mqttoptions.set_keep_alive(Duration::from_secs(10));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe(&config.queue.topic, QoS::AtMostOnce).await.unwrap();

    let topic = config.queue.topic.clone();
    let _thr = task::spawn(async move {
        debug!("[API] sending to queue: {}", &id);
        let _res = client.publish(
            topic,
            QoS::AtLeastOnce,
            false,
            // vec![1, id]
            format!("{}", id).into_bytes()
        ).await.unwrap();

        time::sleep(Duration::from_millis(100)).await;
        debug!("{:?}", &_res);
    });

    // This is just to wait until the client has sent the message
    while let Ok(notification) = eventloop.poll().await {
        debug!("[API] Received = {:?}", &notification);
        // break;
        match notification {
            Event::Outgoing(Outgoing::Publish(ack)) => {
                debug!("{:?}", &ack);
                break
            }
            _ => {}
        }
    }

    Ok(id.into())
}

use rocket::Shutdown;
use crate::config::{init_app, Config};

#[get("/shutdown")]
fn shutdown(shutdown: Shutdown) -> &'static str {
    shutdown.notify();
    "Shutting down..."
}

use rocket::http::ContentType;
use std::borrow::Cow;
use std::ffi::OsStr;
#[get("/<file..>")]
fn web_app(file: PathBuf) -> Option<(ContentType, Cow<'static, [u8]>)> {
    let _file = if file.display().to_string() == "" {
        PathBuf::from("index.html".to_string())
    }
    else { file };
    let filename = _file.display().to_string();

    let asset = Asset::get(&filename)?;
    let content_type = _file
        .extension()
        .and_then(OsStr::to_str)
        .and_then(ContentType::from_extension)
        .unwrap_or(ContentType::Bytes);

    Some((content_type, asset.data))
}

#[rocket::main]
pub async fn run_api() -> Result<(), rocket::Error> {
    #[derive(OpenApi)]
    #[openapi(
        info(description = "sounds"),
        paths(sounds, categories, subcategories, play),
        components(schemas(Category, SubCategory, Sound))
    )]
    struct ApiDoc;

    // load config again (because rocket::main cannot have parameters)
    let (_dirs, conf) = init_app().expect("failed to init app");

    rocket::build()
        .attach(Db::fairing())
        .manage(conf)
        .mount("/api/v1", routes![
            index, sounds, categories, subcategories, play, shutdown])
        .mount("/", routes![web_app])
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>")
                .url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .launch()
        .await?;

    Ok(())
}
