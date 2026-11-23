//! The stories, from `assets/stories/*.json`, and laying a long run of them over the map:
//! each story's stops a walk apart, each new story somewhere else in the city.

use super::places::{Role, StoryPlace};
use rand::seq::SliceRandom;
use rand::Rng;
use serde::Deserialize;

/// The longest walk between two stops of a story, and from its first stop to any other.
const HOP: f64 = 250.0;
const REACH: f64 = 450.0;
/// The hunt's places: a walk from the meeting place, and apart from each other.
const HUNT_REACH: std::ops::RangeInclusive<f64> = 80.0..=300.0;
const HUNT_SPREAD: f64 = 60.0;
/// A new story starts far from the last few.
const RECENT: usize = 3;

#[derive(Deserialize)]
struct File {
    stories: Vec<Story>,
}

#[derive(Deserialize)]
pub struct Story {
    pub key: String,
    pub title: String,
    pub series: Option<String>,
    pub episode: Option<u32>,
    pub finale: String,
    pub gift: Option<String>,
    pub steps: Vec<Step>,
    pub hunt: Option<Hunt>,
}

#[derive(Deserialize)]
pub struct Step {
    pub roles: Vec<Role>,
    #[serde(flatten)]
    pub beat: Beat,
}

/// What happens at a stop. `{next}` in a text is the next stop's name.
#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Beat {
    /// Someone waits there and says `line`.
    Talk { npc: String, line: String },
    /// The place is not named: the step before gives the riddle, `hint` stays on the bar,
    /// and reaching it shows `line`.
    Riddle { hint: String, line: String },
    /// `thing` lies hidden near the place, unmarked; finding it shows `line`.
    Search {
        thing: String,
        item: String,
        line: String,
    },
    /// Someone asks `line`; `options[answer]` is right and shows `right`, the others `wrong`.
    Choice {
        npc: String,
        line: String,
        options: [String; 3],
        answer: usize,
        wrong: String,
        right: String,
    },
}

#[derive(Deserialize)]
pub struct Hunt {
    pub line: String,
    pub items: [(String, String); 3],
}

impl Step {
    /// The text shown once the step is done, which points on to the next stop.
    pub fn onward(&self) -> &str {
        match &self.beat {
            Beat::Talk { line, .. } | Beat::Riddle { line, .. } | Beat::Search { line, .. } => line,
            Beat::Choice { right, .. } => right,
        }
    }
}

/// Every story Arnis ships, read once.
pub fn book() -> &'static [Story] {
    static BOOK: std::sync::OnceLock<Vec<Story>> = std::sync::OnceLock::new();
    BOOK.get_or_init(|| {
        [
            include_str!("../../../assets/stories/a.json"),
            include_str!("../../../assets/stories/b.json"),
            include_str!("../../../assets/stories/c.json"),
            include_str!("../../../assets/stories/d.json"),
        ]
        .iter()
        .flat_map(|f| parse(f).expect("the shipped stories are checked by tests"))
        .collect()
    })
}

pub fn parse(json: &str) -> Result<Vec<Story>, serde_json::Error> {
    serde_json::from_str::<File>(json).map(|f| f.stories)
}

/// A story laid over the map: its stops, then the hunt's places, as indices into the places.
pub struct Placed<'a> {
    pub story: &'a Story,
    pub stops: Vec<usize>,
    pub hunt: Vec<usize>,
}

fn dist(a: (i32, i32), b: (i32, i32)) -> f64 {
    (((a.0 - b.0) as f64).powi(2) + ((a.1 - b.1) as f64).powi(2)).sqrt()
}

/// Every story the map can carry, in a random order with each series' episodes in turn.
pub fn run<'a>(book: &'a [Story], places: &[StoryPlace], rng: &mut impl Rng) -> Vec<Placed<'a>> {
    let mut order: Vec<usize> = (0..book.len()).collect();
    order.shuffle(rng);
    // A series keeps the slots its episodes drew, filled in episode order.
    let mut series: Vec<&str> = book.iter().filter_map(|s| s.series.as_deref()).collect();
    series.sort();
    series.dedup();
    for name in series {
        let slots: Vec<usize> = (0..order.len())
            .filter(|&i| book[order[i]].series.as_deref() == Some(name))
            .collect();
        let mut eps: Vec<usize> = slots.iter().map(|&i| order[i]).collect();
        eps.sort_by_key(|&s| book[s].episode);
        for (slot, ep) in slots.into_iter().zip(eps) {
            order[slot] = ep;
        }
    }
    let mut out: Vec<Placed> = Vec::new();
    let mut broken: Vec<String> = Vec::new();
    for s in order {
        let recent: Vec<(i32, i32)> = out
            .iter()
            .rev()
            .take(RECENT)
            .map(|p| places[p.stops[0]].place.anchor)
            .collect();
        // A later episode would recall one that never played, so a broken series stops.
        if book[s].series.as_ref().is_some_and(|n| broken.contains(n)) {
            continue;
        }
        match lay(&book[s], places, &recent, rng) {
            Some(p) => out.push(p),
            None => broken.extend(book[s].series.clone()),
        }
    }
    out
}

