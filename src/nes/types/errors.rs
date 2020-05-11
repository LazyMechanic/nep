use snafu::Backtrace;
use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Error with message: {}", msg))]
    WithMsg {
        backtrace: Backtrace,
        msg:       String,
    },
    #[snafu(display("Error during read cartridge: {}", detail))]
    ReadCartridge {
        backtrace: Backtrace,
        detail:    String,
    },
    #[snafu(display("Error during read file: {}", source))]
    ReadFile {
        backtrace: Backtrace,
        source:    std::io::Error,
    },
    #[snafu(display("Error during open file: {}", source))]
    OpenFile {
        backtrace: Backtrace,
        source:    std::io::Error,
    },
}
