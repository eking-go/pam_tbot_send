# pam_tbot_send

Sends information about logins in system to telegram chat (uses PAM)

# Installation

Install Rust + cargo and build the binary:
```
cargo build --release
```

And copy one to the `/usr/local/bin/pam_tbot_send` (for example)
```
cp target/release/pam_tbot_send /usr/local/bin/pam_tbot_send
```

then create file (default location, you can change it with `-c` option) `/etc/pam_tbot_send.json`:
```
{
  "type": "Telegram",
  "conf": {"token": "ewfn_token", "chat": "2432154"}
}
```

add next line to the pam configuration (for example in the file `/etc/pam.d/common-session`):
```
session optional pam_exec.so debug /usr/local/bin/pam_tbot_send
```
