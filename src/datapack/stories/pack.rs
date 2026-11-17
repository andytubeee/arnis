//! The world's story, worked out once at generation and written into the pack.

use super::lines::{pick, Plan};
use super::places::StoryPlace;
use crate::datapack::json_text;
use rand::seq::SliceRandom;
use rand::Rng;
use serde_json::{json, Value};
use std::fmt::Write;

/// Most errands the pack carries, and how far apart their two places are, in blocks.
const ERRANDS: usize = 24;
const ERRAND_REACH: std::ops::RangeInclusive<f64> = 80.0..=600.0;
/// The hunt's places are at least this far apart, so everyone goes somewhere else.
const SPREAD: f64 = 80.0;

fn dist(a: (i32, i32), b: (i32, i32)) -> f64 {
    (((a.0 - b.0) as f64).powi(2) + ((a.1 - b.1) as f64).powi(2)).sqrt()
}

pub struct Story {
    pub plan: Plan,
    /// Every place a story or an errand can use; the plan's stops index into it.
    pub places: Vec<StoryPlace>,
    /// Errands as (from, to) indices into `places`.
    pub errands: Vec<(usize, usize)>,
    /// Where the hunt's things lie, indices into `places`; empty when no spare place fits.
    pub hunt: Vec<usize>,
    /// With a Zombie Outbreak running, night finales leave the time alone.
    pub outbreak: bool,
}

impl Story {
    /// The world's storyline and errands, or `None` when the map has too few places.
    pub fn new(places: Vec<StoryPlace>, outbreak: bool, rng: &mut impl Rng) -> Option<Self> {
        let plan = pick(&places, rng)?;
        let mut errands = Vec::new();
        for _ in 0..ERRANDS * 20 {
            if errands.len() == ERRANDS {
                break;
            }
            let (a, b) = (
                rng.random_range(0..places.len()),
                rng.random_range(0..places.len()),
            );
            let d = dist(places[a].place.anchor, places[b].place.anchor);
            if ERRAND_REACH.contains(&d) && !errands.contains(&(a, b)) {
                errands.push((a, b));
            }
        }
        // Spare places a walk from the meeting place and from each other.
        let at = |i: usize| places[i].place.anchor;
        let meet = at(plan.stops[plan.stops.len() - 2]);
        let mut spare: Vec<usize> = (0..places.len())
            .filter(|i| !plan.stops.contains(i))
            .collect();
        spare.shuffle(rng);
        let mut hunt: Vec<usize> = Vec::new();
        for i in spare {
            if hunt.len() < 3
                && ERRAND_REACH.contains(&dist(at(i), meet))
                && hunt.iter().all(|&j| dist(at(i), at(j)) >= SPREAD)
            {
                hunt.push(i);
            }
        }
        Some(Self {
            plan,
            places,
            errands,
            hunt,
            outbreak,
        })
    }

    /// A place's quoted name, and where to stand there: x, the block above its ground, z.
    fn spot(&self, i: usize) -> (String, i32, i32, i32) {
        let p = &self.places[i].place;
        (
            json_text(&json!(p.name)),
            p.anchor.0,
            p.ground_y.unwrap_or(63) + 1,
            p.anchor.1,
        )
    }

