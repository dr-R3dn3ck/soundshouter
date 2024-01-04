pub mod models;

use std::time::Duration;
use models::{Sound, Category, SubCategory};

use reqwest;
use reqwest::Client;
use serde_json::{from_str};
use leptos_query::*;


async fn get_sub_categories(limit: i32) -> Option<Vec<SubCategory>> {
    let body = reqwest::get(
        format!("/api/subcategories?limit={}", limit))
        .await.ok()?
        .text()
        .await.ok()?;

    let parse_result= from_str::<Vec<SubCategory>>(body.as_str());
    match parse_result {
        Ok(res) => Some(res),
        Err(_msg) => None
    }
}

pub fn use_subcategory_query(id: impl Fn() -> i32 + 'static) -> QueryResult<Option<Vec<SubCategory>>, impl RefetchFn> {
    leptos_query::use_query(
        id,
        get_sub_categories,
        QueryOptions {
            default_value: None,
            refetch_interval: None,
            resource_option: ResourceOption::NonBlocking,
            // Considered stale after 10 seconds.
            stale_time: Some(Duration::from_secs(10)),
            // Infinite cache time.
            cache_time: None,
        },
    )
}

async fn get_categories(limit: i32) -> Option<Vec<Category>> {
    let body = reqwest::get(
        format!("/api/categories?limit={}", limit))
        .await.ok()?
        .text()
        .await.ok()?;

    let parse_result= from_str::<Vec<Category>>(body.as_str());
    match parse_result {
        Ok(res) => Some(res),
        Err(_msg) => None
    }
}

pub fn use_category_query(id: impl Fn() -> i32 + 'static) -> QueryResult<Option<Vec<Category>>, impl RefetchFn> {
    leptos_query::use_query(
        id,
        get_categories,
        QueryOptions {
            default_value: None,
            refetch_interval: None,
            resource_option: ResourceOption::NonBlocking,
            // Considered stale after 10 seconds.
            stale_time: Some(Duration::from_secs(10)),
            // Infinite cache time.
            cache_time: None,
        },
    )
}

async fn get_sounds(limit: i32) -> Option<Vec<Sound>> {
    let body = reqwest::get(
        format!("/api/sounds?limit={}", limit))
        .await.ok()?
        .text()
        .await.ok()?;

    let parse_result= from_str::<Vec<Sound>>(body.as_str());
    match parse_result {
        Ok(res) => Some(res),
        Err(_msg) => None
    }
}


pub fn use_sound_query(id: impl Fn() -> i32 + 'static) -> QueryResult<Option<Vec<Sound>>, impl RefetchFn> {
    leptos_query::use_query(
        id,
        get_sounds,
        QueryOptions {
            default_value: None,
            refetch_interval: None,
            resource_option: ResourceOption::NonBlocking,
            // Considered stale after 10 seconds.
            stale_time: Some(Duration::from_secs(10)),
            // Infinite cache time.
            cache_time: None,
        },
    )
}


pub async fn play_sound(id: i32) {
    let client = Client::new();
    let _response = client.put(
        format!("/api/play/{}", id))
        .send()
        .await.unwrap();

    ()
}
