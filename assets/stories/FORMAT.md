# City Stories data format

Each `*.json` file here is `{"stories": [ ... ]}`. Arnis loads every story, places it on the
real map at generation (all of a story's stops within a walk of each other, each new story in
a different part of the city), and the game plays them back to back, forever.

## A story

```json
{
  "key": "b_ufo_1",                 // unique, snake_case, starts with the file's letter
  "title": "Lights Over the Park",  // shown when it starts and in the advancements tab
  "series": "b_ufo",                // optional: episodes of one series play in order,
  "episode": 1,                     //   with other stories in between, like a TV show
  "finale": "gift",                 // "gift", or one of the 15 original finale keys
  "gift": "minecraft:cake",         // with "gift": a vanilla item everyone nearby gets
  "steps": [ ... 3 to 6 steps, the last is the finale ... ],
  "hunt": { ... }                   // optional, see below
}
```

The 15 original finale keys, each a scene of its own: lost_dog, time_capsule, surprise_party,
band_reunion, riddle_treasure, friendly_ghost, stargazer, cat_mayor, message_bottle,
allay_courier, founders_festival, secret_garden, golems_heart, picnic_bakeoff, postcard_tour.

## Steps

Every step has `kind` and `roles`: the kinds of real place it may happen at, best first,
always ending in `"any"`. Roles: park, food_shop, school, viewpoint, faith, bridge, water,
station, civic, culture, landmark, any. `{next}` in a text becomes the next stop's real name.

- **talk**: someone waits at the place; talking to them shows `line`.
  `{"kind":"talk","roles":["park","any"],"npc":"Marty Vance","line":"..."}`
- **riddle**: the place is not named. The step before gives the riddle in its text (and must
  NOT use `{next}`); `hint` is a short reminder on the bar at the top (under 40 characters);
  reaching the place shows `line`.
  `{"kind":"riddle","roles":["bridge","any"],"hint":"Where two banks shake hands","line":"..."}`
- **search**: something is hidden near the place, unmarked; the bar says warmer or colder.
  Finding it shows `line`. `thing` is its name, `item` a vanilla item id.
  `{"kind":"search","roles":["park","any"],"thing":"Strange Crystal","item":"minecraft:amethyst_shard","line":"..."}`
- **choice**: someone asks `line`, a question with three `options`; `answer` is the right
  one's index (0, 1 or 2). The answer must follow from clues earlier in the same story.
  A wrong pick shows `wrong` (a gentle nudge, nothing lost); the right one shows `right`.
  `{"kind":"choice","roles":["culture","any"],"npc":"The curator","line":"Which painting was stolen... er, borrowed?","options":["a","b","c"],"answer":1,"wrong":"...","right":"..."}`

The onward pointer: every step but the last points to the next stop in the text shown when
it is done (`line` for talk, riddle and search; `right` for choice). That text contains
`{next}`, unless the next step is a riddle: then it holds the riddle itself and no `{next}`.
The last step is a `talk` whose `line` is the ending and has no `{next}`.

## Hunt (optional)

Only when the second-to-last step is a `talk`. There the person sends everyone off to three
other places nearby, one thing each, and the story goes on once they regroup; that step's
`line` is said at the reunion (and points to the finale with `{next}`).
`"hunt": {"line": "... {items} ...", "items": [["Label","minecraft:id"], [...], [...]]}`
`{items}` becomes "the Label at Place, ... and the Label at Place".

## Rules

- No crime: no police, stealing, weapons, fighting or danger. Low stress: no timers, no
  failing. Puzzles nudge, never punish.
- Lines are one or two sentences, under 200 characters. Warm, funny, specific.
- Labels (`thing`, hunt item names) have no `"` or `\`.
- Items that can be placed or used up (buckets, maps, food) are fine as gifts but avoid them
  for `search` and hunt things; prefer things like paper, books, gems, nuggets, string, bells.
- Vary the steps: most stories should have at least one riddle, search or choice, and not
  every step should be "go to X". At most two riddles in a story.