/// The first place of the first role that has one, among `pick`.
fn first_role(places: &[StoryPlace], roles: &[Role], pick: impl Fn(usize) -> bool) -> Vec<usize> {
    roles
        .iter()
        .map(|r| {
            (0..places.len())
                .filter(|&i| places[i].roles.contains(r) && pick(i))
                .collect::<Vec<_>>()
        })
        .find(|c| !c.is_empty())
        .unwrap_or_default()
}

fn lay<'a>(
    story: &'a Story,
    places: &[StoryPlace],
    recent: &[(i32, i32)],
    rng: &mut impl Rng,
) -> Option<Placed<'a>> {
    let at = |i: usize| places[i].place.anchor;
    // Of a few places that fit, the furthest from where the last stories began whose stops
    // all lie within a walk; an edge of the map may have too few around it.
    let mut starts = first_role(places, &story.steps.first()?.roles, |_| true);
    starts.shuffle(rng);
    starts.truncate(12);
    let away = |i: usize| {
        recent
            .iter()
            .map(|&r| dist(at(i), r))
            .fold(f64::MAX, f64::min)
    };
    starts.sort_by(|&a, &b| away(b).total_cmp(&away(a)));
    let stops = starts
        .into_iter()
        .find_map(|start| walk(story, places, start))?;
    let mut hunt = Vec::new();
    if story.hunt.is_some() && stops.len() >= 2 {
        let meet = at(stops[stops.len() - 2]);
        let mut spare: Vec<usize> = (0..places.len()).filter(|i| !stops.contains(i)).collect();
        spare.shuffle(rng);
        for i in spare {
            if hunt.len() < 3
                && HUNT_REACH.contains(&dist(at(i), meet))
                && hunt.iter().all(|&j| dist(at(i), at(j)) >= HUNT_SPREAD)
            {
                hunt.push(i);
            }
        }
    }
    Some(Placed { story, stops, hunt })
}

