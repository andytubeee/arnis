//! The world's run of stories, laid over the map at generation and written into the pack.

use super::lines::{run, Beat, Placed, Story as Def};
use super::places::StoryPlace;
use crate::datapack::json_text;
use rand::Rng;
use serde_json::{json, Value};
use std::fmt::Write;

/// Most errands the pack carries, and how far apart their two places are, in blocks.
const ERRANDS: usize = 24;
const ERRAND_REACH: std::ops::RangeInclusive<f64> = 80.0..=600.0;

fn dist(a: (i32, i32), b: (i32, i32)) -> f64 {
    (((a.0 - b.0) as f64).powi(2) + ((a.1 - b.1) as f64).powi(2)).sqrt()
}

pub struct Story {
    /// The stories in the order they play, each on its own places; played round and round.
    pub run: Vec<Placed<'static>>,
    /// Every place a story or an errand can use; the run indexes into it.
    pub places: Vec<StoryPlace>,
    /// Errands as (from, to) indices into `places`.
    pub errands: Vec<(usize, usize)>,
    /// With a Zombie Outbreak running, night finales leave the time alone.
    pub outbreak: bool,
}

impl Story {
    /// The world's stories and errands, or `None` when no story fits the map.
    pub fn new(places: Vec<StoryPlace>, outbreak: bool, rng: &mut impl Rng) -> Option<Self> {
        Self::with_book(super::lines::book(), places, outbreak, rng)
    }

