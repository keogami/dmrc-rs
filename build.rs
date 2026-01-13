use std::{collections::BTreeMap, env, fs, path::Path};

use anyhow::{Context, anyhow};
use gtfs_structures::Gtfs;
use raptor::{Timetable, gtfs::GtfsTimetable};
use rayon::prelude::*;
use types::{Journey, Journeys};

fn main() -> anyhow::Result<()> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
    let path = Path::new(&manifest_dir).join("dmrc-gtfs");

    println!("cargo::rerun-if-changed=dmrc-gtfs");

    let journeys = pre_compute(&path).context("Couldn't precompute the journeys")?;
    let bytes = journeys.as_bytes();

    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("journeys.rkyv");
    fs::write(&dest_path, &bytes)?;

    Ok(())
}

fn pre_compute(path: impl AsRef<Path>) -> anyhow::Result<Journeys> {
    let path = path.as_ref();
    let gtfs = Gtfs::from_path(path)?;
    let timetable = GtfsTimetable::new(&gtfs);

    // still keeping nmrc's stations cuz im lazy tbh. removing them requires either:
    // - remove from source, which means keeping a patch everytime we update our copy of dmrc's static gtfs
    // - or, update gtfs after parsing, which i might actually do later
    let mut stops: Vec<_> = gtfs.stops.keys().cloned().collect();
    stops.sort();
    let mut routes: Vec<_> = gtfs.routes.keys().cloned().collect();
    routes.sort();

    // dmrc's gtfs includes nmrc stops as well, for whatever reason.
    // but no trips actually touch these stops, so they just waste space
    let mut actual_stops = stops.clone();
    actual_stops
        .retain(|item| !(500..=520).contains(&item.parse().expect("dmrc IDs to be valid numbers")));

    if actual_stops.is_empty() {
        println!("cargo::warning=No stops found");
    }

    // Collect all (ps, pt) pairs, filtering out ps == pt
    let pairs: Vec<_> = actual_stops
        .iter()
        .flat_map(|ps| actual_stops.iter().map(move |pt| (ps, pt)))
        .filter(|(ps, pt)| ps != pt)
        .collect();

    let journeys: BTreeMap<_, _> = pairs
        .par_iter()
        .map(|(ps, pt)| {
            let ps_idx = timetable.lookup_stop(ps).expect("invalid ps") as u32;
            let pt_idx = timetable.lookup_stop(pt).expect("invalid pt") as u32;

            // maybe a tuple might give similar packing?
            // TODO: confirm this is optimal
            let key: u32 = ps_idx << 16 | pt_idx;

            let plan: Vec<Journey> = timetable
                .raptor(4, 17120, ps_idx as usize, pt_idx as usize)
                .into_iter()
                .map(|j| Journey {
                    arrival: j.arrival as u16,
                    plan: j.plan.into_iter().map(|(r, s)| (r as _, s as _)).collect(),
                })
                .collect();

            (key, plan)
        })
        .collect();

    Ok(Journeys {
        stop_ids: stops,
        route_ids: routes,
        journeys,
    })
}
