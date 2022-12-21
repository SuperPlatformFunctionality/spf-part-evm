//! Generate an Ethereum account.

use clap::Parser;
use cli_opt::account_key::GenerateAccountKey;

#[derive(Debug, Parser)]
#[clap(author = "SPF Dev")]
struct Opt {
	#[clap(flatten)]
	cmd: GenerateAccountKey,
}

impl Opt {
	fn run(&self) {
		self.cmd.run()
	}
}

fn main() {
	// Parses the options
	let opt = Opt::parse();
	opt.run();
}
