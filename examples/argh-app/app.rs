use argh::FromArgs;

/// App
#[derive(Debug, FromArgs)]
struct AppArgs {
    /// sets number
    #[argh(option)]
    number: u32,

    /// sets optional number
    #[argh(option)]
    opt_number: Option<u32>,

    /// sets width [default: 10]
    #[argh(option, default = "10", from_str_fn(parse_width))]
    width: u32,

    /// input
    #[argh(positional)]
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

fn main() {
    let args: AppArgs = argh::from_env();
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
