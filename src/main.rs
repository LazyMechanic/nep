mod app;

use std::env;
use std::process;

use app::App;

fn main() {
    let args: Vec<String> = env::args().collect();
    // if args.len() < 2 {
    //     eprintln!("*.nes file required");
    //     process::exit(1);
    // }

    let mut app = App::new();
    if let Err(err) = app.load_rom(vec![]) {
        println!("{}", err);
    }

    app.run();
}
