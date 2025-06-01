# Chai Reaction

## Write your game

The best way to get started is to play around with the code you find in [`src/game/`](./src/game).
This template comes with a basic project structure that you may find useful:

| Path                                               | Description                                                        |
| -------------------------------------------------- | ------------------------------------------------------------------ |
| [`src/main.rs`](./src/main.rs)                     | App entrypoint(not much going on there)                            |
| [`src/loading/`](./src/loading)                    | A high-level way to load collections of asset handles as resources |
| [`src/game/`](./src/game)                          | Game mechanics & content(inputs, scene, player control & animation)|
| [`src/audio.rs`](./src/audio.rs)                   | Marker components for sound effects and music                      |
| [`src/dev_tools.rs`](./src/dev_tools.rs)           | Dev tools for dev builds (press \` aka backtick to toggle)         |
| [`src/screens/`](./src/screens)                    | Splash screen, title screen, gameplay screen, etc.                 |
| [`src/ui/`](./src/ui)                              | Reusable UI widgets & theming                                      |

## Run your game

We recommend using the [Bevy CLI] to run your game.
Running your game locally is very simple:

- Use `bevy run` to run a native dev build.
- Use `bevy run web` to run a web dev build.

<details>
    <summary><ins>Running release builds</ins></summary>

    - Use `bevy run --release` to run a native release build.
- Use `bevy run --release web` to run a web release build.
</details>

<details>
    <summary><ins>Installing Linux dependencies</ins></summary>

  If you're using Linux, make sure you've installed Bevy's [Linux dependencies].
  Note that this template enables Wayland support, which requires additional dependencies as detailed in the link above.
  Wayland is activated by using the `bevy/wayland` feature in the [`Cargo.toml`](./Cargo.toml).
</details>

<details>
    <summary><ins>(Optional) Improving compile times</ins></summary>

[`.cargo/config.toml`](./.cargo/config.toml) contains documentation on how to set up your environment to improve compile times.
</details>

WARNING: if you work in a private repository, please be aware that macOS and Windows runners cost more build minutes.
**For public repositories the workflow runners are free!**

## Release your game

This template uses [GitHub workflows] to run tests and build releases.
Check the [release-flow](.github/workflows/release.yaml)

## Known issues

There are some known issues in Bevy that can require arcane workarounds.

### My audio is stuttering on web

Audio in web-builds can have issues in some browsers.
This seems to be a general performance issue with wasm builds in all engines, the best solution is just to artificially extend loading phase(seems to be a solution people go for in other engines)

- If you're using materials, you should force your render pipelines to [load at the start of the game]
- Optimize your game as much as you can to keep its FPS high.
- Apply the suggestions from the blog post [Workaround for the Choppy Music in Bevy Web Builds].
- Advise your users to try a Chromium-based browser if there are still issues.

### My game window flashes white for a split second when I start the game on Windows

The game window is created before the GPU is ready to render everything.
This means that it'll start with a white screen for a few frames.
The workaround is to [spawn the Window hidden] and only [make it visible a few frames later]

### My character or camera movement is choppy

Choppy character movement is often caused by movement updates being tied to the frame rate.
See the [`physics_in_fixed_timestep`] example for how to fix this.

Choppy camera movement is almost always caused by the camera being tied too tightly to a moving target position.
You can use [`smooth_nudge`] to make your camera smoothly approach its target position instead.


[Bevy CLI]: https://github.com/TheBevyFlock/bevy_cli
[Linux dependencies]: https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md
[GitHub workflows]: https://docs.github.com/en/actions/using-workflows

[Workaround for the Choppy Music in Bevy Web Builds]: https://necrashter.github.io/bevy-choppy-music-workaround
[spawn the Window hidden]: https://github.com/bevyengine/bevy/blob/release-0.14.0/examples/window/window_settings.rs#L29-L32
[make it visible a few frames later]: https://github.com/bevyengine/bevy/blob/release-0.14.0/examples/window/window_settings.rs#L56-L64
[`physics_in_fixed_timestep`]: https://github.com/bevyengine/bevy/blob/main/examples/movement/physics_in_fixed_timestep.rs
[`smooth_nudge`]: https://github.com/bevyengine/bevy/blob/main/examples/movement/smooth_follow.rs#L127-L142
[load at the start of the game]: https://github.com/rparrett/bevy_pipelines_ready/blob/main/src/lib.rs

