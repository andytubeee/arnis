# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

Arnis turns OpenStreetMap and elevation data into Minecraft Java, Bedrock, Luanti and Hytale worlds. It is one Rust crate that builds both a Tauri desktop GUI (default `gui` feature) and a CLI (`--no-default-features`).

## This checkout is a fork

- `origin` is upstream louis-e/arnis, and it refuses pushes. `fork` is andytubeee/arnis. Work on the `supply-crates` branch and push it to the fork's main with `git push fork supply-crates:main`. Local `main` is upstream's code, so don't work on it.
- Fork releases are tagged `v<upstream latest>-andytubeee`, for example `v3.5.0-andytubeee`. Never invent a version upstream hasn't released.
- Build releases only with `arnis-build` (`~/.local/bin`), never with Docker or GitHub Actions. `arnis-build --release` also moves the tag and replaces the release assets. It's fast only while compiled dependencies are cached in `target/`.

## Commands

```bash
cargo run --release                          # GUI
cargo run --release --no-default-features -- --output-dir=<saves dir> --bbox="min_lat,min_lng,max_lat,max_lng"   # CLI
cargo test --release                         # whole suite (~3k tests, under a minute once built)
cargo test --release <name_substring>        # one test, or every test whose path contains it
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt -- --check
```

- **CI** (`.github/workflows/ci-build.yml`) runs fmt, clippy with `-D warnings`, then build and test with `--all-targets --all-features`.
- **Network tests:** tests under `tests/` that need the network are `#[ignore]`. Run them explicitly, for example `cargo test --release --no-default-features --test one_world_seam -- --ignored`.
- **GUI tests:** the frontend (`src/gui/`, plain JS with no bundler, served by Tauri from `frontendDist: src/gui`) has Node tests that read the HTML, CSS and JS directly. Run them one at a time with `node tests/gui/<name>.mjs`, or all of them with `for f in tests/gui/*.mjs; do node "$f" || echo "FAIL $f"; done`.

## Architecture

**One pipeline, two front ends.** `main.rs` (`run_cli` → `run_generation_cli`) and `gui.rs` both drive the same pipeline, so they duplicate its setup. Comments marked `Same as run_cli` in `gui.rs` flag the places that must stay in step: a change to argument handling, size checks or One World preparation in one usually needs the matching change in the other. `args.rs` (clap `Args`) is the single source of options. The GUI fills an `Args` from its form, then calls `apply_body_defaults` and `apply_feature_needs`.

**Pipeline stages:**
1. **Fetch, in parallel threads** (`main.rs`, `std::thread::scope`):
   - OSM from Overpass (`retrieve_data.rs`), the PMTiles tile archive (`osm_tiles.rs`) or a local file or PBF
   - Overture buildings (`overture/`)
   - elevation and land cover in `ground::generate_ground_data` (`elevation/`, `land_cover/`)
2. **Parse:** `osm_parser.rs` turns raw OSM into `ProcessedElement`s, projected into block coordinates (`coordinate_system/`, `projection/`). `map_transformation/` can rewrite the element set.
3. **Generate:** `data_processing::generate_world_with_options` cuts the area into 512-block tiles aligned to regions (`tile.rs`). It runs `element_processing/` per tile in parallel on rayon under a RAM budget, placing roads, buildings, water, trees and so on through `WorldEditor`.
4. **Write:** `world_editor/` holds block state and writes it per format (`java.rs` Anvil, `bedrock.rs`, `luanti.rs`, `hytale.rs`). It can stream finished regions to disk.
5. **Extras:**
   - `datapack/`: one Java data pack for the gameplay modes (City Stories, Crime Spree, discoveries, disasters, dialogs). Its base layer targets 1.21.4, with overlays per pack format.
   - `distant_horizons/` and `voxy/` write LOD data.

**Terrain height** lives in `ground.rs` and `elevation/postprocess.rs`. `ElevationAffine` maps metres to Y. Tall terrain uses a soft top and bottom (asinh bends) instead of clipping. A world records its mapping in a manifest, so an area built later lines up with the earlier ones.

**One World** (`one_world.rs`) is a persistent Java world that every run extends:
- `arnis_one_world.json`, next to `level.dat`, pins a Web Mercator origin and scale, the elevation mapping and the list of areas.
- `MANIFEST_VERSION` and `written_version()` gate which builds can open a world. Bump them when the format changes meaning, so older builds refuse the world instead of misreading it.
- Large selections are cut into region-group pieces by `work_units.rs`.

**Live mode** (`live/`) runs a One World on a local vanilla server (`server.rs`, `mojang.rs` for downloads, `rcon.rs`) and generates land around players as they move:
- **Tiles:** each tile is a child `arnis --one-world --live-tile` process (`tiles.rs`), merged into the region files (`region.rs`).
- **Game zone:** when a game mode is on, a new world first builds the zone around the start as an ordinary One World run (`zone.rs`), so the data pack has a street graph to plan on.
- **Server extras:** LAN in `lan.rs`, and the resource pack served over HTTP in `pack.rs`.
- **Child progress:** child runs report progress as NDJSON (`--progress json`, `progress_json.rs`).

## Conventions that bite

- `src/gui/locales/en.json` is a symlink to `en-US.json`. When adding a locale key across `src/gui/locales/*.json`, skip the symlink or the key lands twice.
- Every Settings control in `index.html` must be registered in `src/gui/js/settings-store.js` `SETTINGS`, or it isn't saved between launches (`tests/gui/settings-registered.mjs` checks selects).
- Setting headings put the tooltip `<span>` right after the name with no whitespace between them (`tests/gui/setting-heading.mjs`).
- `mod/` is a separate Gradle project for the client modpack (`mod/modpack/modrinth.index.json`), not part of the cargo build.
