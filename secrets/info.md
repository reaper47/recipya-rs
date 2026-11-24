# Required files in the `secrets` directory

This folder contains three secret files.

## db_password.txt

This file contains the database user's password without any trailing new lines. 

For example:
```text
my-super-secret-password
```

## db_user.txt

This file contains the database user without any trailing new lines. 

For example:
```text
recipya
```

## master_key.json

This file contains the master key used to encrypt data such as user-provided API keys for different services.

First run the following command to generate a random, base64-encoded 32-byte key :
```bash
head -c 32 /dev/urandom | base64
```

Then, add the following content to the file and replace `{GENERATED_KEY}` with the string above:
```json
{
  "currentVersion": 1,
  "keys": {
    "1": "{GENERATED_KEY}"
  }
}
```
