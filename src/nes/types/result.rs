use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Error with message: {}", msg))]
    WithMsg { msg: String },
}

pub type Result<T> = std::result::Result<T, Error>;
