use clap::Parser;

pub fn get_args() -> Args {
    Args::parse()
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, default_value = "./readme.toml")]
    pub config: String,
}
