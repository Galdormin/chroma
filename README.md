# Chroma

A small 2D exploration platformer made with [Bevy](https://bevyengine.org/), where your color decides what is solid and what is not.

**Play it on itch.io: <https://galdormin.itch.io/chroma>**

> [!NOTE]
> Chroma is an unfinished prototype and development is currently paused.
> Expect missing content and rough edges.

## The game

Collectibles are hidden all over the map, and your goal is to find them all.

The twist: you collide with everything that is **not** your color.
Anything that shares your color lets you walk right through it, so a wall in one color is an open path in another.
Pick up colored droplets to change your color and reach new parts of the map.

## Controls

| Action       | Key              |
| ------------ | ---------------- |
| Move left    | `A`              |
| Move right   | `D`              |
| Jump         | `Space`          |
| Pause / Back | `P` or `Escape`  |

Keys are bound by physical position, so on an AZERTY keyboard, move left is `Q`.

## Running from source

You need a [Rust toolchain](https://www.rust-lang.org/tools/install) and the [Bevy dependencies](https://bevyengine.org/learn/quick-start/getting-started/setup/#installing-os-dependencies) for your OS.

```sh
cargo run
```

With the [Bevy CLI](https://github.com/TheBevyFlock/bevy_cli), you can also run the web build:

```sh
bevy run web
```

## Credits

- Code, design and art by Galdormin
- Button SFX by Jaszunio15 (CC0)
- Music: "Exploration" by Ansimuz
- Font: [monogram](https://datagoblin.itch.io/monogram) by datagoblin (CC0)
- Built with [Bevy](https://bevyengine.org/), [Avian](https://github.com/Jondolf/avian), [bevy_ecs_ldtk](https://github.com/Trouv/bevy_ecs_ldtk) and the [Bevy New 2D](https://github.com/TheBevyFlock/bevy_new_2d) template
