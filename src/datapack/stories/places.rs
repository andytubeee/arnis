//! The named places a story can send players to, and what each is good for.

use crate::coordinate_system::cartesian::XZBBox;
use crate::datapack::discoveries::{classify, outline, Category, Place};
use crate::osm_parser::ProcessedElement;

/// Most places a story picks from, best first.
const MAX_PLACES: usize = 200;
/// Two places closer than this, in blocks, are one stop.
const MIN_APART: i32 = 12;

/// What a place can stand for in a story.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Park,
    FoodShop,
    School,
    Viewpoint,
    Faith,
    Bridge,
    Water,
    Station,
    Civic,
    Culture,
    Landmark,
    /// Any named place, for stories that only need somewhere to go.
    Any,
}

pub struct StoryPlace {
    pub place: Place,
    /// Its role, then `Role::Any`.
    pub roles: Vec<Role>,
    /// Where a search step hides its thing: a few steps off the street, in the open.
    pub nook: (i32, i32),
}

/// How far a nook may be from the street spot, in blocks.
const NOOK: std::ops::RangeInclusive<i32> = 3..=8;

impl StoryPlace {
    /// Moves the place onto the nearest street a player can walk to and finds its nook;
    /// false when no street is near, and a story must not send anyone there.
    pub fn settle(
        &mut self,
        open: impl Fn(i32, i32) -> bool,
        walkable: impl Fn(i32, i32) -> bool,
        ground_y: impl Fn(i32, i32) -> i32,
    ) -> bool {
        if !self
            .place
            .settle_on(|x, z| open(x, z) && walkable(x, z), &ground_y)
        {
            return false;
        }
        self.nook = nook(self.place.anchor, &open, &walkable);
        true
    }
}

/// An open cell off the street near `at`, with open ground all round it and nothing built
/// on the straight way there, so a hidden thing is a few steps away and never indoors.
/// The street spot itself when there is none.
pub fn nook(
    at: (i32, i32),
    open: impl Fn(i32, i32) -> bool,
    street: impl Fn(i32, i32) -> bool,
) -> (i32, i32) {
    let clear = |x: i32, z: i32| (-1..=1).all(|dx| (-1..=1).all(|dz| open(x + dx, z + dz)));
    let reach = |x: i32, z: i32| {
        let n = (x - at.0).abs().max((z - at.1).abs());
        (0..=n).all(|i| {
            let f = i as f64 / n.max(1) as f64;
            let (px, pz) = (
                at.0 + ((x - at.0) as f64 * f).round() as i32,
                at.1 + ((z - at.1) as f64 * f).round() as i32,
            );
            open(px, pz)
        })
    };
    for r in NOOK {
        for d in -r..=r {
            for (x, z) in [
                (at.0 + d, at.1 - r),
                (at.0 + r, at.1 + d),
                (at.0 - d, at.1 + r),
                (at.0 - r, at.1 - d),
            ] {
                if !street(x, z) && clear(x, z) && reach(x, z) {
                    return (x, z);
                }
            }
        }
    }
    at
}

fn role(element: &ProcessedElement) -> Option<(Role, Category)> {
    let tags = element.tags();
    let get = |k: &str| tags.get(k).map(String::as_str);
    let everyday = match (get("amenity"), get("shop"), get("leisure"), get("landuse")) {
        (Some("school" | "kindergarten" | "college"), ..) => Some(Role::School),
        (Some("cafe" | "marketplace" | "pub"), ..) => Some(Role::FoodShop),
        (Some("post_office" | "fire_station"), ..) => Some(Role::Civic),
        (_, Some("bakery" | "supermarket" | "convenience" | "greengrocer" | "deli"), ..) => {
            Some(Role::FoodShop)
        }
        (_, _, Some("playground"), _) => Some(Role::Park),
        (_, _, _, Some("farmland" | "farmyard")) => Some(Role::FoodShop),
        _ => None,
    };
    if let Some(r) = everyday {
        return Some((r, Category::Civic));
    }
    // The mountains' own stops, so a story runs where the trails lead past no town.
    let mountain = match (get("tourism"), get("natural"), get("mountain_pass")) {
        (Some("alpine_hut" | "wilderness_hut"), ..) => Some(Role::FoodShop),
        // Lodges are towns' hotels too, so only somewhere to go, never an errand's shop.
        (Some("guest_house" | "hostel"), ..) => Some(Role::Any),
        (Some("camp_site"), ..) => Some(Role::Park),
        (_, Some("glacier" | "saddle" | "volcano"), _) | (.., Some("yes")) => Some(Role::Viewpoint),
        _ => None,
    };
    if let Some(r) = mountain {
        return Some((r, Category::Nature));
    }
    let (category, label) = classify(element)?;
    let r = match category {
        Category::Nature if matches!(label, "Beach" | "Lake") => Role::Water,
        Category::Nature => Role::Park,
        Category::Viewpoint => Role::Viewpoint,
        Category::Faith => Role::Faith,
        Category::Bridge => Role::Bridge,
        Category::Transport => Role::Station,
        Category::Civic => Role::Civic,
        Category::Culture => Role::Culture,
        Category::Landmark | Category::Sport => Role::Landmark,
    };
    Some((r, category))
}

