use clap::{ App, Arg };

fn main() {
    let _matches = App::new("echor")
        .version("0.1.0")
        .author("Siva Karasala <siva.karasala@gmail.com>")
        .about("Rust echo")
        .get_matches();
}
