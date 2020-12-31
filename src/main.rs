use warp::Filter;

mod create_recipe;
mod db;
mod error;
mod list_recipes;
mod templates_loader;

use db::with_db;

#[tokio::main]
async fn main() {
    let with_templates = templates_loader::load_templates();

    let db_pool = db::init().await;

    let list_recipes = warp::get()
        .and(warp::path("recipes"))
        .and(warp::path::end())
        .and(with_db(db_pool.clone()))
        .and(with_templates.clone())
        .and_then(list_recipes::get);
    let create_recipe_form = warp::get()
        .and(warp::path("create"))
        .and(warp::path("recipe"))
        .and(warp::path::end())
        .and(with_templates.clone())
        .and_then(create_recipe::get);
    let create_recipe = warp::post()
        .and(warp::path("create"))
        .and(warp::path("recipe"))
        .and(warp::path::end())
        .and(warp::body::form())
        .and(with_db(db_pool.clone()))
        .and(with_templates.clone())
        .and_then(create_recipe::post);

    let routes = list_recipes.or(create_recipe_form).or(create_recipe);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