/// Named places with a role, best first, none within `MIN_APART` of a better one.
pub fn collect(elements: &[ProcessedElement], world: &XZBBox) -> Vec<StoryPlace> {
    let mut found: Vec<StoryPlace> = elements
        .iter()
        .filter_map(|element| {
            let tags = element.tags();
            let name = tags
                .get("name")
                .map(|n| n.trim())
                .filter(|n| !n.is_empty())?;
            let (role, category) = role(element)?;
            let ((min, max), point) = outline(element)?;
            let pad = if point { 8 } else { 3 };
            let min = (
                (min.0 - pad).max(world.min_x()),
                (min.1 - pad).max(world.min_z()),
            );
            let max = (
                (max.0 + pad).min(world.max_x()),
                (max.1 + pad).min(world.max_z()),
            );
            if min.0 > max.0 || min.1 > max.1 {
                return None;
            }
            let notable = tags.contains_key("wikidata") || tags.contains_key("wikipedia");
            let area = ((max.0 - min.0 + 1) as f64 * (max.1 - min.1 + 1) as f64).log2();
            Some(StoryPlace {
                nook: (0, 0),
                place: Place {
                    name: name.to_string(),
                    category,
                    label: String::new(),
                    min,
                    max,
                    anchor: ((min.0 + max.0) / 2, (min.1 + max.1) / 2),
                    ground_y: None,
                    notable,
                    score: if notable { 100 } else { 0 } + area as i32,
                },
                roles: vec![role, Role::Any],
            })
        })
        .collect();
    found.sort_by_key(|p| std::cmp::Reverse(p.place.score));
    let mut kept: Vec<StoryPlace> = Vec::new();
    for p in found {
        let (x, z) = p.place.anchor;
        let close = |k: &StoryPlace| {
            (k.place.anchor.0 - x).abs() < MIN_APART && (k.place.anchor.1 - z).abs() < MIN_APART
        };
        if !kept.iter().any(close) {
            kept.push(p);
            if kept.len() == MAX_PLACES {
                break;
            }
        }
    }
    kept
}

