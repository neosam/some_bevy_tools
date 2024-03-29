# Some bevy tools

This repo contains extensions for the great [Bevy Engine](https://bevyengine.org/). My goal is to have a crate which
provides ECS stuff I regularly use in projects to save me time in future Bevy Game Jams.  This crate tries to
make the usage as simple as possible so the developer can focus on the main content of the game.

Currently supported features are:

* Automatic despawn after a period of time.
* Automatic despawn of components on a state change.
* Range component which keeps its value between a min and a max value and writes events
  if min or max was reached.  For example it can be used for health to detect death.
* Simplified processing of events on collisions in rapier.
* Mapping of user inputs to custom events. (currently only keyboard events are supported for now)
* Loading of assets on a loading state and storing them automatically in a resource using reflect.
* Split screen support.
* SBS support. It is basically a split screen which allows a sterioscopic view by using special
  hardware like XReal or Virture glasses.

Additionally, I try to document each module with at least one example. This should ensure that
there are no accidential breaking changes.

## Version history and Bevy version
| Version | Bevy |
| ------- | ---- |
| 0.1     | 0.13 |

## CI (copied from the Bevy starter template)

Definition: [.github/workflows/ci.yaml](./.github/workflows/ci.yaml)

This workflow runs on every commit to `main` branch, and on every PR targeting the `main` branch.

It will use rust stable on linux, with cache between different executions, those commands:

* `cargo test`
* `cargo clippy -- -D warnings`
* `cargo fmt --all -- --check`

If you are using anything OS specific or rust nightly, you should update the file [ci.yaml](./.github/workflows/ci.yaml) to use those.

## License

* Apache License, Version 2.0
   ([LICENSE-APACHE-2.0](LICENSE-Apache-2.0) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT License
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
* CC0-1.0 License
   ([LICENSE-CC0-1.0](LICENSE-CC0-1.0) or <https://creativecommons.org/publicdomain/zero/1.0/legalcode>)

The Ducky sprite is CC-0 licensed by [Caz Creates Games](https://caz-creates-games.itch.io/ducky-2).
The eeh-eeh song is [CC-BY 4.0] (https://creativecommons.org/licenses/by/4.0/) licensed by [neosam](https://github.com/neosam).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
triple licensed as above, without any additional terms or conditions.
