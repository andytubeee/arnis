//! The street network, written into the world resource pack for the Arnis map mod, which
//! plans routes along it. Clients get the pack from the world or the server, so the mod
//! needs nothing on the server.

use crate::osm_parser::ProcessedElement;
use fnv::FnvHashMap;
use serde_json::{json, Value};

/// Where the mod reads it from.
pub const PATH: &str = "assets/arnis/roads.json";

/// Every street and path as runs of node indices; ways meeting at a node share it.
pub struct Graph {
    /// x, z of each node, flattened.
    nodes: Vec<i32>,
    /// A way's name ("" when it has none) and its nodes.
    ways: Vec<(String, Vec<u32>)>,
}

impl Graph {
    pub fn build(elements: &[ProcessedElement]) -> Self {
        let mut index: FnvHashMap<u64, u32> = FnvHashMap::default();
        let mut nodes = Vec::new();
        let mut ways = Vec::new();
        for e in elements {
            let ProcessedElement::Way(w) = e else {
                continue;
            };
            let Some(kind) = w.tags.get("highway") else {
                continue;
            };
            if w.nodes.len() < 2
                || matches!(
                    kind.as_str(),
                    "construction" | "proposed" | "razed" | "abandoned"
                )
            {
                continue;
            }
            let run = w
                .nodes
                .iter()
                .map(|n| {
                    *index.entry(n.id).or_insert_with(|| {
                        nodes.extend([n.x, n.z]);
                        (nodes.len() / 2 - 1) as u32
                    })
                })
                .collect();
            let name = w.tags.get("name").cloned().unwrap_or_default();
            ways.push((name, run));
        }
        Self { nodes, ways }
    }

    /// The file, with the named places a route can be set to by name.
    pub fn json<'a>(&self, places: impl IntoIterator<Item = (&'a str, (i32, i32))>) -> Value {
        let mut seen = std::collections::HashSet::new();
        let places: Vec<Value> = places
            .into_iter()
            .filter(|(n, _)| seen.insert(*n))
            .map(|(n, (x, z))| json!({ "n": n, "x": x, "z": z }))
            .collect();
        let ways: Vec<Value> = self
            .ways
            .iter()
            .map(|(n, p)| json!({ "n": n, "p": p }))
            .collect();
        json!({ "v": 1, "nodes": self.nodes, "ways": ways, "places": places })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::osm_parser::{ProcessedNode, ProcessedWay};
    use std::collections::HashMap;

    fn way(id: u64, highway: &str, name: &str, nodes: &[(u64, i32, i32)]) -> ProcessedElement {
        let mut tags = HashMap::from([("highway".to_string(), highway.to_string())]);
        if !name.is_empty() {
            tags.insert("name".into(), name.into());
        }
        ProcessedElement::Way(ProcessedWay {
            id,
            tags,
            nodes: nodes
                .iter()
                .map(|&(id, x, z)| ProcessedNode {
                    id,
                    tags: HashMap::new(),
                    x,
                    z,
                })
                .collect(),
        })
    }

    #[test]
    fn streets_meeting_at_a_node_share_it_and_unbuilt_ones_are_left_out() {
        let g = Graph::build(&[
            way(1, "residential", "King St", &[(10, 0, 0), (11, 50, 0)]),
            way(2, "footway", "", &[(11, 50, 0), (12, 50, 40)]),
            way(3, "construction", "Soon Rd", &[(12, 50, 40), (13, 90, 40)]),
        ]);
        let j = g.json([("Grange Park", (5, 6)), ("Grange Park", (7, 8))]);
        assert_eq!(j["nodes"], json!([0, 0, 50, 0, 50, 40]));
        assert_eq!(
            j["ways"],
            json!([{ "n": "King St", "p": [0, 1] }, { "n": "", "p": [1, 2] }])
        );
        assert_eq!(j["places"], json!([{ "n": "Grange Park", "x": 5, "z": 6 }]));
    }
}
