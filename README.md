# Learning Rust

Because why not!?

## Docker

Use the [official Docker image](https://hub.docker.com/_/rust/).

```bash
docker pull rust:1.64.0

docker run --rm rust:1.64.0 cat /etc/os-release
PRETTY_NAME="Debian GNU/Linux 11 (bullseye)"
NAME="Debian GNU/Linux"
VERSION_ID="11"
VERSION="11 (bullseye)"
VERSION_CODENAME=bullseye
ID=debian
HOME_URL="https://www.debian.org/"
SUPPORT_URL="https://www.debian.org/support"
BUG_REPORT_URL="https://bugs.debian.org/"
```

Use `script/start_container.sh` to start and restart a container using
rust:1.64.0 called `learning_rust`.

## Notes

* Cargo is Rust's build tool, package manager, and test runner.