    /// `story/plan`: the steps with their real places and lines, the errands, and the counts
    /// the static story functions read.
    pub fn plan_function(&self) -> String {
        let stops = &self.plan.stops;
        let mut steps = Vec::new();
        let last = stops.len() - 1;
        let hunt_step = if self.hunt.is_empty() {
            None
        } else {
            Some(last - 1)
        };
        let things = &self.plan.line.hunt.items;
        let mut items: Vec<String> = self
            .hunt
            .iter()
            .zip(things)
            .map(|(&i, (label, _))| format!("the {label} at {}", self.places[i].place.name))
            .collect();
        let tail = items.pop().unwrap_or_default();
        let items = if items.is_empty() {
            tail
        } else {
            format!("{} and {tail}", items.join(", "))
        };
        for (k, (step, i)) in self.plan.steps().enumerate() {
            let next = stops
                .get(k + 1)
                .map_or("", |&j| self.places[j].place.name.as_str());
            let line = step.line.replace("{next}", next);
            // At the hunt step the person sends everyone off first, and says their line
            // once all are back.
            let hunting = hunt_step == Some(k);
            let said = if hunting {
                self.plan.line.hunt.line.replace("{items}", &items)
            } else {
                line.clone()
            };
            // Shown whole by a macro, which writes a compound out as valid SNBT.
            let dialog = json!({
                "type": "minecraft:notice",
                "title": { "text": step.npc, "color": "aqua" },
                "body": { "type": "minecraft:plain_message", "contents": { "text": said } },
                "action": {
                    "label": if hunting { "Let's split up!" } else if k == last { "Thank you!" } else { "On my way" },
                    "action": {
                        "type": "run_command",
                        "command": format!("trigger arnis.story.ctl set {}", 10 + k),
                    },
                },
                "pause": false,
            });
            let (name, x, y, z) = self.spot(i);
            steps.push(format!(
                // sy: spreadplayers searches down from there for ground, as travel does.
                "{{name:{name},x:{x},y:{y},sy:{},z:{z},npc:{},dialog:{},after:{}}}",
                y + 3,
                json_text(&json!(step.npc)),
                json_text(&dialog),
                json_text(&json!(line))
            ));
        }
        let errands: Vec<String> = self
            .errands
            .iter()
            .map(|&(a, b)| {
                let (from, fx, fy, fz) = self.spot(a);
                let (to, tx, ty, tz) = self.spot(b);
                format!("{{from:{from},fx:{fx},fy:{fy},fz:{fz},to:{to},tx:{tx},ty:{ty},tz:{tz}}}")
            })
            .collect();
        let hunt: Vec<String> = self
            .hunt
            .iter()
            .zip(things)
            .enumerate()
            .map(|(k, (&i, (label, item)))| {
                let (name, x, y, z) = self.spot(i);
                format!(
                    "{{k:{k},label:{},item:\"{item}\",name:{name},x:{x},y:{y},sy:{},z:{z}}}",
                    json_text(&json!(label)),
                    y + 3
                )
            })
            .collect();
        let mut out = String::new();
        writeln!(
            out,
            "data modify storage arnis:story hunt set value [{}]",
            hunt.join(",")
        )
        .unwrap();
        writeln!(
            out,
            "scoreboard players set #hunt_step arnis.story {}",
            hunt_step.map_or(-1, |h| h as i64)
        )
        .unwrap();
        writeln!(
            out,
            "scoreboard players set #arts arnis.story {}",
            hunt.len()
        )
        .unwrap();
        writeln!(
            out,
            "data modify storage arnis:story steps set value [{}]",
            steps.join(",")
        )
        .unwrap();
        writeln!(
            out,
            "data modify storage arnis:story errands set value [{}]",
            errands.join(",")
        )
        .unwrap();
        writeln!(
            out,
            "data modify storage arnis:story finale set value {}",
            json_text(&json!(self.plan.line.key))
        )
        .unwrap();
        writeln!(
            out,
            "scoreboard players set #steps arnis.story {}",
            stops.len()
        )
        .unwrap();
        writeln!(
            out,
            "scoreboard players set #outbreak arnis.story {}",
            u8::from(self.outbreak)
        )
        .unwrap();
        out
    }

