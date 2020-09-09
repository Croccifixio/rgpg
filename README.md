# rgpg

This is a CLI tool to make decrypting and encrypting files using `gpg` more ergonomic.


## Prerequisites

For `rgpg` to be useful, it requires the following criteria to be met:
- A secret key has already been generated/imported.
- At least one public key, which is assigned to a different email than the default public key, has been imported.

## Usage

**Encrypting a file**

```
rgpg encrypt -f filename.ext
```

Running the above command will open a prompt that can be used to pick the recipients of the encrypted file. The encrypted file will be signed using the default secret key.

**Decrypting a file**

```
rgpg decrypt -f filename.ext.gpg
```

Running the above command will decrypt the file, saving the decrypted version to `filename.ext` in the working directory. If `rgpg` cannot deduce the name of the output file to save to from the encrypted file's name, it will open a prompt asking for the output file's name to be specified.

**Printing help information**

```
rgpg -h
rgpg encrypt -h
rgpg decrypt -h
```
