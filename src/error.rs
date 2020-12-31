use mobc_postgres::tokio_postgres;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Tera template error {0}")]
    TeraTemplateError(tera::Error),

    #[error("error getting connection from DB pool: {0}")]
    DBPoolError(mobc::Error<tokio_postgres::Error>),

    #[error("error executing DB query: {0}")]
    DBQueryError(#[from] tokio_postgres::Error),

    #[error("error reading file: {0}")]
    ReadFileError(#[from] std::io::Error),
}

impl warp::reject::Reject for Error {}

impl From<Error> for warp::reject::Rejection {
    fn from(e: Error) -> warp::reject::Rejection {
        warp::reject::custom(e)
    }
}

impl From<tera::Error> for Error {
    fn from(e: tera::Error) -> Error {
        Error::TeraTemplateError(e)
    }
}

impl From<mobc::Error<tokio_postgres::Error>> for Error {
    fn from(e: mobc::Error<tokio_postgres::Error>) -> Error {
        Error::DBPoolError(e)
    }
}
