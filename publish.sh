#!/bin/sh

### How to publish the framework
#
# Prerequisites:
# - You need rights to publish on crates.io
# - You need an API access token (you obtain it from https://crates.io/me)
# - You need to call `cargo login <token>` in a console, follow the instructions on crates.io for this.
#
# Steps:
#
# 1. Have a look at commits on GitHub, everything that changed since the last release must be published.
# Be mindful that hotfixes need to be backwards compatible, minor releases do not.
# We always publish all `elrond-wasm-*` crates together.
# We always publish `elrond-codec` and `elrond-codec-derive` together.
# `elrond-wasm-*` depend on both `elrond-codec` and `mandos`, so if you have a minor release on the latter,
# you also need a minor release on `elrond-wasm-*`.
#
# 2. Mass replace previous version -> new version (elrond-wasm, elrond-codec, mandos - different numbers).
# Be careful to not accidentally replace some of the other dependencies we have.
#
# 3. Write release name, date and description in `CHANGELOG.md`.
#
# 4. Run `cargo test`, to make sure nothing was broken and all dependencies still work fine.
#
# 5. Commit changes. The name of the commit should be the released crates and versions, same as the changelog title,
# e.g. `elrond-wasm 0.21.1, elrond-codec 0.8.1, mandos 0.11.1`.
# The branch doesn't need to be published for the following steps to work.
# 
# 5. Run this script, `./publish.sh`.
# You can comment out the crates you are not publishing. Nothing happens if you do not,
# you will just receive a message that they are already published.
# 
# 6. Search for `elrond` on `crates.io` and check that the new versions appear for all crates.
# If any of the crates was not published, check what went wrong and try again.
#
# 7. Create tag.
# `git tag -s -a vX.X.X -m 'very short description of the release'`
# `git push origin vX.X.X`
#
# 8. Go to https://github.com/ElrondNetwork/elrond-wasm-rs/tags
# Click on the new tag.
# Click `Create release from tag`.
# The title should be the released crates and versions, same as in the changelog and the commit message.
# The description should be copied from CHANGELOG.md, as is.
#
# 9. Create pull request on GitHub. The faster it gets merged in master, the better.
#
# 10. (optional) Test the new framework on one of the contracts that are not in the same repo, e.g. DNS, DEX, etc.
#
# 11. (optional) Announce on Telegram.
# Skip this step if you feel the new release is a bit too experimental, or if it doesn't work with the latest VM.
#

cd elrond-codec-derive
cargo publish
cd ..

sleep 20

cd elrond-codec
cargo publish
cd ..

cd mandos
cargo publish
cd ..

sleep 20

cd elrond-wasm-derive
cargo publish
cd ..

sleep 20

cd elrond-wasm
cargo publish
cd ..

sleep 20

cd elrond-wasm-node
cargo publish
cd ..

cd elrond-wasm-debug
cargo publish
cd ..

cd elrond-wasm-output
cargo publish
cd ..

cd elrond-wasm-modules
cargo publish
cd ..
