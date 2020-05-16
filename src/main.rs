mod app;

use std::env;
use std::io::Cursor;
use std::process;

use app::App;

const FILE_POS: usize = 1;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("*.nes file required");
        process::exit(1);
    }

    let file_path = args[1].as_str();

    let mut app = App::new();
    if let Err(err) = app.load_from_file(file_path) {
        eprintln!("{:?}", err);
        process::exit(1);
    }

    app.run();
}
