use serde::Serialize;
use tera::Tera;

use std::sync::Arc;

use crate::db::{get_db_conn, DBPool};
use crate::error;

#[derive(Serialize)]
struct RecipeListItem {
    name: String,
}

#[derive(Serialize)]
struct ListRecipesContext {
    recipes: Vec<RecipeListItem>,
}

async fn list_recipes(
    db_pool: DBPool,
    tera: Arc<Tera>,
) -> Result<impl warp::Reply, error::Error> {
    let db = get_db_conn(&db_pool).await?;
    let rows = db.query("SELECT (name) FROM Recipes", &[]).await?;
    let tera_context = ListRecipesContext {
        recipes: rows
            .iter()
            .map(|row| RecipeListItem {
                name: row.get("name"),
            })
            .collect(),
    };
    Ok(warp::reply::html(tera.render(
        "list_recipes.tera",
        &tera::Context::from_serialize(tera_context)?,
    )?))
}

pub async fn get(
    db_pool: DBPool,
    tera: Arc<Tera>,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(list_recipes(db_pool, tera).await?)
}
