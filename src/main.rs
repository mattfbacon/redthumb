use clap::Parser;

mod image;
mod net;

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
	post_id: String,
	#[clap(long, default_value = "sans")]
	font: String,
}

pub fn main() -> anyhow::Result<()> {
	let Args { post_id, font } = Args::parse();
	if atty::is(atty::Stream::Stdout) {
		eprintln!("This utility writes PNG data to stdout. You probably wanted to redirect the output to a file. We'll wait a few seconds for you to terminate the process before trashing your terminal.");
		std::thread::sleep(std::time::Duration::from_secs(10));
	}
	net::generate(&font, &post_id, &mut std::io::stdout().lock())
}
