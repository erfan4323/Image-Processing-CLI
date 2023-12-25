#[cfg(test)]
mod tests {
    use crate::i_args;
    use clap::{arg, command, value_parser, ArgAction, ArgMatches, Command};

    #[test]
    fn args_test() {
        let matches: ArgMatches = i_args();

        let effect = matches.get_one::<String>("effect").unwrap().as_str();
        match effect {
            "blur" => println!("blur"),
            "brighten" => println!("brighten"),
            "crop" => println!("crop"),
            "rotate" => println!("rotate"),
            "invert" => println!("invert"),
            "graysclae" => println!("grayscale"),
            "generate" => println!("generate"),
            "fractal" => println!("fractal"),
            _ => println!("Please get some help"),
        }
    }
}
