# City Stories + NPC Interaction Menu Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** A chill co-op story mode whose storyline and places are picked at generation from the real
map, plus a right-click menu on every pedestrian that both City Stories and Crime Spree use.

**Architecture:** Rust picks the story places, rolls one of 15 storylines and writes its steps and
advancements into the data pack (like Discoveries and Disasters). Static `ov80` mcfunctions run the
steps, errands, finales and the menu. Each walker carries an `interaction` passenger that catches the
right-click, which opens an inline dialog built by a macro.

**Tech Stack:** Rust (serde_json, rand 0.9 + rand_chacha), Minecraft Java data pack overlay `ov80`
(1.21.6+ dialogs and SNBT text), Tauri GUI (HTML/JS, locales).

**Spec:** `docs/superpowers/specs/2026-11-14-city-stories-design.md`

## Global Constraints

- Java only; all runtime behaviour in overlay `ov80`; base layer gets stubs (any function a base
  tag or base function calls must exist in base) and prints the "unsupported" line like Crime Spree.
- City Stories and Crime Spree are independent: each works alone and both together. Neither turns
  the other off; shared code (menu, walkers) must branch on what is on, never assume one mode.
- `--city-stories` forces `pause_menu`, `pedestrians`, `moving_vehicles` to true (overwrites the GUI).
- Off-Earth reset clears `city_stories`; Bedrock rejects it (Java-only extras list).
- Mode dropped with a warning when there are no footways (no pedestrians) or fewer than 3 named
  story places.
- Never stuck, never rushed: no timers that fail, no required items, progress kept on death.
- No spoilers: GUI and pause menu never name the storyline before its finale.
- Shared progress lives in scoreboard fake players / `arnis:story` storage, never on a player.
- Per-second loops, not per-tick, except handling a click or trigger.
- Every new static file is listed in the `STORIES`/`NPC` arrays in `src/datapack/mod.rs`; every
  locale gets every new key (test `locales_match_en_us_keys`).
- Ponytail: shortest robust code, comments only for the why, no speculative options.
- Run `/code-review high --fix` at the end of each stage (after Tasks 3, 6, 9, 11) before its commit.

## Plan deviations from the spec (builder's calls)

- Menu icons use Unicode symbols the vanilla font already draws (☺ ✎ ⚑ $ ✖ ✉ ★) instead of
  resource-pack glyphs: same look in a grid, no new pack files, works without the resource pack.
- Rob is never offered on story NPCs (keeps the story safe when both modes are on).

## Review Focus

1. Both modes on: a pedestrian's menu shows Say hi, Chat, Ask directions, Rob, Never mind; a story
   NPC's shows Talk and no Rob; Crime alone shows no Talk; Stories alone shows no Rob.
2. A click on a car and a walker share `player_interacted_with_entity`: boarding a car must still work
   and must not open the menu; clicking a walker must not board anything.
3. The target walker despawns (or walks off past 4 blocks) while the menu is open: the button does
   nothing but say they left; it never robs or talks to another walker.
4. Two players open the menu on the same walker: each acts on it once; a second Rob is refused
   (already robbed).
5. A player joins after steps were done (or was offline): they get every story advancement so far on
   their first second, and the pause page shows the same step as everyone.

---

### Task 1: In-game test harness (needs the user's OK to download the server jar)

**Files:**
- Create: `scripts/mc-check.sh` (gitignored work dir `.mc-check/`)
- Modify: `.gitignore`

