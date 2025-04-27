use diesel::prelude::*;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Queryable, Selectable, Serialize, ToSchema, Debug, Clone)]
#[diesel(table_name = crate::db::schema::category)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Category {
    pub id: i32,
    pub name: String
}

#[derive(Insertable, Serialize)]
#[diesel(table_name = crate::db::schema::category)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewCategory {
    pub name: String,
}

#[derive(Queryable, Selectable, Serialize, ToSchema, Debug, Clone)]
#[diesel(table_name = crate::db::schema::subcategory)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SubCategory {
    pub id: i32,
    pub name: String,
    pub category_id: i32,
}

#[derive(Insertable, Serialize)]
#[diesel(table_name = crate::db::schema::subcategory)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewSubCategory {
    pub name: String,
    pub category_id: i32,
}

#[derive(Queryable, Selectable, Debug, Serialize)]
#[diesel(table_name = crate::db::schema::sound)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PSound {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub duration: f32,
    pub play_count: i32,
    pub category_id: Option<i32>,
    pub subcategory_id: Option<i32>
}

#[derive(Queryable, Selectable, Debug, Serialize, ToSchema, Clone)]
#[diesel(table_name = crate::db::schema::sound)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Sound {
    pub id: i32,
    pub name: String,
    // pub path: String,
    pub duration: f32,
    pub play_count: i32,
    pub category_id: Option<i32>,
    pub subcategory_id: Option<i32>
}

#[derive(Insertable, Debug, Serialize)]
#[diesel(table_name = crate::db::schema::sound)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewSound {
    pub name: String,
    pub path: String,
    pub duration: f32,
    pub play_count: i32,
    pub category_id: Option<i32>,
    pub subcategory_id: Option<i32>
}