use clap::{arg, command, value_parser, Arg, ArgAction, Command};

pub fn i_args() -> clap::ArgMatches {
    command!()
        .about("image processing to blur, brighten, crop, rotate, invert, grayscale, generate and fractal images.")
        .arg(
                Arg::new("effect")
                    .short('e')
                    .long("effect")
                    .aliases(["eff", "e", "effect"])
                    .help("you can specify you desired effect here, to see the avalible effects, run --effect-help")
                    .required(true)
            )
        .arg(
                Arg::new("input")
                    .short('i')
                    .long("input")
                    .aliases(["in", "input"])
                    .help("your image input should be specified here.")
                    .required(true)
            )
        .arg(
                Arg::new("output")
                    .short('o')
                    .long("output")
                    .aliases(["out", "output"])
                    .help("your image output should be specified here.")
                    .required(true)
            )
        .arg(
                Arg::new("effect-help")
                    .long("effect-help")
                    .aliases(["effect-help", "eh"])
                    .help("this will print the available effects")
            )
        .get_matches()
}
