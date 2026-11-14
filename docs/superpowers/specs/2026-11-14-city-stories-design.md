# City Stories: a chill co-op story mode on real maps

## Intent

A game mode for Arnis worlds that is fun alone and with 3+ friends on a server: low stress,
co-op, explore the real city, talk to NPCs, do missions. Each world generation secretly picks
one of 15 storylines, so every world is "generate and find out". Steps send players to real
named places from the OSM data (a school, a park, the library, a bridge), chosen to suit the
story, not at random. Between story steps, random GTA-style side errands keep the streets busy.

Why: Crime Spree is the action mode. This is the opposite mood, built on what makes Arnis
special: players know these streets and places.

## Goals

- One storyline per world, picked at generation from those the map can support, ending in a
  finale at a real landmark. Upcoming steps stay hidden until reached.
- Story steps name the real place ("Ask the librarian at Lillian H. Smith Library").
- NPCs you walk up to and right-click; they talk in a dialog and send you on.
- Random side errands between steps that need no story knowledge.
- Shared progress: any player completes a step for everyone; the finale plays for all nearby.
- Works in single player and on dedicated servers; cheap on slow devices.

## Non-goals

- Branching choices, voiced lines, custom models, a story that changes after generation.
- Bedrock; Minecraft before 1.21.6 (the base layer prints an "unsupported" line, as Crime
  Spree does).
- Rerolling the storyline in-game (approach B, rejected below).

## Anti-goals (a failure even if it works)

- Anything that rushes or punishes: timers, fail states, losing progress on death.
- Getting stuck: an unreachable place, a lost quest item, a despawned NPC.
- Spoilers: the GUI and pause menu never name the storyline before its finale.
- Walls of text; grind (a step should be one trip and one talk).

## Constraints

- Java Edition only, data pack overlay `ov80` (1.21.6+), for dialogs and SNBT text.
- Fits the existing data pack wiring: an `Args` flag, `Extras` field, GUI toggle, locales (all
  locales get every key), static file list plus generated files, as Discoveries and Disasters do.
- Per-second work, not per-tick, except what a click needs. Few entities: one NPC per active
  place, spawned only while a player is near.
- Short, robust code (ponytail): data-driven storylines, one shared step engine.

## Approaches

- **A. Pick at generation (chosen).** Rust rolls the storyline, chooses fitting real places,
  writes the step text, place coordinates and advancements into the pack. Real names in the
  dialog, testable in Rust, each story uses places that suit it. Cost: a world keeps its story;
  regenerate for another. This is the "generate and find out" the user asked for.
- **B. Pick in-game on first load.** All 15 shipped, rolled at load, restartable from the pause
  menu. Rejected: text assembled from storage in mcfunction is fragile and generic, hard to
  test, and places can't be fitted to the story as well.
- **C. Hand-written per area.** Rejected: doesn't scale to arbitrary maps.

## Design

### 1. Story places

A collector over the OSM elements, broader than Discoveries. It reuses `discoveries::classify`
and adds everyday places stories need: school, cafe, bakery, supermarket or convenience shop,
post office, fire station, playground, marketplace, pub, plus Discoveries' parks, libraries,
churches, viewpoints, bridges, stations, town halls, museums, beaches and lakes. Each place
gets one or more **roles** (for example `park`, `food_shop`, `school`, `viewpoint`, `faith`,
`bridge`, `water`, `station`, `civic`, `culture`, `landmark`, `any`). Named places only. Places
are settled to a reachable street spot the way `Place::settle` does.

### 2. Storylines as data

Each of the 15 storylines is a small Rust table: a title, 4–6 steps and a finale. A step
names the role it wants (with fallbacks), the NPC (name, look), a one- or two-line text
template with `{place}`, and an optional flavour item. A storyline is eligible when every step
can be filled with a distinct place. Among eligible ones, generation picks one at random,
weighted to prefer those that fit this map's mix best. Places in a chain are picked so each
hop is a walk or a ride rather than a trek: a soft cap of about 600 blocks, relaxed if the map
is sparse. **The Postcard Tour** needs only `any` places and shrinks to however many exist (3–6
steps), so it is always eligible; every other storyline needs its full set of distinct places
(builder's call, from the review). With fewer than 3 named places the mode is dropped with a warning, the way Crime Spree is dropped without
pedestrians.

The 15 storylines, each finale at a real place:

1. **The Lost Dog**: parks. Find the dog; it becomes a tamed, named pet.
2. **Time Capsule**: library → landmark. Dig up a chest of letters (written books).
3. **Surprise Party**: food shops → viewpoint. Party with fireworks; pedestrians gather.
4. **Band Reunion**: stations → square or park. Note-block concert by three musicians.
5. **Riddle Treasure**: riddles naming real places → bridge. Treasure chest.
6. **The Friendly Ghost**: letters to deliver → church. The ghost leaves in particles.
7. **Stargazer**: three viewpoints, at night. Firework "meteor shower".
8. **The Cat Mayor**: around town → town hall. The cat is crowned (named, tamed cat).
9. **Message in a Bottle**: water → bridge. Two old friends reunite.
10. **The Allay Courier**: lost items around town. The allay joins the players.
11. **Founders' Festival**: hang banners at civic places → square. Lantern festival night.
12. **The Secret Garden**: seeds from parks and gardens → a park that blooms.
13. **The Golem's Heart**: parts around town → a friendly iron golem guardian.
14. **Picnic Bake-off**: farms or food shops → park picnic.
15. **The Postcard Tour**: famous spots → a signed painting for home.

### 3. Steps in game

- The current step's place gets an NPC (a mannequin with a skin from 1.21.9, a villager
  before) and an `interaction` entity for the click. Both are spawned when a player comes
  within about 48 blocks and removed when no one is near, so nothing piles up or despawns
  for good.
