use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config_result = minigrep::Config::new(&args);

    let config = match config_result {
        Ok(config) => config,
        Err(err) => {
            println!("Problem parsing arguments: {}", err);
            process::exit(1);
        }
    };

    if let Err(err) = minigrep::run(config) {
        println!("Application Failed {}", err);
        process::exit(1);
    }
}
