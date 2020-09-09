use clap::clap_app;
use rgpg::{decrypt, encrypt};

fn main() {
  let matches = clap_app!(rgpg =>
		(version: "0.1.0")
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

  match matches.subcommand_name().unwrap() {
    "decrypt" => decrypt(
      matches
        .subcommand_matches("decrypt")
        .unwrap()
        .value_of("file")
        .unwrap(),
    ),
    "encrypt" => {
      let subcommand = matches.subcommand_matches("encrypt").unwrap();
      encrypt(
        subcommand.value_of("file").unwrap(),
        subcommand.value_of("passphrase_file"),
      )
    }
    _ => unreachable!(),
  };
}
