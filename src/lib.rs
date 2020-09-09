use dialoguer::{theme::ColorfulTheme, Input, MultiSelect};
use regex::Regex;
use std::{
  process::{Command, Output},
  str,
};

pub enum KeyType {
  Private,
  Public,
}

pub fn decrypt(file: &str) {
  // parse file extension
  let rg = Regex::new(r".*\.\S*(\.asc$|\.gpg$)").unwrap();
  let output_file = match rg.is_match(file) {
    true => String::from(Regex::new(r"\.asc$|\.gpg$").unwrap().replace(file, "")),
    false => Input::new()
      .with_prompt("Please enter the output file")
      .interact()
      .unwrap(),
  };

  // decrypt file
  let mut decrypt_command = Command::new("gpg");
  let decrypt_command_output = decrypt_command
    .arg("--yes")
    .arg("--output")
    .arg(output_file)
    .arg("--decrypt")
    .arg(file)
    .output()
    .expect("Failed to decrypt file");

  handle_command(decrypt_command, decrypt_command_output);
}

pub fn encrypt(file: &str, passphrase_file: Option<&str>) {
  // read gpg keys
  let public_keys = get_gpg_keys(KeyType::Public);
  let private_keys = get_gpg_keys(KeyType::Private);
  let recipient_keys: Vec<String> = public_keys
    .iter()
    .filter(|k| !private_keys.contains(&k))
    .map(|k| String::from(k))
    .collect();

  // prompt for key selection
  let recipients = MultiSelect::with_theme(&ColorfulTheme::default())
    .with_prompt("Pick a recipient(s)")
    .items(&recipient_keys[..])
    .interact()
    .unwrap();
  if recipients.is_empty() {
    panic!("You did not select any recipients");
  }

  let mut encrypt_command = Command::new("gpg");
  encrypt_command
    .arg("--trust-model")
    .arg("always")
    .arg("--yes")
    .arg("--sign");

  // handle passphrase file
  if passphrase_file.is_some() {
    encrypt_command
      .arg("--batch")
      .arg("--passphrase-file")
      .arg(passphrase_file.unwrap());
  }

  // add recipients
  for recipient in recipients {
    encrypt_command
      .arg("--recipient")
      .arg(&recipient_keys[recipient]);
  }

  // encrypt file
  let encrypt_command_output = encrypt_command
    .arg("--encrypt")
    .arg(file)
    .output()
    .expect("Failed to encrypt file");

  handle_command(encrypt_command, encrypt_command_output);
}

pub fn handle_command(mut command: Command, output: Output) {
  if command.status().is_err() {
    panic!("error => {}", String::from_utf8(output.stderr).unwrap());
  }
}

pub fn get_gpg_keys(key_type: KeyType) -> Vec<String> {
  let rg = Regex::new(r"<(((([a-z]|\d|[!#\$%&'\*\+\-/=\?\^_`{\|}~]|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])+(\.([a-z]|\d|[!#\$%&'\*\+\-/=\?\^_`{\|}~]|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])+)*)|((\x22)((((\x20|\x09)*(\x0d\x0a))?(\x20|\x09)+)?(([\x01-\x08\x0b\x0c\x0e-\x1f\x7f]|\x21|[\x23-\x5b]|[\x5d-\x7e]|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])|(\\([\x01-\x09\x0b\x0c\x0d-\x7f]|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF]))))*(((\x20|\x09)*(\x0d\x0a))?(\x20|\x09)+)?(\x22)))@((([a-z]|\d|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])|(([a-z]|\d|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])([a-z]|\d|-|\.|_|~|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])*([a-z]|\d|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])))\.)+(([a-z]|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])|(([a-z]|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])([a-z]|\d|-|\.|_|~|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])*([a-z]|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF]))))>").unwrap();

  let key_flag = match key_type {
    KeyType::Private => "--list-secret-keys",
    KeyType::Public => "--list-public-keys",
  };
  let no_keys = match key_type {
    KeyType::Private => "No private keys found",
    KeyType::Public => "No public keys found",
  };
  let Output { stdout, .. } = Command::new("gpg")
    .arg(key_flag)
    .arg("--with-colons")
    .output()
    .expect("Failed to read keys");
  let mut emails_iterator = rg
    .captures_iter(str::from_utf8(&stdout).unwrap())
    .peekable();
  let mut keys: Vec<String> = vec![];
  if emails_iterator.peek().is_none() {
    panic!(no_keys)
  }
  for matches in emails_iterator {
    let email = matches.get(1).unwrap().as_str();
    keys.push(String::from(email));
  }
  keys
}
