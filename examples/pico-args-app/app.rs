const HELP: &str = "\
App

USAGE:
  app [OPTIONS] --number NUMBER [INPUT]..

FLAGS:
  -h, --help            Prints help information

OPTIONS:
  --number NUMBER       Sets a number
  --opt-number NUMBER   Sets an optional number
  --width WIDTH         Sets width [default: 10]

ARGS:
  <INPUT>
";

#[derive(Debug)]
struct AppArgs {
    number: u32,
    opt_number: Option<u32>,
    width: u32,
    input: Vec<std::path::PathBuf>,
}

fn parse_width(s: &str) -> Result<u32, String> {
    let w = s.parse().map_err(|_| "not a number")?;
    if w != 0 {
        Ok(w)
    } else {
        Err("width must be positive".to_string())
    }
}

fn parse_path(s: &std::ffi::OsStr) -> Result<std::path::PathBuf, pico_args::Error> {
    Ok(std::path::PathBuf::from(s))
}

fn main() {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    #[cfg(debug_assertions)]
    {
        println!("{:#?}", args.number);
        println!("{:#?}", args.opt_number);
        println!("{:#?}", args.width);
        if 10 < args.input.len() {
            println!("{:#?}", args.input.len());
        } else {
            println!("{:#?}", args);
        }
    }
    std::hint::black_box(args);
}

fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    let mut args = AppArgs {
        number: pargs.value_from_str("--number")?,
        opt_number: pargs.opt_value_from_str("--opt-number")?,
        width: pargs
            .opt_value_from_fn("--width", parse_width)?
            .unwrap_or(10),
        input: Vec::new(),
    };

    while let Some(value) = pargs.opt_free_from_os_str(parse_path)? {
        args.input.push(value);
    }

    Ok(args)
}
