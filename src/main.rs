use std::env;

mod day1;
mod day2;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    let module = args.get(1).map_or_else(
        || {
            println!("No module name specified, defaulting to day1.");
            "day1"
        },
        |module| module,
    );

    match module {
        "day1" => day1::main(),
        "day2" => day2::main(),
        _ => println!("Invalid module name."),
    }
}