/// A one-block place with `role`, for tests.
#[cfg(test)]
pub(crate) fn test_place(name: &str, x: i32, z: i32, role: Role) -> StoryPlace {
    StoryPlace {
        nook: (x + 4, z),
        place: Place {
            name: name.to_string(),
            category: Category::Civic,
            label: String::new(),
            min: (x, z),
            max: (x, z),
            anchor: (x, z),
            ground_y: Some(70),
            notable: false,
            score: 0,
        },
        roles: vec![role, Role::Any],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coordinate_system::cartesian::XZBBox;
    use crate::datapack::discoveries::tests::square;

    fn world() -> XZBBox {
        XZBBox::rect_from_min_max(0, 0, 1000, 1000).unwrap()
    }

    #[test]
    fn a_school_and_a_bakery_get_their_roles() {
        let elements = vec![
            square(
                1,
                10,
                10,
                30,
                &[("amenity", "school"), ("name", "Jarvis Collegiate")],
            ),
            square(
                2,
                200,
                200,
                10,
                &[("shop", "bakery"), ("name", "Nova Bakery")],
            ),
            square(
                3,
                400,
                400,
                60,
                &[("leisure", "park"), ("name", "Grange Park")],
            ),
        ];
        let places = collect(&elements, &world());
        let roles = |n: &str| {
            places
                .iter()
                .find(|p| p.place.name == n)
                .unwrap()
                .roles
                .clone()
        };
        assert_eq!(roles("Jarvis Collegiate"), vec![Role::School, Role::Any]);
        assert_eq!(roles("Nova Bakery"), vec![Role::FoodShop, Role::Any]);
        assert_eq!(roles("Grange Park"), vec![Role::Park, Role::Any]);
    }

    #[test]
    fn mountain_huts_camps_glaciers_and_passes_are_story_stops() {
        let elements = vec![
            square(
                1,
                10,
                10,
                10,
                &[("tourism", "alpine_hut"), ("name", "Hörnli Hut")],
            ),
            square(
                2,
                100,
                100,
                10,
                &[("tourism", "guest_house"), ("name", "Namaste Lodge")],
            ),
            square(
                3,
                200,
                200,
                60,
                &[("tourism", "camp_site"), ("name", "Base Camp")],
            ),
            square(
                4,
                400,
                400,
                60,
                &[("natural", "glacier"), ("name", "Khumbu Glacier")],
            ),
            square(
                5,
                600,
                600,
                10,
                &[("mountain_pass", "yes"), ("name", "Kongma La")],
            ),
        ];
        let places = collect(&elements, &world());
        let role = |n: &str| {
            places
                .iter()
                .find(|p| p.place.name == n)
                .map(|p| p.roles[0])
        };
        assert_eq!(role("Hörnli Hut"), Some(Role::FoodShop));
        assert_eq!(role("Namaste Lodge"), Some(Role::Any));
        assert_eq!(role("Base Camp"), Some(Role::Park));
        assert_eq!(role("Khumbu Glacier"), Some(Role::Viewpoint));
        assert_eq!(role("Kongma La"), Some(Role::Viewpoint));
    }

    #[test]
    fn a_place_off_every_street_is_dropped_and_one_near_a_street_stands_on_it() {
        // A street along z = 20; a building covering x 0..10, z 0..10.
        let open = |x: i32, z: i32| !((0..10).contains(&x) && (0..10).contains(&z));
        let street = |_x: i32, z: i32| z == 20;
        let mut near = test_place("Near", 5, 5, Role::Park);
        assert!(near.settle(open, street, |_, _| 64));
        assert_eq!(near.place.anchor.1, 20);
        assert_eq!(near.place.ground_y, Some(64));
        let mut far = test_place("Far", 5, 5, Role::Park);
        assert!(!far.settle(open, |_, _| false, |_, _| 64));
    }

    #[test]
    fn a_nook_is_off_the_street_in_the_open_and_never_through_a_wall() {
        // Street along z = 0; buildings fill z = -10..=-1 north of it, except a gap at x = 0.
        let open = |x: i32, z: i32| z >= 0 || x == 0 && z > -12 || z < -10;
        let street = |_x: i32, z: i32| z == 0;
        let (x, z) = nook((0, 0), open, street);
        assert!(
            z > 0,
            "{x},{z} should be south, off the street, not behind the wall"
        );
        assert!((3..=8).contains(&(x.abs().max(z.abs()))));
        // Nowhere off the street to hide it: the street spot.
        assert_eq!(nook((0, 0), |_, z| z == 0, |_, _| true), (0, 0));
    }

    #[test]
    fn unnamed_and_duplicate_places_are_skipped() {
        let elements = vec![
            square(1, 10, 10, 10, &[("amenity", "cafe")]),
            square(2, 100, 100, 10, &[("amenity", "cafe"), ("name", "Tim's")]),
            square(3, 104, 104, 10, &[("amenity", "cafe"), ("name", "Tim's")]),
        ];
        let places = collect(&elements, &world());
        assert_eq!(places.len(), 1);
        assert_eq!(places[0].place.name, "Tim's");
    }
}
