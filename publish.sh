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
# We always publish all `framework/*` crates together.
# We always publish `multiversx-codec` and `multiversx-codec-derive` together.
# `framework/*` depend on both `multiversx-codec` and `multiversx-chain-scenario-format`,
# so if you have a minor release on the latter, you also need a minor release on `framework/*`.
# See the Changelog for more details.
#
# 2. Mass replace previous version -> new version.
# Be careful to not accidentally replace some of the other dependencies we have.
#
# 3. Write release name, date and description in `CHANGELOG.md`.
#
# 4. Run `cargo test`, to make sure nothing was broken and all dependencies still work fine.
#
# 5. Commit changes. The name of the commit should be the released crates and versions, same as the changelog title,
# e.g. `sc 0.39.0, codec 0.17.0, chain-vm 0.1.0, chain-scenario-format 0.19.0, sdk 0.1.0`.
# The branch doesn't need to be published for the following steps to work.
#
# 6. Make sure that the contract upgrade tool is still sound.
# At the very least add the new version to `VERSIONS` and change `DEFAULT_LAST_VERSION` in 
# `/home/andreim/multiversx/rs/mx-sdk-rs/framework/meta/src/sc_upgrade/upgrade_versions.rs`
# 
# 7. Run this script, `./publish.sh`.
# You can comment out the crates you are not publishing. The script will stop otherwise when it cannot publish them.
# 
# 8. Search for `multiversx` on `crates.io` and check that the new versions appear for all crates.
# If any of the crates was not published, check what went wrong and try again.
#
# 9. Create tag.
# `git tag -s -a vX.X.X -m 'very short description of the release'`
# `git push origin vX.X.X`
#
# 10. Go to https://github.com/multiversx/mx-sdk-rs/tags
# Click on the new tag.
# Click `Create release from tag`.
# The title should be the released crates and versions, same as in the changelog and the commit message.
# The description should be copied from CHANGELOG.md, as is.
#
# 11. Run `sc-meta all update`. This will update the `Cargo.lock` files.
#
# 12. Create pull request on GitHub. The faster it gets merged in master, the better.
#
# 13. (optional) Test the new framework on one of the contracts that are not in the same repo, e.g. DNS, DEX, etc.
#
# 14. Post in Slack to `release-announcements`.
#
# 15. Write a release announcement in Confluence.
#

cd vm
cargo publish || return 1
cd ..

cd sdk/core
cargo publish || return 1
cd ../..

cd sdk/scenario-format/
cargo publish || return 1
cd ../..

cd data/codec-derive
cargo publish || return 1
cd ../..

cd data/codec
cargo publish || return 1
cd ../..

cd framework/derive
cargo publish || return 1
cd ../..

cd framework/base
cargo publish || return 1
cd ../..

cd framework/meta
cargo publish || return 1
cd ../..

cd framework/scenario
cargo publish || return 1
cd ../..

cd framework/snippets
cargo publish || return 1
cd ../..

cd framework/wasm-adapter
cargo publish || return 1
cd ../..

cd contracts/modules
cargo publish || return 1
cd ../..

cd contracts/core/price-aggregator
cargo publish || return 1
cd ../../..

cd contracts/core/wegld-swap
cargo publish || return 1
cd ../../..
