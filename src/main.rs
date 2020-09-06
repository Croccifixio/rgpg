use clap::clap_app;
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use regex::Regex;
use std::{
	path::PathBuf,
	process::{Command, Output},
	str,
};

fn decrypt(file: &str) {
	println!("WIP");
	println!("Decrypting {}", file);
}

fn encrypt(file: &str, passphrase_file: Option<&str>) {
	// REGEX STUFF
	let rg = Regex::new(r"<(((([a-z]|\d|[!#\$%&'\*\+\-/=\?\^_`{\|}~]|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])+(\.([a-z]|\d|[!#\$%&'\*\+\-/=\?\^_`{\|}~]|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])+)*)|((\x22)((((\x20|\x09)*(\x0d\x0a))?(\x20|\x09)+)?(([\x01-\x08\x0b\x0c\x0e-\x1f\x7f]|\x21|[\x23-\x5b]|[\x5d-\x7e]|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])|(\\([\x01-\x09\x0b\x0c\x0d-\x7f]|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF]))))*(((\x20|\x09)*(\x0d\x0a))?(\x20|\x09)+)?(\x22)))@((([a-z]|\d|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])|(([a-z]|\d|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])([a-z]|\d|-|\.|_|~|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])*([a-z]|\d|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])))\.)+(([a-z]|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])|(([a-z]|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])([a-z]|\d|-|\.|_|~|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])*([a-z]|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF]))))>").unwrap();
	let Output {
		stdout: public_keys_stdout,
		..
	} = Command::new("gpg")
		.arg("-k")
		.arg("--with-colons")
		.output()
		.expect("Failed to read public keys");
	let Output {
		stdout: private_keys_stdout,
		..
	} = Command::new("gpg")
		.arg("-K")
		.arg("--with-colons")
		.output()
		.expect("Failed to read private keys");
	let mut public_key_emails_iterator = rg
		.captures_iter(str::from_utf8(&public_keys_stdout).unwrap())
		.peekable();
	let mut private_key_emails_iterator = rg
		.captures_iter(str::from_utf8(&private_keys_stdout).unwrap())
		.peekable();
	let mut public_keys: Vec<&str> = vec![];
	let mut private_keys: Vec<&str> = vec![];
	if public_key_emails_iterator.peek().is_none() {
		panic!("No public keys found")
	}
	if private_key_emails_iterator.peek().is_none() {
		panic!("No private keys found")
	}
	for matches in public_key_emails_iterator {
		let email = matches.get(1).unwrap().as_str();
		public_keys.push(email);
	}
	for matches in private_key_emails_iterator {
		let email = matches.get(1).unwrap().as_str();
		private_keys.push(email);
	}
	println!("{:?}", public_keys);
	println!("{:?}", private_keys);
	let recepient_keys: Vec<&&str> = public_keys
		.iter()
		.filter(|&k| !private_keys.contains(k))
		.collect();
	println!("{:?}", recepient_keys);

	let selections = MultiSelect::with_theme(&ColorfulTheme::default())
		.with_prompt("Pick a recepient(s)")
		.items(&recepient_keys[..])
		.interact()
		.unwrap();

	if selections.is_empty() {
		panic!("You did not select anything :(");
	}

	// ENCRYPT
	let mut encrypt_command = Command::new("gpg");
	encrypt_command
		.arg("-e")
		.arg("-s")
		.arg("-a")
		.arg("--trust-model")
		.arg("always");

	// TODO: handle passphrase file args
	println!("passphrase_file.is_some() {:?}", passphrase_file.is_some());
	println!("passphrase_file.unwrap() {:?}", passphrase_file.unwrap());
	if passphrase_file.is_some() {
		encrypt_command
			.arg("--batch")
			.arg("--passphrase-file")
			.arg(passphrase_file.unwrap());
	}

	for recepient in selections {
		println!("  {}", recepient_keys[recepient]);
		encrypt_command.arg("-r").arg(recepient_keys[recepient]);
	}

	let encrypt_command_output = encrypt_command
		.arg(file)
		.output()
		.expect("Failed to encrypt file");

	if encrypt_command.status().is_err() {
		panic!(
			"error => {}",
			String::from_utf8(encrypt_command_output.stderr).unwrap()
		);
	}
}

fn main() {
	let matches = clap_app!(rgpg =>
		(version: "1.0")
		(author: "Coccifixio")
		(about: "Makes encrypting and decrypting files with gpg more ergonomic")
		(@subcommand encrypt =>
			(about: "Encrypts a file")
			(@arg file: -f --file <FILE> "File to encrypt")
			(@arg passphrase_file: -p --("passphrase-file") [PASSPHRASE_FILE] "File containing the passphrase")
		)
		(@subcommand decrypt =>
			(about: "Decrypts a file")
			(@arg file: -f --file <FILE> "File to decrypt")
		)
	)
	.get_matches();

	let mode = match matches.subcommand_name().unwrap() {
		"decrypt" => decrypt(
			matches
				.subcommand_matches("decrypt")
				.unwrap()
				.value_of("file")
				.unwrap(),
		),
		"encrypt" => {
			let subcmd = matches.subcommand_matches("encrypt").unwrap();
			encrypt(
				subcmd.value_of("file").unwrap(),
				subcmd.value_of("passphrase_file"),
			)
		}
		_ => println!("You broke it"),
	};

	let y = matches
		.subcommand_matches("encrypt")
		.unwrap()
		.value_of("file")
		.unwrap();
	println!("y {:?}", y)
}
