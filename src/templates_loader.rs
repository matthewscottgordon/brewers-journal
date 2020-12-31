use include_dir::{include_dir, Dir};
use itertools::{Either, Itertools};
use tera::Tera;
use warp::Filter;

use std::convert::Infallible;
use std::sync::Arc;

static TERA_TEMPLATE_DIR: Dir = include_dir!("src/templates");

pub fn load_templates() -> impl Filter<Extract = (Arc<Tera>,), Error = Infallible> + Clone
{
    let mut tera = Tera::new("templates/*.html").expect("Tera templates can be loaded");
    let (templates, errors): (Vec<_>, Vec<_>) = TERA_TEMPLATE_DIR
        .files()
        .iter()
        .partition_map(|file| match (file.path().to_str(), file.contents_utf8()) {
            (None, _) => Either::Right(String::from("Filename not UTF-8")),
            (Some(filename), None) => {
                Either::Right(format!("\"{0}\": file contents not UTF-8", filename))
            }
            (Some(filename), Some(file_contents)) => {
                Either::Left((filename, file_contents))
            }
        });
    if !errors.is_empty() {
        eprint!(
            "Error(s) reading {0} of {1} Tera templates:",
            errors.len(),
            TERA_TEMPLATE_DIR.files().len()
        );
        for error in errors {
            eprint!("\t{}", error);
        }
    }
    tera.add_raw_templates(templates)
        .expect("Tera templates can be loaded");
    let tera = Arc::new(tera);

    warp::any().map(move || Arc::clone(&tera))
}
