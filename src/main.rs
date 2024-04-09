use clap::Parser;
use clap_stdin::MaybeStdin;
use url::form_urlencoded::parse;

#[derive(Debug, Parser)]
struct Args {
    //#[clap(default_value = "-")]
    value: MaybeStdin<String>,
}

fn main() {
    let args = Args::parse();
    let decoded: String = parse(args.value.as_bytes())
        .map(|(key, val)| [key, val].concat())
        .collect();
    println!("{}", decoded);
}






