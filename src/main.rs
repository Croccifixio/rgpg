use dialoguer::{theme::ColorfulTheme, MultiSelect};
use iter_set::difference;
use regex::Regex;
use std::process::{Command, Output};
use std::str;

fn main() {
	//  let multiselected = &[
	//      "Ice Cream",
	//      "Vanilla Cupcake",
	//      "Chocolate Muffin",
	//      "A Pile of sweet, sweet mustard",
	//  ];
	//  let defaults = &[false, false, true, false];
	//  let selections = MultiSelect::with_theme(&ColorfulTheme::default())
	//      .with_prompt("Pick your food")
	//      .items(&multiselected[..])
	//      .defaults(&defaults[..])
	//      .interact()
	//      .unwrap();

	//  if selections.is_empty() {
	//      println!("You did not select anything :(");
	//  } else {
	//      println!("You selected these things:");
	//      for selection in selections {
	//          println!("  {}", multiselected[selection]);
	//      }
	//  }

	// REGEX STUFF
	let rg = Regex::new(r"<(((([a-z]|\d|[!#\$%&'\*\+\-/=\?\^_`{\|}~]|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])+(\.([a-z]|\d|[!#\$%&'\*\+\-/=\?\^_`{\|}~]|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])+)*)|((\x22)((((\x20|\x09)*(\x0d\x0a))?(\x20|\x09)+)?(([\x01-\x08\x0b\x0c\x0e-\x1f\x7f]|\x21|[\x23-\x5b]|[\x5d-\x7e]|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])|(\\([\x01-\x09\x0b\x0c\x0d-\x7f]|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF]))))*(((\x20|\x09)*(\x0d\x0a))?(\x20|\x09)+)?(\x22)))@((([a-z]|\d|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])|(([a-z]|\d|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])([a-z]|\d|-|\.|_|~|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])*([a-z]|\d|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])))\.)+(([a-z]|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])|(([a-z]|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])([a-z]|\d|-|\.|_|~|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])*([a-z]|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF]))))>").unwrap();
	//  let mut gpg = Command::new("gpg");
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
		println!("You did not select anything :(");
	} else {
		println!("You selected these things:");
		for selection in selections {
			println!("  {}", recepient_keys[selection]);
		}
	}
}
