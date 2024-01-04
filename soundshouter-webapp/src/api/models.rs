use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubCategory {
    pub id: i32,
    pub name: String,
    pub category_id: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Sound {
    pub id: i32,
    pub name: String,
    pub duration: f32,
    pub play_count: i32,
    pub category_id: i32,
    pub subcategory_id: Option<i32>,
}

impl Sound {
    pub fn pretty_name(self) -> String {
        return self.name
            .replace("-", " ")
            .replace("_", " ");
    }

    pub fn match_query(&self, query: String) -> bool {
        self.name.contains(&query)
    }
}