    /// The City Stories tab, as (path below `advancement/story`, JSON): the root, then a
    /// hidden advancement per step, granted by the story functions, the finale last.
    pub fn advancements(&self) -> Vec<(String, Value)> {
        let mut out = vec![(
            "root".to_string(),
            json!({
                "display": {
                    "icon": { "id": "minecraft:writable_book" },
                    "title": "City Stories",
                    "description": "Talk to people and follow where they send you",
                    "background": "minecraft:block/oak_planks",
                    "show_toast": false,
                    "announce_to_chat": false,
                },
                "criteria": { "joined": { "trigger": "minecraft:tick" } },
            }),
        )];
        let last = self.plan.stops.len() - 1;
        let stops = &self.plan.stops;
        for (k, (step, i)) in self.plan.steps().enumerate() {
            let next = stops
                .get(k + 1)
                .map_or("", |&j| self.places[j].place.name.as_str());
            let parent = if k == 0 {
                "arnis:story/root".to_string()
            } else {
                format!("arnis:story/step_{}", k - 1)
            };
            let (title, icon, frame) = if k == last {
                (self.plan.line.title, "minecraft:nether_star", "challenge")
            } else {
                (self.places[i].place.name.as_str(), "minecraft:map", "task")
            };
            out.push((
                format!("step_{k}"),
                json!({
                    "parent": parent,
                    "display": {
                        "icon": { "id": icon },
                        "title": title,
                        "description": step.line.replace("{next}", next),
                        "frame": frame,
                        "hidden": true,
                        "show_toast": true,
                        "announce_to_chat": false,
                    },
                    "criteria": { "done": { "trigger": "minecraft:impossible" } },
                }),
            ));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::datapack::stories::places::{test_place, Role};
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    fn story() -> Story {
        let names = [
            "Union Station",
            "Rosa's \"Best\" Bakery",
            "Grange Park",
            "City Hall",
        ];
        let places = names
            .iter()
            .enumerate()
            .map(|(i, n)| test_place(n, i as i32 * 120, 0, Role::Any))
            .collect();
        Story::new(places, false, &mut ChaCha8Rng::seed_from_u64(3)).unwrap()
    }

    #[test]
    fn the_plan_names_real_places() {
        let s = story();
        let text = s.plan_function();
        for &i in &s.plan.stops {
            let name = &s.places[i].place.name;
            assert!(
                text.contains(&name.replace('"', "\\\"")),
                "{name} missing:\n{text}"
            );
        }
        assert!(
            text.contains("scoreboard players set #steps arnis.story 4"),
            "{text}"
        );
        assert!(
            text.contains("scoreboard players set #outbreak arnis.story 0"),
            "{text}"
        );
    }

    /// The story's four places and six more a street away, for the hunt.
    fn town() -> Story {
        let mut places: Vec<_> = (0..4)
            .map(|i| test_place(&format!("Stop {i}"), i * 120, 0, Role::Any))
            .collect();
        places.extend((0..6).map(|i| test_place(&format!("Spare {i}"), i * 130, 250, Role::Any)));
        Story::new(places, false, &mut ChaCha8Rng::seed_from_u64(5)).unwrap()
    }

    #[test]
    fn the_hunt_sends_everyone_somewhere_else_near_the_meeting_place() {
        let s = town();
        assert!((1..=3).contains(&s.hunt.len()), "{:?}", s.hunt);
        let at = |i: usize| s.places[i].place.anchor;
        let meet = at(s.plan.stops[s.plan.stops.len() - 2]);
        for (n, &i) in s.hunt.iter().enumerate() {
            assert!(!s.plan.stops.contains(&i));
            assert!(ERRAND_REACH.contains(&dist(at(i), meet)));
            for &j in &s.hunt[..n] {
                assert!(dist(at(i), at(j)) >= SPREAD);
            }
        }
    }

    #[test]
    fn the_meeting_step_asks_to_split_up_and_keeps_its_line_for_the_reunion() {
        let s = town();
        let text = s.plan_function();
        let h = s.plan.stops.len() - 2;
        assert!(
            text.contains(&format!(
                "scoreboard players set #hunt_step arnis.story {h}"
            )),
            "{text}"
        );
        assert!(text.contains(&format!(
            "scoreboard players set #arts arnis.story {}",
            s.hunt.len()
        )));
        assert!(text.contains("Let's split up!"), "{text}");
        assert!(text.contains(",after:"), "{text}");
        assert!(
            text.contains("data modify storage arnis:story hunt set value [{k:0,"),
            "{text}"
        );
        let (label, item) = s.plan.line.hunt.items[0];
        assert!(text.contains(label) && text.contains(item), "{text}");
    }

    #[test]
    fn without_spare_places_there_is_no_hunt() {
        let s = story();
        assert!(s.hunt.is_empty());
        let text = s.plan_function();
        assert!(
            text.contains("scoreboard players set #hunt_step arnis.story -1"),
            "{text}"
        );
        assert!(!text.contains("Let's split up!"), "{text}");
    }

    #[test]
    fn people_stand_on_the_ground_and_spread_from_above_it() {
        // Ground at 70: stand at 71, and spreadplayers searches down from 74 as travel does.
        let text = story().plan_function();
        assert!(text.contains(",y:71,sy:74,"), "{text}");
        assert!(text.contains(",ty:71,"), "{text}");
    }

    #[test]
    fn steps_are_hidden_until_reached() {
        let s = story();
        let advs = s.advancements();
        assert_eq!(advs.len(), 1 + s.plan.stops.len());
        for (n, (path, adv)) in advs.iter().skip(1).enumerate() {
            assert_eq!(path, &format!("step_{n}"));
            assert_eq!(adv["display"]["hidden"], true, "{path}");
            let parent = if n == 0 {
                "arnis:story/root".to_string()
            } else {
                format!("arnis:story/step_{}", n - 1)
            };
            assert_eq!(adv["parent"], parent.as_str());
            assert_eq!(adv["criteria"]["done"]["trigger"], "minecraft:impossible");
        }
    }

    #[test]
    fn a_name_with_quotes_stays_valid_snbt() {
        let text = story().plan_function();
        let line = text.lines().find(|l| l.contains("Bakery")).unwrap();
        assert!(line.contains(r#"Rosa's \"Best\" Bakery"#), "{line}");
    }

    #[test]
    fn errands_join_places_80_to_600_blocks_apart() {
        let s = story();
        assert!(!s.errands.is_empty());
        for &(a, b) in &s.errands {
            let (pa, pb) = (s.places[a].place.anchor, s.places[b].place.anchor);
            let d = (((pa.0 - pb.0).pow(2) + (pa.1 - pb.1).pow(2)) as f64).sqrt();
            assert!((80.0..=600.0).contains(&d), "{d}");
        }
    }
}
