//! The fifteen storylines, and fitting one to the map's places.

use super::places::{Role, StoryPlace};
use rand::seq::IndexedRandom;
use rand::Rng;
use Role::*;

/// A hop longer than this, in blocks, is taken only when no closer place fits.
const HOP: i64 = 600;
/// The fewest places a story needs.
const MIN_STOPS: usize = 3;

/// One stop: who waits there, the roles its place may have (best first), and what they say.
/// `{next}` in the line is the next stop's name.
pub struct Step {
    pub roles: &'static [Role],
    pub npc: &'static str,
    pub line: &'static str,
}

pub struct Storyline {
    pub key: &'static str,
    pub title: &'static str,
    pub steps: &'static [Step],
    pub finale: Step,
}

/// The storyline this world got, and its stops as indices into the places: the steps it
/// uses, then the finale. A short Postcard Tour uses only its first steps.
pub struct Plan {
    pub line: &'static Storyline,
    pub stops: Vec<usize>,
}

impl Plan {
    /// The steps in play, finale last, each with its stop.
    pub fn steps(&self) -> impl Iterator<Item = (&'static Step, usize)> + '_ {
        let n = self.stops.len() - 1;
        self.line.steps[..n]
            .iter()
            .chain(std::iter::once(&self.line.finale))
            .zip(self.stops.iter().copied())
    }
}

const fn step(roles: &'static [Role], npc: &'static str, line: &'static str) -> Step {
    Step { roles, npc, line }
}

pub static STORYLINES: [Storyline; 15] = [
    Storyline {
        key: "lost_dog",
        title: "The Lost Dog",
        steps: &[
            step(&[Park, Any], "Mrs. Okafor", "My dog Biscuit slipped his lead! Someone saw a scruffy dog heading for {next}."),
            step(&[FoodShop, Any], "Sam the baker", "A dog? He begged for a crust here and ran off towards {next}."),
            step(&[School, Civic, Any], "A school janitor", "He dug up the flower bed and bolted. Kids saw him going to {next}."),
            step(&[Water, Bridge, Park], "A fisher", "Wet paws, muddy nose, happy as anything. He trotted off to {next}."),
        ],
        finale: step(&[Park, Any], "Biscuit", "There he is, tail going like mad. Mrs. Okafor says he picked you, so he's yours now."),
    },
    Storyline {
        key: "time_capsule",
        title: "Time Capsule",
        steps: &[
            step(&[Culture, Civic, Any], "The librarian", "This old map fell out of a 1985 yearbook. The first mark is at {next}."),
            step(&[School, Any], "A retired teacher", "My class buried a capsule! I only remember we met up after at {next}."),
            step(&[FoodShop, Any], "The owner", "Oh, those kids! They argued here for an hour, then marched to {next}."),
            step(&[Park, Landmark, Any], "A dog walker", "There's an old marker post by the path. The map's last X points to {next}."),
        ],
        finale: step(&[Landmark, Park, Any], "The capsule", "A rusty tin full of letters from 1985. Everyone takes one to read."),
    },
    Storyline {
        key: "surprise_party",
        title: "Surprise Party",
        steps: &[
            step(&[Civic, Any], "Priya", "It's Grandpa Lou's 90th and nothing's ready! Can you get the cake from {next}?"),
            step(&[FoodShop, Any], "The baker", "One giant cake, done. Priya also wanted music, try the busker at {next}."),
            step(&[Station, Park, Any], "A busker", "A party? I'm in. You'll want fireworks, my cousin keeps some at {next}."),
            step(&[Civic, Landmark, Any], "Cousin Dee", "Fireworks, check. Lou loves the view, so the party is at {next}."),
        ],
        finale: step(&[Viewpoint, Park, Landmark], "Grandpa Lou", "SURPRISE! Lou nearly drops his hat. The whole street came to watch the fireworks."),
    },
    Storyline {
        key: "band_reunion",
        title: "Band Reunion",
        steps: &[
            step(&[Culture, Any], "Old Ray", "The Night Owls haven't played in 30 years. Our drummer works at {next}."),
            step(&[Station, Any], "Dot the drummer", "Ray wants a reunion? About time. Our bassist busks near {next}."),
            step(&[Station, Park, Any], "Lenny on bass", "I still have the riffs. Our singer runs a little place at {next}."),
            step(&[FoodShop, Any], "Marisol", "Fine, one show! Tell everyone to meet at {next}."),
        ],
        finale: step(&[Park, Civic, Landmark], "The Night Owls", "Count it in! The Night Owls play their comeback show, and the crowd sings along."),
    },
    Storyline {
        key: "riddle_treasure",
        title: "Riddle Treasure",
        steps: &[
            step(&[Culture, Landmark, Any], "A note in a bottle", "\"Where old things sleep and new eyes wake, find the next clue at {next}.\""),
            step(&[Faith, Landmark, Any], "A chalk mark", "\"Bells and stones, quiet tones. Walk on to {next}.\""),
            step(&[Park, Any], "A carved bench", "\"Green and wide where children play, the river waits, go {next} way.\""),
            step(&[Water, Station, Any], "A soggy riddle", "\"Over the water, under the sky, the treasure waits at {next}.\""),
        ],
        finale: step(&[Bridge, Water, Landmark], "The treasure", "Tucked under the span, an old chest full of gold and diamonds. Not bad."),
    },
    Storyline {
        key: "friendly_ghost",
        title: "The Friendly Ghost",
        steps: &[
            step(&[Landmark, Culture, Any], "A faint voice", "Psst. I'm Edith, I'm a ghost, don't scream. I never sent my letters. The first goes to {next}."),
            step(&[Civic, Any], "The postmaster", "A letter from Edith? She was my grandmother's friend. Her sister lived near {next}."),
            step(&[Park, Any], "Edith's grand-niece", "Aunt Edith wrote to us? Thank you. She loved the place at {next}."),
            step(&[FoodShop, Any], "An old waiter", "Edith had tea here every Sunday. She said she'd rest at {next}."),
        ],
        finale: step(&[Faith, Landmark, Park], "Edith", "All my letters delivered. Thank you, friends. Edith smiles and fades into the light."),
    },
    Storyline {
        key: "stargazer",
        title: "Stargazer",
        steps: &[
            step(&[Culture, School, Any], "Dr. Lin", "A meteor shower tonight, but the town's too bright. Light my beacon at {next}."),
            step(&[Viewpoint, Landmark, Any], "A night owl", "Beacon one, lit! Dr. Lin wants another one at {next}."),
            step(&[Viewpoint, Park, Any], "A night walker", "Beacon two, glowing. The last one's at {next}."),
            step(&[Viewpoint, Landmark, Any], "Dr. Lin's assistant", "All lit! Dr. Lin is setting up at {next}. Hurry, it's starting."),
        ],
        finale: step(&[Viewpoint, Park, Landmark], "Dr. Lin", "Look up! The sky fills with falling stars. Make a wish."),
    },
    Storyline {
        key: "cat_mayor",
        title: "The Cat Mayor",
        steps: &[
            step(&[Civic, Any], "The town clerk", "Mayor Whiskers is missing before the ceremony! She was last seen near {next}."),
            step(&[FoodShop, Any], "The fishmonger", "Her Honour stole a sardine and strolled towards {next}."),
            step(&[Park, Any], "A gardener", "The mayor napped in my tulips, then went to {next}."),
            step(&[Faith, Culture, Any], "A sleepy caretaker", "A cat with a tiny sash? She's heading back towards {next}."),
        ],
        finale: step(&[Civic, Landmark, Any], "Mayor Whiskers", "Found her! Mayor Whiskers is crowned for another term, purring the whole time."),
    },
    Storyline {
        key: "message_bottle",
        title: "Message in a Bottle",
        steps: &[
            step(&[Water, Bridge, Any], "A beachcomber", "This bottle washed up: \"To Arthur, from Rose, 1962.\" Someone at {next} might know them."),
            step(&[Civic, Culture, Any], "An archivist", "Rose Bell! She moved away. Arthur still lives near {next}."),
            step(&[Park, Any], "Arthur", "Rose? After all these years? She always loved {next}. Could you find her?"),
            step(&[FoodShop, Any], "Rose", "Arthur kept my letter? Tell him to meet me at {next}."),
        ],
        finale: step(&[Bridge, Water, Viewpoint], "Rose and Arthur", "Two old friends meet again on the bridge, laughing like it was 1962."),
    },
    Storyline {
        key: "allay_courier",
        title: "The Allay Courier",
        steps: &[
            step(&[Park, Any], "A tiny blue allay", "La-la! I dropped my shiny things all over town. The first is at {next}!"),
            step(&[FoodShop, Any], "A shopkeeper", "A little blue thing took a cookie and left a bell. It flew to {next}."),
            step(&[Station, Any], "A conductor", "Found this ribbon on platform two. The allay went towards {next}."),
            step(&[School, Culture, Any], "A student", "It sang in our class and dropped a note block! It's waiting at {next}."),
        ],
        finale: step(&[Park, Landmark, Any], "The allay", "All its shiny things back! The allay twirls around you and decides to stay."),
    },
    Storyline {
        key: "founders_festival",
        title: "Founders' Festival",
        steps: &[
            step(&[Civic, Any], "The festival planner", "The Founders' Festival is tonight and the banners aren't up! Start at {next}."),
            step(&[Culture, Faith, Any], "A volunteer", "Banners up here, beautiful. The next lot go to {next}."),
            step(&[School, Any], "Kids from the art club", "We painted lanterns! Take some to {next}."),
            step(&[Station, Landmark, Any], "A busy conductor", "Banners and lanterns, perfect. Everyone's gathering at {next}."),
        ],
        finale: step(&[Park, Civic, Landmark], "The whole town", "Lanterns glow, music plays and the Founders' Festival begins. You made it happen."),
    },
    Storyline {
        key: "secret_garden",
        title: "The Secret Garden",
        steps: &[
            step(&[Park, Any], "Old Mrs. Greene", "My secret garden needs one seed from every park in town. Start at {next}."),
            step(&[Park, Any], "A park ranger", "Sunflower seeds, take some. The poppies grow best at {next}."),
            step(&[Park, Water, Any], "A birdwatcher", "Poppy seeds, here. Cornflowers? Ask at {next}."),
            step(&[FoodShop, School, Any], "A florist", "Cornflowers, and a little extra. Mrs. Greene's garden is hidden at {next}."),
        ],
        finale: step(&[Park, Landmark, Any], "Mrs. Greene", "You plant the seeds and the whole garden bursts into bloom."),
    },
    Storyline {
        key: "golems_heart",
        title: "The Golem's Heart",
        steps: &[
            step(&[Landmark, Civic, Any], "An old engineer", "This town once had an iron guardian. Its parts were scattered. One's at {next}."),
            step(&[Station, Any], "A rail worker", "A big iron arm in the lost and found! The other one's at {next}."),
            step(&[Bridge, Civic, Any], "A bridge keeper", "The second arm, here. The heart was kept safe at {next}."),
            step(&[Culture, Faith, Any], "A curator", "The golem's heart, still warm. Rebuild it at {next}."),
        ],
        finale: step(&[Civic, Landmark, Park], "The iron golem", "Clank by clank, the guardian stands again and gives you a poppy."),
    },
    Storyline {
        key: "picnic_bakeoff",
        title: "Picnic Bake-off",
        steps: &[
            step(&[FoodShop, Any], "Chef Amara", "The great picnic bake-off is today and I'm out of everything! Wheat first, from {next}."),
            step(&[FoodShop, Park, Any], "A farmer", "Wheat, freshly cut. Eggs are at {next}."),
            step(&[FoodShop, Any], "A grocer", "A dozen eggs, careful now. Sugar from {next}."),
            step(&[FoodShop, School, Any], "A cafe owner", "Sugar and a pinch of cinnamon. The picnic is at {next}!"),
        ],
        finale: step(&[Park, Water, Any], "Chef Amara", "Everyone shares cake on the grass. Amara declares it a tie, so everyone wins."),
    },
    Storyline {
        key: "postcard_tour",
        title: "The Postcard Tour",
        steps: &[
            step(&[Any], "A lost tourist", "I dropped all my postcards around town! The first one shows {next}."),
            step(&[Any], "A friendly local", "A postcard? It's a picture of {next}. Go on!"),
            step(&[Any], "A street sweeper", "Found another one here. This one shows {next}."),
            step(&[Any], "A cyclist", "Postcard! It's of {next}, not far."),
            step(&[Any], "A photographer", "Last postcard. The tourist is waiting at {next}."),
        ],
        finale: step(&[Any], "The tourist", "All my postcards! Here, take this painting of the town. Sign it, it's yours."),
    },
];

fn dist2(a: (i32, i32), b: (i32, i32)) -> i64 {
    let (dx, dz) = ((a.0 - b.0) as i64, (a.1 - b.1) as i64);
    dx * dx + dz * dz
}

/// The nearest unused place for the first of `roles` that has one within `HOP`, else the
/// nearest unused place for the first role that has any.
fn next_stop(
    places: &[StoryPlace],
    used: &[usize],
    from: (i32, i32),
    roles: &[Role],
) -> Option<usize> {
    let mut far = None;
    for role in roles {
        let nearest = (0..places.len())
            .filter(|i| !used.contains(i) && places[*i].roles.contains(role))
            .min_by_key(|&i| dist2(from, places[i].place.anchor));
        if let Some(i) = nearest {
            if dist2(from, places[i].place.anchor) <= HOP * HOP {
                return Some(i);
            }
            far.get_or_insert(i);
        }
    }
    far
}

/// `line`'s first `stops - 1` steps and its finale, each on a distinct place, starting
/// nearest the middle of the places.
fn fill(line: &Storyline, places: &[StoryPlace], stops: usize) -> Option<Vec<usize>> {
    let n = places.len().max(1) as i64;
    let sum = places.iter().fold((0i64, 0i64), |s, p| {
        (s.0 + p.place.anchor.0 as i64, s.1 + p.place.anchor.1 as i64)
    });
    let mut at = ((sum.0 / n) as i32, (sum.1 / n) as i32);
    let mut used = Vec::new();
    for step in line.steps[..stops - 1].iter().chain([&line.finale]) {
        let i = next_stop(places, &used, at, step.roles)?;
        at = places[i].place.anchor;
        used.push(i);
    }
    Some(used)
}

/// A storyline the map can carry, picked at random, or a Postcard Tour as long as there
/// are places for it; `None` below `MIN_STOPS` places.
pub fn pick(places: &[StoryPlace], rng: &mut impl Rng) -> Option<Plan> {
    let (tour, rest) = STORYLINES.split_last()?;
    let fitting: Vec<Plan> = rest
        .iter()
        .filter_map(|line| {
            Some(Plan {
                line,
                stops: fill(line, places, line.steps.len() + 1)?,
            })
        })
        .collect();
    let chosen = (0..fitting.len()).collect::<Vec<_>>().choose(rng).copied();
    if let Some(i) = chosen {
        return fitting.into_iter().nth(i);
    }
    let stops = places.len().min(tour.steps.len() + 1);
    (stops >= MIN_STOPS).then(|| Plan {
        line: tour,
        stops: fill(tour, places, stops).unwrap(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::datapack::stories::places::test_place as place;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    /// Three of every role, spread over a 900-block square.
    fn rich_map() -> Vec<StoryPlace> {
        use Role::*;
        let roles = [
            Park, FoodShop, School, Viewpoint, Faith, Bridge, Water, Station, Civic, Culture,
            Landmark,
        ];
        let mut out = Vec::new();
        for (i, r) in roles.iter().enumerate() {
            for k in 0..3 {
                let n = (i * 3 + k) as i32;
                out.push(place(
                    &format!("{r:?} {k}"),
                    (n * 97) % 900,
                    (n * 211) % 900,
                    *r,
                ));
            }
        }
        out
    }

    #[test]
    fn every_storyline_is_filled_with_distinct_places_on_a_rich_map() {
        let places = rich_map();
        for line in &STORYLINES {
            let stops = fill(line, &places, line.steps.len() + 1)
                .unwrap_or_else(|| panic!("{} not filled", line.key));
            assert_eq!(stops.len(), line.steps.len() + 1, "{}", line.key);
            let mut seen = stops.clone();
            seen.sort();
            seen.dedup();
            assert_eq!(seen.len(), stops.len(), "{} repeats a place", line.key);
        }
    }

    #[test]
    fn every_storyline_has_four_or_five_steps_naming_the_next_stop() {
        assert_eq!(STORYLINES.len(), 15);
        for line in &STORYLINES {
            assert!((4..=5).contains(&line.steps.len()), "{}", line.key);
            assert!(
                line.steps.iter().all(|s| s.line.contains("{next}")),
                "{}",
                line.key
            );
            assert!(!line.finale.line.contains("{next}"), "{}", line.key);
        }
    }

    #[test]
    fn a_sparse_map_gets_a_short_postcard_tour() {
        let places: Vec<_> = (0..4)
            .map(|i| place(&format!("P{i}"), i * 50, 0, Role::Any))
            .collect();
        let plan = pick(&places, &mut ChaCha8Rng::seed_from_u64(1)).unwrap();
        assert_eq!(plan.line.key, "postcard_tour");
        assert_eq!(plan.stops.len(), 4);
    }

    #[test]
    fn two_places_get_no_story() {
        let places: Vec<_> = (0..2)
            .map(|i| place(&format!("P{i}"), i * 50, 0, Role::Park))
            .collect();
        assert!(pick(&places, &mut ChaCha8Rng::seed_from_u64(1)).is_none());
    }

    #[test]
    fn hops_prefer_places_within_600_blocks() {
        use Role::*;
        // From the park in the middle, the near school wins over the far one.
        let places = vec![
            place("Park", 500, 500, Park),
            place("Far School", 500, 1400, School),
            place("Near School", 900, 500, School),
        ];
        assert_eq!(next_stop(&places, &[], (500, 500), &[School]), Some(2));
        assert_eq!(next_stop(&places, &[2], (500, 500), &[School]), Some(1));
    }

    #[test]
    fn the_pick_follows_the_seed() {
        let places = rich_map();
        let a = pick(&places, &mut ChaCha8Rng::seed_from_u64(7)).unwrap();
        let b = pick(&places, &mut ChaCha8Rng::seed_from_u64(7)).unwrap();
        assert_eq!(a.line.key, b.line.key);
        assert_eq!(a.stops, b.stops);
        assert_ne!(a.line.key, "postcard_tour");
    }
}