    pub fn with_book(
        book: &'static [Def],
        places: Vec<StoryPlace>,
        outbreak: bool,
        rng: &mut impl Rng,
    ) -> Option<Self> {
        if places.len() < 3 {
            return None;
        }
        let run = run(book, &places, rng);
        if run.is_empty() {
            return None;
        }
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
        Some(Self {
            run,
            places,
            errands,
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

    /// One story as SNBT: its steps with real places and lines, its hunt and its finale.
    fn story_snbt(&self, p: &Placed) -> String {
        let def = p.story;
        let name = |i: usize| self.places[i].place.name.as_str();
        let last = p.stops.len() - 1;
        let hunt_step = (def.hunt.is_some() && !p.hunt.is_empty()).then(|| last - 1);
        let mut hunt = Vec::new();
        let mut items: Vec<String> = Vec::new();
        if let Some(h) = &def.hunt {
            for (k, (&i, (label, item))) in p.hunt.iter().zip(&h.items).enumerate() {
                let (place, x, y, z) = self.spot(i);
                hunt.push(format!(
                    "{{k:{k},label:{},item:\"{item}\",name:{place},x:{x},y:{y},sy:{},z:{z}}}",
                    json_text(&json!(label)),
                    y + 3
                ));
                items.push(format!("the {label} at {}", name(i)));
            }
        }
        let tail = items.pop().unwrap_or_default();
        let items = if items.is_empty() {
            tail
        } else {
            format!("{} and {tail}", items.join(", "))
        };
        let mut steps = Vec::new();
        for (k, (step, &i)) in def.steps.iter().zip(&p.stops).enumerate() {
            let next = p.stops.get(k + 1).map_or("", |&j| name(j));
            let onward = json_text(&json!(step.onward().replace("{next}", next)));
            let (place, x, y, z) = self.spot(i);
            let press = format!("trigger arnis.story.ctl set {}", 10 + k);
            // sy: spreadplayers searches down from there for ground, as travel does.
            let mut s = format!("name:{place},x:{x},y:{y},sy:{},z:{z},after:{onward}", y + 3);
            let mut person = |npc: &str, dialog: Value| {
                write!(
                    s,
                    ",npc:{},dialog:{}",
                    json_text(&json!(npc)),
                    json_text(&dialog)
                )
                .unwrap();
            };
            match &step.beat {
                Beat::Talk { npc, line } => {
                    let hunting = hunt_step == Some(k);
                    let said = match &def.hunt {
                        Some(h) if hunting => h.line.replace("{items}", &items),
                        _ => line.replace("{next}", next),
                    };
                    let label = if hunting {
                        "Let's split up!"
                    } else if k == last {
                        "Thank you!"
                    } else {
                        "On my way"
                    };
                    // Shown whole by a macro, which writes a compound out as valid SNBT.
                    person(
                        npc,
                        json!({
                            "type": "minecraft:notice",
                            "title": { "text": npc, "color": "aqua" },
                            "body": { "type": "minecraft:plain_message", "contents": { "text": said } },
                            "action": { "label": label, "action": { "type": "run_command", "command": press } },
                            "pause": false,
                        }),
                    );
                    s.push_str(",kind:\"talk\"");
                }
                Beat::Choice {
                    npc,
                    line,
                    options,
                    answer,
                    wrong,
                    ..
                } => {
                    // A wrong pick (8) only gets a nudge; the right one moves the story on.
                    let actions: Vec<Value> = options
                        .iter()
                        .enumerate()
                        .map(|(j, o)| {
                            let command = if j == *answer { press.clone() } else { "trigger arnis.story.ctl set 8".into() };
                            json!({ "label": o, "action": { "type": "run_command", "command": command } })
                        })
                        .collect();
                    person(
                        npc,
                        json!({
                            "type": "minecraft:multi_action",
                            "title": { "text": npc, "color": "aqua" },
                            "body": { "type": "minecraft:plain_message", "contents": { "text": line } },
                            "actions": actions,
                            "columns": 1,
                            "pause": false,
                        }),
                    );
                    write!(s, ",kind:\"choice\",wrong:{}", json_text(&json!(wrong))).unwrap();
                }
                Beat::Riddle { hint, .. } => {
                    write!(s, ",kind:\"riddle\",hint:{}", json_text(&json!(hint))).unwrap();
                }
                Beat::Search { thing, item, .. } => {
                    let (hx, hz) = self.places[i].nook;
                    write!(
                        s,
                        ",kind:\"search\",thing:{},item:\"{item}\",hx:{hx},hz:{hz}",
                        json_text(&json!(thing))
                    )
                    .unwrap();
                }
            }
            steps.push(format!("{{{s}}}"));
        }
        format!(
            "{{key:{},title:{},ep:{},finale:{},gift:{},hunt_step:{},arts:{},hunt:[{}],steps:[{}]}}",
            json_text(&json!(def.key)),
            json_text(&json!(def.title)),
            def.episode.unwrap_or(0),
            json_text(&json!(def.finale)),
            json_text(&json!(def.gift.as_deref().unwrap_or("minecraft:cake"))),
            hunt_step.map_or(-1, |h| h as i64),
            hunt.len(),
            hunt.join(","),
            steps.join(",")
        )
    }

    /// `story/plan`: every story of the run, one line each, the errands, and the counts the
    /// static story functions read.
    pub fn plan_function(&self) -> String {
        let mut out = String::from("data modify storage arnis:story list set value []\n");
        for p in &self.run {
            writeln!(
                out,
                "data modify storage arnis:story list append value {}",
                self.story_snbt(p)
            )
            .unwrap();
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
        writeln!(
            out,
            "data modify storage arnis:story errands set value [{}]",
            errands.join(",")
        )
        .unwrap();
        writeln!(
            out,
            "scoreboard players set #stories arnis.story {}",
            self.run.len()
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

    /// The City Stories tab, as (path below `advancement/story`, JSON): the root, then one
    /// hidden advancement per story, granted to everyone online when it ends.
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
        for p in &self.run {
            let def = p.story;
            let ending = def.steps.last().map_or("", |s| s.onward());
            out.push((
                def.key.clone(),
                json!({
                    "parent": "arnis:story/root",
                    "display": {
                        "icon": { "id": "minecraft:map" },
                        "title": def.title,
                        "description": ending,
                        "frame": "task",
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
    use crate::datapack::stories::lines::parse;
    use crate::datapack::stories::lines::tests::{city, SAMPLE};
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    fn story() -> Story {
        let book: &'static [Def] = Box::leak(parse(SAMPLE).unwrap().into_boxed_slice());
        let mut places = city();
        places[0].place.name = "Rosa's \"Best\" Bakery".into();
        Story::with_book(book, places, false, &mut ChaCha8Rng::seed_from_u64(3)).unwrap()
    }

    fn line_of<'a>(text: &'a str, key: &str) -> &'a str {
        text.lines()
            .find(|l| l.contains(&format!("key:\"{key}\"")))
            .unwrap()
    }

    #[test]
    fn every_story_is_written_with_its_real_places() {
        let s = story();
        let text = s.plan_function();
        assert!(
            text.contains("scoreboard players set #stories arnis.story 3"),
            "{text}"
        );
        for p in &s.run {
            let line = line_of(&text, &p.story.key);
            for &i in &p.stops {
                let name = s.places[i].place.name.replace('"', "\\\"");
                assert!(line.contains(&name), "{name} missing:\n{line}");
            }
        }
    }

    #[test]
    fn people_stand_on_the_ground_and_spread_from_above_it() {
        // Ground at 70: stand at 71, and spreadplayers searches down from 74 as travel does.
        let text = story().plan_function();
        assert!(text.contains(",y:71,sy:74,"), "{text}");
    }

    #[test]
    fn each_kind_of_step_carries_what_the_game_needs() {
        let s = story();
        let text = s.plan_function();
        let line = line_of(&text, "t_one");
        assert!(
            line.contains("kind:\"riddle\",hint:\"Where trains sleep\""),
            "{line}"
        );
        assert!(
            line.contains("kind:\"search\",thing:\"Old Key\",item:\"minecraft:tripwire_hook\",hx:"),
            "{line}"
        );
        // The choice's right answer is step 3's button; the others only nudge.
        assert!(line.contains(r#"{"action":{"command":"trigger arnis.story.ctl set 13","type":"run_command"},"label":"Old"}"#), "{line}");
        assert_eq!(line.matches("arnis.story.ctl set 8").count(), 2, "{line}");
        assert!(
            line.contains("kind:\"choice\",wrong:\"Hmm, no.\""),
            "{line}"
        );
        assert!(line.contains("gift:\"minecraft:cake\""), "{line}");
    }

    #[test]
    fn the_hunt_splits_everyone_up_at_the_step_before_the_ending() {
        let s = story();
        let text = s.plan_function();
        let line = line_of(&text, "t_one");
        assert!(
            line.contains("hunt_step:4,arts:3,hunt:[{k:0,label:\"A\",item:\"minecraft:paper\""),
            "{line}"
        );
        assert!(line.contains("Let's split up!"), "{line}");
        assert!(!line_of(&text, "t_ep1").contains("Let's split up!"));
    }

    #[test]
    fn a_name_with_quotes_stays_valid_snbt() {
        let text = story().plan_function();
        if let Some(line) = text.lines().find(|l| l.contains("Bakery")) {
            assert!(line.contains(r#"Rosa's \"Best\" Bakery"#), "{line}");
        }
    }

    #[test]
    fn every_story_has_a_hidden_advancement() {
        let s = story();
        let advs = s.advancements();
        assert_eq!(advs.len(), 1 + s.run.len());
        for (path, adv) in advs.iter().skip(1) {
            assert!(s.run.iter().any(|p| &p.story.key == path));
            assert_eq!(adv["display"]["hidden"], true, "{path}");
            assert_eq!(adv["parent"], "arnis:story/root");
        }
    }

    #[test]
    fn errands_join_places_80_to_600_blocks_apart() {
        let s = story();
        assert!(!s.errands.is_empty());
        for &(a, b) in &s.errands {
            let d = dist(s.places[a].place.anchor, s.places[b].place.anchor);
            assert!(ERRAND_REACH.contains(&d), "{d}");
        }
    }
}
