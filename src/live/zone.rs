//! The game zone: the area around the start that a new live world builds whole, as its first
//! One World area, so it carries the world's data pack. Stories, police, people and the map's
//! street graph are planned over land that exists, and Live grows the world past it.

use super::tiles::value_name;
use crate::args::Args;
use crate::coordinate_system::geographic::LLBBox;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;

/// Half the side of the zone, in metres: room for a hundred story walks.
const HALF_SIDE_M: f64 = 1500.0;
const METRES_PER_DEGREE: f64 = 111_320.0;

/// The flags of a zone run, or `None` when no game mode needs one.
pub fn flags(a: &Args, manifest: &crate::one_world::Manifest) -> Option<Vec<String>> {
    let modes: Vec<String> = crate::args::pack_extras(a)
        .into_iter()
        .filter(|(on, _)| *on)
        .map(|(_, flag)| flag.to_string())
        .collect();
    if modes.is_empty() {
        return None;
    }
    let mut f: Vec<String> = super::tiles::tile_flags(a, manifest)
        .into_iter()
        .filter(|f| f != "--live-tile")
        .collect();
    // The first area writes the world's settings, so it gets the ones the world was made with.
    f.push(format!("--gamemode={}", value_name(&a.gamemode)));
    f.push(format!("--world-time={}", a.world_time));
    f.push(format!("--world-type={}", value_name(&a.world_type)));
    f.push(format!("--mob-spawning={}", value_name(&a.mob_spawning)));
    f.push("--progress=json".to_string());
    f.extend(modes);
    Some(f)
}

/// A square of `HALF_SIDE_M` each way around (`lat`, `lon`).
pub fn bbox(lat: f64, lon: f64) -> Result<LLBBox, String> {
    let dlat = HALF_SIDE_M / METRES_PER_DEGREE;
    let dlon = HALF_SIDE_M / (METRES_PER_DEGREE * lat.to_radians().cos().max(0.01));
    LLBBox::new(lat - dlat, lon - dlon, lat + dlat, lon + dlon)
}

/// The percentage a `--progress json` line reports, if it reports one.
fn progress_of(line: &str) -> Option<f64> {
    if !line.starts_with("{\"v\":") {
        return None;
    }
    let v: serde_json::Value = serde_json::from_str(line).ok()?;
    v.get("progress")?.as_f64()
}

/// Builds the zone into the world, reporting each percentage, until done or `stop` says so.
pub fn build(
    saves_dir: &Path,
    world_name: &str,
    zone: &LLBBox,
    flags: &[String],
    progress: &dyn Fn(f64),
    stop: &dyn Fn() -> bool,
) -> Result<(), String> {
    let exe = std::env::current_exe().map_err(|e| format!("Cannot find the arnis program: {e}"))?;
    let mut cmd = Command::new(exe);
    cmd.arg("--one-world")
        .arg(format!("--output-dir={}", saves_dir.display()))
        .arg(format!("--world-name={world_name}"))
        .arg(format!(
            "--bbox={},{},{},{}",
            zone.min().lat(),
            zone.min().lng(),
            zone.max().lat(),
            zone.max().lng()
        ))
        .args(flags)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit());
    super::server::detach(&mut cmd);
    let mut child = cmd
        .spawn()
        .map_err(|e| format!("Could not start building the game zone: {e}"))?;
    let out = child
        .stdout
        .take()
        .ok_or("The game zone build has no output")?;
    let (tx, rx) = std::sync::mpsc::channel();
    let reader = std::thread::spawn(move || {
        for line in BufReader::new(out).lines().map_while(Result::ok) {
            match progress_of(&line) {
                Some(p) => drop(tx.send(p)),
                None => println!("[zone] {line}"),
            }
        }
    });
    let status = loop {
        if let Some(p) = rx.try_iter().last() {
            progress(p / 100.0);
        }
        if stop() {
            let _ = child.kill();
            let _ = child.wait();
            return Err("Cancelled".to_string());
        }
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) => std::thread::sleep(Duration::from_millis(200)),
            Err(e) => return Err(e.to_string()),
        }
    };
    let _ = reader.join();
    if status.success() {
        Ok(())
    } else {
        Err("The game zone around the start could not be built. Check your internet connection and try again.".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    fn manifest() -> crate::one_world::Manifest {
        serde_json::from_value(serde_json::json!({
            "version": 3, "created_with": "test", "created_at": 0, "origin_lat": 43.45,
            "origin_lon": -80.49, "scale": 1.0, "ground_level": -62, "terrain": true,
            "disable_height_limit": true, "aws_only_elevation": false, "elevation": null,
            "next_area_id": 1, "areas": []
        }))
        .unwrap()
    }

    #[test]
    fn a_zone_is_a_first_area_with_the_game_modes_and_the_world_settings() {
        let plain = Args::parse_from(["arnis", "--one-world", "--live", "--output-dir=."]);
        assert!(
            flags(&plain, &manifest()).is_none(),
            "no game mode, no zone"
        );

        let a = Args::parse_from([
            "arnis",
            "--one-world",
            "--live",
            "--output-dir=.",
            "--city-stories",
            "--crime-spree",
            "--gamemode=survival",
            "--winter",
        ]);
        let mut argv = vec![
            "arnis".to_string(),
            "--one-world".to_string(),
            "--bbox=1,2,3,4".to_string(),
            "--output-dir=.".to_string(),
        ];
        argv.extend(flags(&a, &manifest()).unwrap());
        let back = Args::try_parse_from(argv).unwrap();
        assert!(back.city_stories && back.crime_spree && back.winter);
        assert!(!back.live.tile, "a zone is a whole area, not a tile");
        assert_eq!(back.gamemode, a.gamemode);
        crate::args::validate_args(&back).unwrap();
    }

    #[test]
    fn the_zone_is_three_kilometres_square() {
        let z = bbox(43.45, -80.49).unwrap();
        let (h, w) = (
            (z.max().lat() - z.min().lat()) * METRES_PER_DEGREE,
            (z.max().lng() - z.min().lng()) * METRES_PER_DEGREE * 43.45f64.to_radians().cos(),
        );
        assert!(
            (h - 3000.0).abs() < 1.0 && (w - 3000.0).abs() < 1.0,
            "{h} x {w}"
        );
    }

    #[test]
    fn progress_is_read_from_the_json_lines_only() {
        assert_eq!(
            progress_of(r#"{"v":1,"type":"progress","progress":35.5}"#),
            Some(35.5)
        );
        assert_eq!(
            progress_of(r#"{"v":1,"type":"phase","name":"x","progress":null}"#),
            None
        );
        assert_eq!(progress_of("Installed the data pack. progress 3"), None);
    }
}
