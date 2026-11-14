//! The named places a story can send players to, and what each is good for.

use crate::coordinate_system::cartesian::XZBBox;
use crate::datapack::discoveries::{classify, outline, Category, Place};
use crate::osm_parser::ProcessedElement;

/// Most places a story picks from, best first.
const MAX_PLACES: usize = 200;
/// Two places closer than this, in blocks, are one stop.
const MIN_APART: i32 = 12;

/// What a place can stand for in a story.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
            let name = tags.get("name").map(|n| n.trim()).filter(|n| !n.is_empty())?;
            let (role, category) = role(element)?;
            let ((min, max), point) = outline(element)?;
            let pad = if point { 8 } else { 3 };
            let min = ((min.0 - pad).max(world.min_x()), (min.1 - pad).max(world.min_z()));
            let max = ((max.0 + pad).min(world.max_x()), (max.1 + pad).min(world.max_z()));
            if min.0 > max.0 || min.1 > max.1 {
                return None;
            }
            let notable = tags.contains_key("wikidata") || tags.contains_key("wikipedia");
            let area = ((max.0 - min.0 + 1) as f64 * (max.1 - min.1 + 1) as f64).log2();
            Some(StoryPlace {
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
            square(1, 10, 10, 30, &[("amenity", "school"), ("name", "Jarvis Collegiate")]),
            square(2, 200, 200, 10, &[("shop", "bakery"), ("name", "Nova Bakery")]),
            square(3, 400, 400, 60, &[("leisure", "park"), ("name", "Grange Park")]),
        ];
        let places = collect(&elements, &world());
        let roles = |n: &str| places.iter().find(|p| p.place.name == n).unwrap().roles.clone();
        assert_eq!(roles("Jarvis Collegiate"), vec![Role::School, Role::Any]);
        assert_eq!(roles("Nova Bakery"), vec![Role::FoodShop, Role::Any]);
        assert_eq!(roles("Grange Park"), vec![Role::Park, Role::Any]);
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
