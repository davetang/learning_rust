#!/usr/bin/env bash

set -euo pipefail

lang=rust
tool=biodemo
name="Dave Tang"
email="me@davetang.org"

curl -sSf https://raw.githubusercontent.com/bionitio-team/bionitio/master/boot/bionitio-boot.sh \
   | bash -s -- -i ${lang} -n ${tool} -a ${name} -e ${email}
