pub(crate) mod models;
pub(crate) mod schema;

use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use diesel::associations::HasTable;
use rocket_sync_db_pools::database;


#[database("soundshouter")]
pub struct Db(diesel::SqliteConnection);


pub fn establish_connection(uri: Option<&String>) -> SqliteConnection {
    dotenv().ok();

    let database_url = if let Some(dburi) = uri {
        dburi
    }
    else {
        &env::var("DATABASE_URL").expect("DATABASE_URL must be set")
    };
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

use crate::db::models::*;
use crate::db::schema::sound::dsl::sound;

pub fn load_all_psounds(db_uri: &String) -> Result<Vec<PSound>, diesel::result::Error> {
    let connection = &mut establish_connection(Some(db_uri));

    sound.select(PSound::as_select())
        .load(connection)
}

pub async fn load_sounds(db: Db, limit: i64, skip: i64) -> Result<Vec<Sound>, diesel::result::Error> {
    use crate::db::schema::sound::dsl::*;

    db.run(move |c| {
        sound.limit(limit)
            .offset(skip)
            .select(Sound::as_select())
            .load(c)
    }).await
}

pub async fn load_categories(db: Db, limit: i64, skip: i64) -> Result<Vec<Category>, diesel::result::Error> {
    use crate::db::schema::category::dsl::*;

    db.run(move |c| {
        category.limit(limit)
            .offset(skip)
            .select(Category::as_select())
            .load(c)
    }).await
}

pub async fn load_subcategories(db: Db, limit: i64, skip: i64) -> Result<Vec<SubCategory>, diesel::result::Error> {
    use crate::db::schema::subcategory::dsl::*;

    db.run(move |c| {
        subcategory.limit(limit)
            .offset(skip)
            .select(SubCategory::as_select())
            .load(c)
    }).await
}

pub fn get_or_create_category(connection: &mut SqliteConnection, cat_name: &str) -> Result<Category, diesel::result::Error> {
    use crate::db::schema::category::dsl::*;

    let result = category.filter(name.eq(cat_name))
        .limit(1)
        .select(Category::as_select())
        .load(connection)?;

    if let Some(cat) = result.first() {
        Ok(cat.clone())
    }
    else {
        let new_category = NewCategory {
            name: cat_name.to_string(),
        };
        diesel::insert_into(category::table())
            .values(&new_category)
            .returning(Category::as_returning())
            .get_result(connection)
    }
}

pub fn get_or_create_subcategory(connection: &mut SqliteConnection, subcat_name: &str, cat_id: i32) -> Result<SubCategory, diesel::result::Error> {
    use crate::db::schema::subcategory::dsl::*;

    let result = subcategory
        .filter(name.eq(subcat_name))
        .filter(category_id.eq(cat_id))
        .limit(1)
        .select(SubCategory::as_select())
        .load(connection)?;

    if let Some(subcat) = result.first() {
        Ok(subcat.clone())
    }
    else {
        let newsub = NewSubCategory {
            name: subcat_name.to_string(),
            category_id: cat_id,
        };
        diesel::insert_into(subcategory::table())
            .values(&newsub)
            .returning(SubCategory::as_returning())
            .get_result(connection)
    }
}

pub fn get_or_create_sound(connection: &mut SqliteConnection, sound_name: &str, spath: &str, sduration: f32, cat_id: Option<i32>, subcat_id: Option<i32>) -> Result<Sound, diesel::result::Error> {
    use crate::db::schema::sound::dsl::*;

    let result = sound
        .filter(name.eq(sound_name))
        .filter(path.eq(spath))
        .limit(1)
        .select(Sound::as_select())
        .load(connection)?;

    if let Some(esound) = result.first() {
        Ok(esound.clone())
    }
    else {
        let newsound = NewSound {
            name: sound_name.to_string(),
            path: spath.to_string(),
            duration: sduration,
            play_count: 0,
            category_id: cat_id,
            subcategory_id: subcat_id,
        };
        let res2 = diesel::insert_into(sound::table())
            .values(&newsound)
            .returning(Sound::as_returning())
            .get_result(connection);

        res2
    }
}

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn run_migration(conn: &mut SqliteConnection) {
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}