**Interfaces:**
- Produces: `scripts/mc-check.sh <world-dir>`: copies the world into `.mc-check/`, starts the
  official vanilla server jar (version from `MC_VERSION`, default the newest the pack supports,
  downloaded once from Mojang's piston-meta manifest) headless with RCON on, runs `reload`, then
  each line of `stdin` as an RCON command, prints the server log lines matching
  `ERROR|Failed to load function|Unknown or incomplete command|Couldn't`, and exits non-zero if any.

- [ ] **Step 1:** Write the script (uses `java`, `curl`, `python3` for RCON; no new dependencies).
- [ ] **Step 2:** Generate a small city world with `--crime-spree` from the release build and run
  `echo "function arnis:crime/load" | scripts/mc-check.sh <world>`. Expected: exit 0, no errors.
- [ ] **Step 3:** Commit `scripts/mc-check.sh`, `.gitignore`.

### Task 2: The `--city-stories` option end to end (args, GUI, Extras)

**Files:**
- Modify: `src/args.rs` (struct ~437, `apply_feature_needs` ~1531, off-Earth ~1589, Java-only ~1781, tests ~2384)
- Modify: `src/gui.rs` (~2381 argv, ~2725 param, ~3155 Args literal)
- Modify: `src/gui/index.html` (beside `crime-spree-toggle` ~1103), `src/gui/js/main.js` (~2188 gate, ~2211 reasons, ~2232 list, ~3757 reason, ~4952 payload `cityStories`), `src/gui/js/settings-store.js` (~61), every file in `src/gui/locales/`
- Modify: `src/datapack/mod.rs` (`Extras.stories`, `from_args`, `any`, `wants_places` → true when stories)

**Interfaces:**
- Produces: `Args.city_stories: bool` (`--city-stories`); `Extras.stories: bool`.

- [ ] **Step 1: Failing test** `city_stories_turns_on_what_it_needs` in `src/args.rs` tests:
```rust
let s = parse(&["--city-stories"]);
assert!(s.pedestrians && s.moving_vehicles && s.pause_menu);
assert!(validate_args(&s).is_ok());
let both = parse(&["--city-stories", "--crime-spree"]);
assert!(both.city_stories && both.crime_spree && validate_args(&both).is_ok());
let moon = parse(&["--city-stories", "--body", "moon"]);
assert!(!moon.city_stories);
assert!(validate_args(&parse(&["--bedrock", "--city-stories"])).is_err());
```
- [ ] **Step 2:** `cargo test --bin arnis city_stories_turns_on` → FAIL (unknown argument).
- [ ] **Step 3:** Add the flag and its wiring exactly where `crime_spree` is wired (same lines listed
  above); GUI keys `city_stories`, `city_stories_desc` ("Explore your streets on a story picked from
  the map, with friends or alone."), `city_stories_tip`, `reason_stories`; English text in all locales.
- [ ] **Step 4:** `cargo test --bin arnis` → all pass (includes `locales_match_en_us_keys`).
- [ ] **Step 5:** Commit "City Stories: the option, its GUI toggle and what it turns on".

### Task 3: Story places and roles

**Files:**
- Create: `src/datapack/stories/mod.rs`, `src/datapack/stories/places.rs`
- Modify: `src/datapack/discoveries.rs` (make `classify` and `outline` `pub(crate)`), `src/datapack/mod.rs` (`pub mod stories;`)

**Interfaces:**
- Consumes: `discoveries::{classify, outline, Place}`.
- Produces:
```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Role { Park, FoodShop, School, Viewpoint, Faith, Bridge, Water, Station, Civic, Culture, Landmark, Any }
pub struct StoryPlace { pub place: Place, pub roles: Vec<Role> } // roles always ends with Role::Any
pub fn collect(elements: &[ProcessedElement], world: &XZBBox) -> Vec<StoryPlace>
```
  Roles: Discoveries categories map (Nature park/garden→Park, beach/lake→Water, Viewpoint→Viewpoint,
  Faith→Faith, Bridge→Bridge, Transport→Station, Civic→Civic, Culture→Culture, Landmark→Landmark);
  extras: `amenity=school|kindergarten|college`→School; `amenity=cafe|marketplace|pub` and
  `shop=bakery|supermarket|convenience|greengrocer|deli`→FoodShop; `amenity=post_office|fire_station`
  →Civic; `leisure=playground`→Park; `landuse=farmland|farmyard`+name→FoodShop. Named only, at most
  200, best scored first, no two within 12 blocks of each other.

- [ ] **Step 1: Failing tests** in `places.rs`: `a_school_and_a_bakery_get_their_roles` (assert
  roles == [School, Any] and [FoodShop, Any]); `unnamed_and_duplicate_places_are_skipped`.
- [ ] **Step 2:** Run → FAIL. **Step 3:** Implement. **Step 4:** Run → PASS.
- [ ] **Step 5:** Call `collect` in `src/data_processing.rs` next to `discoveries::collect` (~1554)
  when `extras.stories`, and settle each place the same way (~1562).
- [ ] **Step 6: Stage review:** `/code-review high --fix` on Tasks 2–3, fix, `cargo test --bin arnis`, commit.

### Task 4: The 15 storylines and the picker

**Files:**
- Create: `src/datapack/stories/lines.rs`

**Interfaces:**
- Consumes: `Role`, `StoryPlace`.
- Produces:
```rust
pub struct Step { pub roles: &'static [Role], pub npc: &'static str, pub line: &'static str } // line has {place}
pub struct Storyline { pub key: &'static str, pub title: &'static str, pub steps: &'static [Step], pub finale: Step }
pub static STORYLINES: [Storyline; 15];
pub struct Plan { pub line: &'static Storyline, pub stops: Vec<usize> } // indices into places, steps then finale
pub fn pick(places: &[StoryPlace], rng: &mut impl Rng) -> Option<Plan>
```
  Keys: `lost_dog, time_capsule, surprise_party, band_reunion, riddle_treasure, friendly_ghost,
  stargazer, cat_mayor, message_bottle, allay_courier, founders_festival, secret_garden, golems_heart,
  picnic_bakeoff, postcard_tour`, roles per the spec's list (Postcard Tour all `Any`). Each storyline
  has 4–5 steps plus the finale. Filling: steps in order, each stop a distinct place having the step's
  first matching role, the nearest unused one within 600 blocks of the previous stop (the first stop
  nearest the area's middle); if none within 600, the nearest at any distance. Eligible = all stops
  filled. Choose uniformly among eligible ones other than `postcard_tour`, which is chosen only when
  nothing else is eligible and then uses min(places, 6) stops (≥3). `None` below 3 places.

- [ ] **Step 1: Failing tests:** `every_storyline_is_filled_with_distinct_places_on_a_rich_map`
  (for each storyline forced via a helper `fill(line, places)`: `Some`, stops unique, stops.len() ==
  steps+1); `a_sparse_map_gets_a_short_postcard_tour` (4 `Any` places → postcard_tour, 4 stops);
  `two_places_get_no_story`; `hops_prefer_places_within_600_blocks`; `the_pick_follows_the_seed`
  (same `ChaCha8Rng` seed → same key).
- [ ] **Step 2–4:** fail, implement, pass.
- [ ] **Step 5:** Commit "City Stories: fifteen storylines fitted to the map".

### Task 5: Writing the story into the pack

**Files:**
- Create: `src/datapack/stories/pack.rs`
- Modify: `src/datapack/mod.rs` (`PackContext.story: Option<stories::Story>`, write generated
  files when `extras.stories`, ticking list entry "stories", pause menu button "City Stories"),
  `src/data_processing.rs` (~2808: drop with warnings — no pedestrians, or `pick` is `None`; build
  `Story` with `rand::rng()`; print `"  City Stories: {n} steps"` without the storyline name)

**Interfaces:**
- Produces:
```rust
pub struct Story { pub plan: Plan, pub places: Vec<StoryPlace> } // only the places stops use
pub fn functions(story: &Story) -> Vec<(String, String)> // ov80/data/arnis/function/story/plan.mcfunction
pub fn advancements(story: &Story, schema: CriteriaSchema) -> Vec<(String, Value)> // data/arnis/advancement/story/...
pub fn errand_pool(places: &[StoryPlace]) -> String // part of plan.mcfunction
```
  `plan.mcfunction` sets `arnis:story` storage: `steps:[{name,x,y,z,npc,line,kind:"step"|"finale"}…]`,
  `finale:"<key>"`, `errands:[{from,to,fx,fy,fz,tx,ty,tz}…]` (up to 24 random pairs 80–600 blocks
  apart) and `#steps arnis.story`, and `#outbreak arnis.story 1` when `extras.outbreak` (night finales check it). Advancements: `story/root` (title "City Stories", tick trigger),
  `story/step_0..N` chain, each `hidden: true`, `criteria: impossible` (granted by functions), title =
  the place name, description = step line with the name filled in; last one is the finale, frame
  `challenge`. JSON text escaping via the existing `json_text` helper.

- [ ] **Step 1: Failing tests:** `the_plan_names_real_places` (fixture story; plan text contains
  each stop's name and `#steps arnis.story 5`); `steps_are_hidden_until_reached` (every
  `step_N` has `hidden: true` and parent `step_{N-1}`); `a_name_with_quotes_stays_valid_snbt`.
- [ ] **Step 2–4:** fail, implement, pass.
- [ ] **Step 5:** Commit.

### Task 6: Walkers get a click target, and the menu opens (shared)

**Files:**
- Create (ov80): `data/arnis/function/npc/{load,attach,click,open,act,tick,left}.mcfunction`,
  `data/arnis/advancement/npc/click.json`; base stubs `data/arnis/function/npc/attach.mcfunction`
  (empty) and `npc/load`, `npc/tick` stubs
- Modify: `assets/.../data/arnis/function/walk/settle.mcfunction` (end: `function arnis:npc/attach`),
  `src/element_processing/pedestrians.rs` `walk/move` generator only if Step 2 shows passengers don't
  follow `tp` (then add `execute on passengers run tp @s ~ ~ ~` after the walker's tp),
  `src/datapack/mod.rs` (`NPC` file list, shipped when `extras.crime || extras.stories`; ticking "npc")

**Interfaces:**
- Produces (in game): walker tags `arnis.walker` + passenger `interaction` tagged `arnis.npc`
  (width 0.8, height 1.9 × the walker's scale); walker `CustomName` from a 40-name pool and a
  trait in `arnis:npc` storage; per-player `arnis.npc.id` (unique) and target walker scored
  `arnis.npc.of = player id`; trigger objective `arnis.npc` (1 hi, 2 chat, 3 directions, 4 rob,
  5 talk, 6 errand, 7 never mind); `#crime arnis.npc` / `#stories arnis.npc` set by each mode's load.
  `npc/open` builds `arnis:npc menu.buttons` (Rob only when `#crime` and the target lacks
  `arnis.story_npc`; Talk only when the target has it; Take the errand only when it has
  `arnis.errand_npc`) and shows it via `$dialog show @s {type:"minecraft:multi_action",title:...,
  actions:$(buttons),columns:3,pause:false}`. `npc/act` runs as the player each tick its trigger
  is ≥1: finds `@e[tag=arnis.walker,distance=..4]` whose `arnis.npc.of` equals the player's id,
  else runs `npc/left` ("They've moved on."). Say hi: the walker faces the player, a villager
  sound, a reply from a 12-line pool. Chat: a line from a 20-line pool; with Stories on, one in four
  names a random stop from `arnis:story steps`. Ask directions: the current story step when Stories
  is on and unfinished, else the nearest entry of the travel destinations storage, as
  "<name>, <m> m <compass>" (same helper as `story/hint`, so `hint` lives in `npc/` and Task 8 calls it); resets the trigger; the walker holds (`arnis.ww` set
  to 100) while targeted.
  `click.json`: `player_interacted_with_entity` with entity `{type:"minecraft:interaction",nbt:"{Tags:[\"arnis.npc\"]}"}`
  so `cars/board` keeps its own unfiltered advancement.

- [ ] **Step 1: Failing pack tests** in `src/datapack/mod.rs` tests:
  `the_menu_ships_with_either_mode` (crime only, stories only, both → `npc/open` present; neither →
  absent); `rob_is_offered_only_with_crime` (assert `npc/open` text gates the Rob button on
  `#crime arnis.npc` and `tag=!arnis.story_npc`).
- [ ] **Step 2:** Harness: summon one walker by hand via RCON, `tp` it 5 blocks, then
  `execute as @e[tag=arnis.walker] on passengers run data get entity @s Pos`. Record whether the
  interaction followed; apply the `move` fallback only if it did not.
- [ ] **Step 3–4:** implement, `cargo test --bin arnis` → PASS; harness on a crime world: no errors,
  `execute as @a run function arnis:npc/open` with a walker in reach succeeds.
- [ ] **Step 5: Stage review:** `/code-review high --fix` on Tasks 4–6 with Review Focus 1–4 named,
  fix, test, commit.

### Task 7: Crime Spree robs through the menu

**Files:**
- Delete: `ov80/.../crime/crouch.mcfunction`
- Modify: `crime/player.mcfunction` (remove the sneak line), `crime/load.mcfunction` (drop
  `arnis.sneak`, `arnis.sneak.last`; set `#crime arnis.npc 1`), `crime/rob.mcfunction` (runs as the
  player, at the player, on the target given by `npc/act`; refuses an `arnis.robbed` target with
  "They've got nothing left."), `crime/intro.mcfunction` and locale `crime_spree_tip` + index.html
  tooltip ("right-click a pedestrian and choose Rob"), `src/datapack/mod.rs` CRIME list

**Interfaces:**
- Consumes: trigger value 4 from Task 6, target selection from `npc/act`.

- [ ] **Step 1:** Update the pack test asserting crime files (crouch gone). Run → FAIL.
- [ ] **Step 2:** Implement; payout, sight, witness and wanted-star rules unchanged.
- [ ] **Step 3:** `cargo test --bin arnis` → PASS; harness: crime world, give a player emeralds 0,
  `trigger`-equivalent `scoreboard players set <p> arnis.npc 4` beside a targeted walker → emeralds > 0.
- [ ] **Step 4:** Commit.

### Task 8: Story runtime: steps, NPCs, hints, catch-up, pause page

**Files:**
- Create (ov80): `data/arnis/function/story/{load,second,near,spawn,despawn,talk,advance,grant,catch_up,hint,skip,where,finale}.mcfunction`;
  base stubs `story/load`, `story/tick`, `story/unsupported`
- Modify: `src/datapack/dialogs.rs` (`pub fn story(ctx) -> Value`: buttons Where next?, Skip this
  step, Current errand; body never names the storyline), `src/datapack/mod.rs` (STORIES list, dialog file)

**Interfaces:**
- Consumes: `arnis:story` storage and `#steps` from Task 5; trigger 5 from Task 6.
- Produces: `#step arnis.story` (0-based, shared), `arnis.story.seen` per player (steps granted),
  story NPC = a walker-look entity tagged `arnis.walker`,`arnis.story_npc` with the same click
  passenger, spawned at the step's place when any player is within 48 and removed when none is
  within 64; `talk` shows the step line in a dialog with "On my way" (trigger 5 again advances);
  `advance` increments `#step`, runs `grant` for all online players, announces the next place;
  `hint` actionbar each second via `npc/hint` (Task 6) at the step's place;
  `skip` = advance with a note; `finale` runs `function arnis:story/finale/<key>` when the last stop is talked to.

- [ ] **Step 1: Failing pack tests:** `the_story_pack_ships_its_functions_and_dialog`;
  `the_pause_page_never_names_the_storyline` (dialog JSON contains none of the 15 titles).
- [ ] **Step 2–3:** implement; `cargo test --bin arnis` → PASS.
- [ ] **Step 4:** Harness on a stories world: `function arnis:story/load`, place a player at step 0,
  run `story/second` twice → story NPC exists; set trigger 5 twice → `#step` = 1 and the player
  has `arnis:story/step_0`; a second fake-joined player run through `catch_up` gets it too.
- [ ] **Step 5:** Commit.

### Task 9: Side errands

**Files:**
- Create (ov80): `data/arnis/function/story/errand/{offer,accept,deliver,find,ride,done,drop}.mcfunction`

**Interfaces:**
- Produces: `#errand arnis.story` (0 none, 1 offered, 2 active), `arnis:story errand` (current pair);
  offer every 180–300 s while none and `#step < #steps`: tags a random walker within 24 of a random
  player `arnis.errand_npc` (chat: "Someone nearby is waving at you."); accept via trigger 6;
  kinds rolled 1–3 (deliver a named parcel to `to`; a glowing dropped item within 40 blocks of the
  player at a footway marker, picked up = done; reach `to` = done for ride); rewards from a
  6-item pool (bread ×4, poppy, firework ×3, music disc cat, 2 emeralds, cake); an active errand
  untouched for 600 s is dropped quietly; never fails.

- [ ] **Step 1: Failing pack test** `errand_files_ship_with_stories`. **Step 2:** implement.
- [ ] **Step 3:** Harness: force offer, accept, `tp` the player to `to` → reward given, `#errand` 0.
- [ ] **Step 4: Stage review:** `/code-review high --fix` on Tasks 7–9 (Review Focus 1–5), fix, commit.

### Task 10: The 15 finales

**Files:**
- Create (ov80): `data/arnis/function/story/finale/<key>.mcfunction` for each of the 15 keys

**Interfaces:**
- Consumes: run `at` the finale place's coordinates by `story/finale`.
- Each: a short scene, a reward for every player within 32, a chat line, grants the last step.
  Pets: tamed to the nearest player (`Owner` from their UUID), named. Night finales
  (`stargazer`, `founders_festival`): `time set night` unless `#outbreak arnis.story` is 1.
  Fireworks via existing `disaster/fireworks.mcfunction` pattern only if it fits; else summon
  `firework_rocket` with SNBT `FireworksItem`.

- [ ] **Step 1: Failing pack test** `every_storyline_has_its_finale` (for each key in `STORYLINES`,
  the pack ships `story/finale/<key>.mcfunction`). **Step 2:** implement the 15.
- [ ] **Step 3:** Harness: for each key, `function arnis:story/finale/<key>` at a test spot → no errors.
- [ ] **Step 4:** Commit.

### Task 11: Both modes together, full check, release-ready

- [ ] **Step 1:** Generate the same small city three ways: stories only, crime only, both.
- [ ] **Step 2:** Harness each: reload clean; menu buttons per Review Focus 1 (read
  `arnis:npc menu.buttons` after `npc/open`); car boarding (Review Focus 2) by running
  `cars/board` with an interaction flagged; despawned target (Focus 3); double rob (Focus 4);
  catch-up (Focus 5).
- [ ] **Step 3:** `cargo test --bin arnis` → all pass; `cargo clippy` clean on touched files.
- [ ] **Step 4: Final stage review:** `/code-review high --fix` over the whole branch, fix, commit,
  push to fork main.
