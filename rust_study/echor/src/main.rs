use clap::{App, Arg};

fn main() {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("Yuki <test@example.com>")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();

        let text = matches.values_of_lossy("text").unwrap();
        let omit_newline = matchs.is_present("omit_newline");

        let ending = "\n";
        if omit_newline {
            ending = "";
        }
        print!("{}{}", text.join(" "), ending);
    
    println!("{:#?}", matches);
}