- Right-click opens a dialog with the NPC's lines and an "On my way" button, which advances
  the shared step, grants the step's advancement to every online player, gives any flavour
  item, and announces the next place in chat.
- A direction hint shows the way each second in the actionbar ("St. James Cathedral, 340 m
  north-east") while a step is active.
- **Never stuck:** quest items are flavour only, never required. The pause-menu page has
  "Skip this step" (it moves the story on, with a note that the place may be hard to reach)
  and "Where next?".

### 3b. Talking to people: the interaction menu (shared with Crime Spree)

A Sims-style menu for every pedestrian and story NPC, vanilla only (no mods). It replaces Crime
Spree's crouch-to-rob.

- **Opening it:** right-click a pedestrian while looking at them. Vanilla can't detect custom
  keys (tilde included), so right-click is the trigger. Each walker carries an invisible
  `interaction` entity, the way cars catch their clicks; the builder checks it follows the
  walker's teleports, by riding or by moving with it. A `player_interacted_with_entity`
  advancement runs a function that finds the clicked walker and tags it as that player's target.
- **The menu:** a 1.21.6 dialog screen, shown with an inline dialog from a macro so it can carry
  the person's name and a one-line trait ("Maria, on her way to work"). Buttons are in a grid,
  not a ring (vanilla lays dialogs out; the user accepted this). Icons come from font glyphs in
  the world resource pack, next to the police uniform. Buttons run a trigger, as the Crime Spree
  settings do; the action applies to the tagged target if it is still within 4 blocks.
- **Actions:** Say hi (they stop, turn to you, a sound and a reply), Chat (a line of small talk,
  sometimes naming a real place nearby), Ask directions (to the current story step, else the
  nearest named place), Never mind. Crime Spree adds **Rob**, with the same payout, sight and
  witness rules as crouch-robbing had. City Stories adds **Talk** on story NPCs and **Take the
  errand** on the passer-by offering one. The person stands still while the menu is open, and
  for a few seconds after.
- **Names:** each walker gets a name and a trait from a small pool when it spawns.

### 4. Advancement tab

A "City Stories" tab: the root, then one advancement per step in a chain, each hidden until
its parent is done, then the finale. Titles name the real place once revealed. Players who
join later are granted everything done so far on their first second.

### 5. Side errands

At most one shared errand at a time, offered by a passer-by every few minutes while none is
active, and only while the story isn't at its finale. Kinds: deliver a parcel to a named
place; find a lost item (a glowing dropped item near a street); take a minecart or car to a
named place. Generation writes a pool of place pairs to draw from. Rewards are small and nice:
food, flowers, fireworks, a music disc, a few emeralds. Errands never fail; a new one replaces
an ignored one after a while.

### 6. Finales

One static mcfunction per storyline, which the generated data points at a real place (stored
coordinates). Each is a short scene, a reward for everyone present, a chat line, and the final
advancement. Afterwards side errands keep going, so the world stays playable.

Night scenes (Stargazer, Founders' Festival) set the time to night when their finale starts,
with a chat line, and never wait for it; if Zombie Outbreak is on, they play at whatever time
it is instead, so they don't fight its day and night logic (builder's call, from the review).

### 7. Multiplayer and performance

Progress lives in scoreboard fake players and storage, not on a player. Steps, errands and the
finale are shared; any player can do them. The per-second loop checks distance to the current
place and errand only. One NPC and one interaction entity at a time.

### 8. Settings and GUI

- New option "City Stories" in the GUI, Java only, with the same wiring as Crime Spree: `Args`
  flag, `apply_feature_needs` turns on the pause menu, pedestrians and moving vehicles
  (forced on whatever the GUI picked, as Crime Spree does; the user's call), the off-Earth reset clears it, every
  locale gets the keys.
- It works alongside other modes; nothing is made exclusive.
- The pause menu gets a "City Stories" page: Where next?, Skip this step, and the current
  errand. It never names the storyline before its finale.

### 9. Tests

- Classifying places into roles.
- Each storyline's eligibility, and the Postcard Tour fallback on a sparse map.
- The mode is dropped below 3 places.
- The generated pack names real places in step text and advancements.
- The menu dialog lists Rob only with Crime Spree, and Talk only on story NPCs.
- Existing pack tests cover function references and shipped files.

## Left to the builder

The exact lines of dialogue, NPC skins, rewards, distance numbers and errand timing, the
direction-hint format, and how the weighting prefers a storyline, all within the rules above.
