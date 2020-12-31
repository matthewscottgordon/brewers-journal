use tera::Tera;

use std::sync::Arc;

use std::collections::HashMap;

use crate::db::{get_db_conn, DBPool};
use crate::error;

pub async fn get(tera: Arc<Tera>) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::html(
        tera.render("create_recipe_get.tera", &tera::Context::new())
            .map_err(error::Error::from)?,
    ))
}

pub async fn post(
    form_contents: HashMap<String, String>,
    db_pool: DBPool,
    tera: Arc<Tera>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db = get_db_conn(&db_pool).await.map_err(warp::reject::custom)?;
    db.execute(
        "INSERT INTO Recipes (name) VALUES ($1)",
        &[&form_contents["name"]],
    )
    .await
    .map_err(|e| warp::reject::custom(error::Error::DBQueryError(e)))?;
    Ok(warp::reply::html(
        tera.render("create_recipe_post.tera", &tera::Context::new())
            .map_err(error::Error::from)?,
    ))
}