/// The story's stops from `start`, each the nearest fitting place a walk from the last.
fn walk(story: &Story, places: &[StoryPlace], start: usize) -> Option<Vec<usize>> {
    let at = |i: usize| places[i].place.anchor;
    let mut stops = vec![start];
    for step in &story.steps[1..] {
        let from = at(*stops.last()?);
        // A riddle's clue describes its kind of place, so it never falls back to just any place.
        let roles: Vec<Role> = match step.beat {
            Beat::Riddle { .. } if step.roles.len() > 1 => step
                .roles
                .iter()
                .copied()
                .filter(|r| *r != Role::Any)
                .collect(),
            _ => step.roles.clone(),
        };
        let near = first_role(places, &roles, |i| {
            !stops.contains(&i) && dist(at(i), from) <= HOP && dist(at(i), at(start)) <= REACH
        });
        let next = near
            .into_iter()
            .min_by(|&a, &b| dist(at(a), from).total_cmp(&dist(at(b), from)))?;
        stops.push(next);
    }
    Some(stops)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::datapack::stories::places::test_place as place;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    /// Every kind of step, a hunt and a two-episode series.
    pub(crate) const SAMPLE: &str = r#"{"stories": [
      {"key": "t_one", "title": "One", "finale": "gift", "gift": "minecraft:cake",
       "steps": [
         {"kind": "talk", "roles": ["park", "any"], "npc": "Ann", "line": "Riddle me this: where do trains sleep?"},
         {"kind": "riddle", "roles": ["station", "any"], "hint": "Where trains sleep", "line": "Here! Now search {next}."},
         {"kind": "search", "roles": ["any"], "thing": "Old Key", "item": "minecraft:tripwire_hook", "line": "The key! Take it to {next}."},
         {"kind": "choice", "roles": ["any"], "npc": "Bo", "line": "Which key was it?", "options": ["Gold", "Old", "New"], "answer": 1, "wrong": "Hmm, no.", "right": "Yes! Meet Cy at {next}."},
         {"kind": "talk", "roles": ["any"], "npc": "Cy", "line": "Everyone, take a look at {next}."},
         {"kind": "talk", "roles": ["any"], "npc": "Di", "line": "The end."}
       ],
       "hunt": {"line": "Fetch {items}.", "items": [["A", "minecraft:paper"], ["B", "minecraft:book"], ["C", "minecraft:string"]]}},
      {"key": "t_ep2", "title": "Two", "series": "t_s", "episode": 2, "finale": "lost_dog",
       "steps": [
         {"kind": "talk", "roles": ["any"], "npc": "Ed", "line": "Go to {next}."},
         {"kind": "talk", "roles": ["any"], "npc": "Fa", "line": "And {next}."},
         {"kind": "talk", "roles": ["any"], "npc": "Gi", "line": "Done again."}
       ]},
      {"key": "t_ep1", "title": "Two begins", "series": "t_s", "episode": 1, "finale": "gift", "gift": "minecraft:emerald",
       "steps": [
         {"kind": "talk", "roles": ["any"], "npc": "Ed", "line": "Go to {next}."},
         {"kind": "talk", "roles": ["any"], "npc": "Fa", "line": "And {next}."},
         {"kind": "talk", "roles": ["any"], "npc": "Gi", "line": "Done."}
       ]}
    ]}"#;

    /// Places every 100 blocks over a 2 km square.
    pub(crate) fn city() -> Vec<StoryPlace> {
        let mut out = Vec::new();
        for x in 0..20 {
            for z in 0..20 {
                let role = if (x + z) % 5 == 0 {
                    Role::Station
                } else {
                    Role::Park
                };
                out.push(place(&format!("P{x}-{z}"), x * 100, z * 100, role));
            }
        }
        out
    }

    /// The finales the pack ships a scene for.
    const FINALES: [&str; 16] = [
        "gift",
        "lost_dog",
        "time_capsule",
        "surprise_party",
        "band_reunion",
        "riddle_treasure",
        "friendly_ghost",
        "stargazer",
        "cat_mayor",
        "message_bottle",
        "allay_courier",
        "founders_festival",
        "secret_garden",
        "golems_heart",
        "picnic_bakeoff",
        "postcard_tour",
    ];

    #[test]
    fn every_shipped_story_follows_the_format() {
        let book = book();
        let mut keys = std::collections::HashSet::new();
        let item = |s: &str| {
            s.starts_with("minecraft:")
                && s[10..]
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        };
        let label = |s: &str| !s.contains(['"', '\\']);
        for s in book {
            let k = &s.key;
            assert!(keys.insert(k.as_str()), "{k} twice");
            // Keys name advancement files.
            assert!(
                k.chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_'),
                "{k}"
            );
            assert!(
                FINALES.contains(&s.finale.as_str()),
                "{k}: finale {}",
                s.finale
            );
            assert!(
                s.finale != "gift" || s.gift.as_deref().is_some_and(item),
                "{k}: gift"
            );
            assert_eq!(
                s.series.is_some(),
                s.episode.is_some(),
                "{k}: series and episode"
            );
            assert!((3..=6).contains(&s.steps.len()), "{k}: steps");
            assert!(
                !matches!(s.steps[0].beat, Beat::Riddle { .. }),
                "{k}: starts with a riddle"
            );
            assert!(
                matches!(s.steps.last().unwrap().beat, Beat::Talk { .. }),
                "{k}: ends without a talk"
            );
            for (i, step) in s.steps.iter().enumerate() {
                assert_eq!(step.roles.last(), Some(&Role::Any), "{k} step {i}: roles");
                let onward = step.onward();
                match s.steps.get(i + 1).map(|n| &n.beat) {
                    None => assert!(!onward.contains("{next}"), "{k}: ending names a next stop"),
                    Some(Beat::Riddle { .. }) => assert!(
                        !onward.contains("{next}"),
                        "{k} step {i} gives the riddle away"
                    ),
                    Some(_) => assert!(onward.contains("{next}"), "{k} step {i} points nowhere"),
                }
                match &step.beat {
                    Beat::Search {
                        thing, item: id, ..
                    } => assert!(label(thing) && item(id), "{k} step {i}"),
                    Beat::Choice { answer, .. } => assert!(*answer < 3, "{k} step {i}"),
                    Beat::Riddle { hint, .. } => assert!(hint.len() <= 40, "{k} step {i}: hint"),
                    Beat::Talk { .. } => {}
                }
            }
            if let Some(h) = &s.hunt {
                assert!(
                    matches!(s.steps[s.steps.len() - 2].beat, Beat::Talk { .. }),
                    "{k}: hunt"
                );
                assert!(h.line.contains("{items}"), "{k}: hunt line");
                // {items} already reads "the Label at Place, ...".
                assert!(
                    !h.line.to_lowercase().contains("the {items}"),
                    "{k}: the the"
                );
                assert!(
                    h.items.iter().all(|(l, i)| label(l) && item(i)),
                    "{k}: hunt items"
                );
            }
        }
        // Each series runs 1, 2, 3... with no gaps.
        let mut series: std::collections::HashMap<&str, Vec<u32>> = Default::default();
        for s in book {
            if let (Some(n), Some(e)) = (&s.series, s.episode) {
                series.entry(n).or_default().push(e);
            }
        }
        for (n, mut eps) in series {
            eps.sort();
            assert_eq!(eps, (1..=eps.len() as u32).collect::<Vec<_>>(), "{n}");
        }
    }

    #[test]
    fn nearly_every_shipped_story_fits_a_city_with_every_kind_of_place() {
        use Role::*;
        let roles = [
            Park, FoodShop, School, Viewpoint, Faith, Bridge, Water, Station, Civic, Culture,
            Landmark,
        ];
        let mut places = Vec::new();
        for x in 0..20 {
            for z in 0..20 {
                places.push(place(
                    &format!("P{x}-{z}"),
                    x * 90,
                    z * 90,
                    roles[(x * 7 + z * 3) as usize % roles.len()],
                ));
            }
        }
        let run = run(book(), &places, &mut ChaCha8Rng::seed_from_u64(9));
        let missing: Vec<&str> = book()
            .iter()
            .filter(|b| !run.iter().any(|p| p.story.key == b.key))
            .map(|b| b.key.as_str())
            .collect();
        assert!(run.len() >= 95, "{} don't fit: {missing:?}", missing.len());
    }

    #[test]
    fn every_kind_of_step_reads() {
        let book = parse(SAMPLE).unwrap();
        assert_eq!(book.len(), 3);
        assert!(matches!(book[0].steps[1].beat, Beat::Riddle { .. }));
        assert!(matches!(book[0].steps[2].beat, Beat::Search { .. }));
        assert_eq!(book[0].steps[3].onward(), "Yes! Meet Cy at {next}.");
        assert_eq!(book[0].steps[1].roles, vec![Role::Station, Role::Any]);
    }

    #[test]
    fn a_story_is_a_walk_and_the_next_starts_elsewhere() {
        let book = parse(SAMPLE).unwrap();
        let places = city();
        let at = |i: usize| places[i].place.anchor;
        let run = run(&book, &places, &mut ChaCha8Rng::seed_from_u64(4));
        assert_eq!(run.len(), 3);
        for p in &run {
            assert_eq!(p.stops.len(), p.story.steps.len());
            for w in p.stops.windows(2) {
                assert!(dist(at(w[0]), at(w[1])) <= HOP, "{}", p.story.key);
            }
            for &s in &p.stops {
                assert!(dist(at(s), at(p.stops[0])) <= REACH);
            }
            let mut seen = p.stops.clone();
            seen.sort();
            seen.dedup();
            assert_eq!(seen.len(), p.stops.len());
        }
        for w in run.windows(2) {
            assert!(
                dist(at(w[0].stops[0]), at(w[1].stops[0])) > REACH,
                "stories bunch up"
            );
        }
        let hunt = &run.iter().find(|p| p.story.key == "t_one").unwrap().hunt;
        assert_eq!(hunt.len(), 3);
    }

    #[test]
    fn a_series_plays_in_episode_order() {
        let book = parse(SAMPLE).unwrap();
        for seed in 0..20 {
            let run = run(&book, &city(), &mut ChaCha8Rng::seed_from_u64(seed));
            let eps: Vec<&str> = run
                .iter()
                .map(|p| p.story.key.as_str())
                .filter(|k| k.starts_with("t_ep"))
                .collect();
            assert_eq!(eps, ["t_ep1", "t_ep2"], "seed {seed}");
        }
    }

    #[test]
    fn a_series_whose_episode_cannot_be_placed_stops_there() {
        let mut book = parse(SAMPLE).unwrap();
        // Episode 1 now needs a school, which the city has none of.
        book[2].steps[1].roles = vec![Role::School];
        let run = run(&book, &city(), &mut ChaCha8Rng::seed_from_u64(2));
        assert!(
            run.iter().all(|p| p.story.series.is_none()),
            "an episode played without the one before"
        );
    }

    #[test]
    fn a_story_with_no_stop_in_walking_range_is_left_out() {
        let book = parse(SAMPLE).unwrap();
        // Three places, each a kilometre from the next.
        let places: Vec<_> = (0..3)
            .map(|i| place(&format!("P{i}"), i * 1000, 0, Role::Any))
            .collect();
        assert!(run(&book, &places, &mut ChaCha8Rng::seed_from_u64(1)).is_empty());
    }
}
