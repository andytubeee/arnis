//! The world's story, worked out once at generation and written into the pack.

use super::lines::{pick, Plan};
use super::places::StoryPlace;
use crate::datapack::json_text;
use rand::Rng;
use serde_json::{json, Value};
use std::fmt::Write;

/// Most errands the pack carries, and how far apart their two places are, in blocks.
const ERRANDS: usize = 24;
const ERRAND_REACH: std::ops::RangeInclusive<f64> = 80.0..=600.0;

pub struct Story {
    pub plan: Plan,
    /// Every place a story or an errand can use; the plan's stops index into it.
    pub places: Vec<StoryPlace>,
    /// Errands as (from, to) indices into `places`.
    pub errands: Vec<(usize, usize)>,
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
            let (pa, pb) = (places[a].place.anchor, places[b].place.anchor);
            let d = (((pa.0 - pb.0) as f64).powi(2) + ((pa.1 - pb.1) as f64).powi(2)).sqrt();
            if ERRAND_REACH.contains(&d) && !errands.contains(&(a, b)) {
                errands.push((a, b));
            }
        }
        Some(Self {
            plan,
            places,
            errands,
            outbreak,
        })
    }

    fn spot(&self, i: usize) -> (String, i32, i32, i32) {
        let p = &self.places[i].place;
        (
            json_text(&json!(p.name)),
            p.anchor.0,
            p.ground_y.unwrap_or(64),
            p.anchor.1,
        )
    }

    /// `story/plan`: the steps with their real places and lines, the errands, and the counts
    /// the static story functions read.
    pub fn plan_function(&self) -> String {
        let stops = &self.plan.stops;
        let mut steps = Vec::new();
        for (k, (step, i)) in self.plan.steps().enumerate() {
            let next = stops
                .get(k + 1)
                .map_or("", |&j| self.places[j].place.name.as_str());
            let (name, x, y, z) = self.spot(i);
            steps.push(format!(
                "{{name:{name},x:{x},y:{y},z:{z},npc:{},line:{}}}",
                json_text(&json!(step.npc)),
                json_text(&json!(step.line.replace("{next}", next)))
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
        let mut out = String::new();
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
