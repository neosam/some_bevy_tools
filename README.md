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
* Loop music on specific positions and change the loop position while the music is playing
* Third party camera and controller
* 2D Camera and simple top down character controller

Additionally, I try to document each module with at least one example. This should ensure that
there are no accidential breaking changes.

## Version history and Bevy version
| Version | Bevy |
| ------- | ---- |
| 0.1     | 0.13 |
| 0.2     | 0.13 |

## Usage
Make sure to use these lines in your Cargo.toml
```toml
[dependencies]
bevy = "0.13.1"
some_bevy_tools = "0.2.1"
```

## Features
By default, all features are enabled to get you started quickly.  To optimize the build, disable
the default features and only use which is requied.

The core features of this crate are:

| Feature      | Description                                                    | Bevy features          |
| ------------ | -------------------------------------------------------------- | ---------------------- |
| audio_loop   | Adds support for looping inside of audio files (usually music) | bevy_audio, bevy_asset |
| loading      | Load assets into resources using reflect.                      | bevy_asset             |
| split_screen | Enables split screen support using two cameras.                | bevy_render            |
| sbs_3d       | Allow 3D output using SBS (side-by-side) rendering.            | bevy_render            |

These features add bevy_rapier as dependency:

| Feature       | Description                                                   | Additional dependency |
| ------------- | ------------------------------------------------------------- | --------------------- |
| bevy_rapier2d | Simplify collision events                                     | bevy_rapier2d         |
| bevy_rapier3d | Simplify collision events                                     | bevy_rapier3d         |

These are just features which enable a bunch of bevy features required to do usual stuff to get started quickly
but do not enable all of the bevy features.

| Feature         | Description                                                  | Bevy features                                                                                                          |
| --------------- | ------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------- |
| audio_deluxe    | Adds audio and audio file formats                            | bevy_audio, vorbis, bevy_asset                                                                                         |
| display_minimum | Only what is required to display a window and process events | x11, bevy_winit                                                                                                        |
| display_deluxe  | Enable features which allow to draw sprites or 3D objects    | x11, bevy_winit, bevy_asset, bevy_render, bevy_sprite, png, bevy_pbr, tonemapping_luts                                 |
| all             | Enable everything except for rapier                          | bevy_audio, vorbis, bevy_asset, x11, bevy_winit, bevy_asset, bevy_render, bevy_sprite, png, bevy_pbr, tonemapping_luts |


## CI (copied from the Bevy starter template)

Definition: [.github/workflows/ci.yaml](./.github/workflows/ci.yaml)

This workflow runs on every commit to `main` branch, and on every PR targeting the `main` branch.

It will use rust stable on linux, with cache between different executions, those commands:

* `cargo test`
* `cargo clippy -- -D warnings`
* `cargo fmt --all -- --check`

If you are using anything OS specific or rust nightly, you should update the file [ci.yaml](./.github/workflows/ci.yaml) to use those.

## Code License

* Apache License, Version 2.0
   ([LICENSE-APACHE-2.0](LICENSE-Apache-2.0) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT License
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
* CC0-1.0 License
   ([LICENSE-CC0-1.0](LICENSE-CC0-1.0) or <https://creativecommons.org/publicdomain/zero/1.0/legalcode>)

## Media License
* The Ducky sprite is CC-0 licensed by [Caz Creates Games](https://caz-creates-games.itch.io/ducky-2).
* The eeh-eeh song is [CC-BY 4.0] (https://creativecommons.org/licenses/by/4.0/) licensed by [neosam](https://github.com/neosam).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
triple licensed as above, without any additional terms or conditions.
