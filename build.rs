use std::{collections::BTreeMap, env, fs, path::Path};

use anyhow::Context;
use gtfs_structures::Gtfs;
use raptor::{Timetable, gtfs::GtfsTimetable};
use rayon::prelude::*;

// Build-time type definitions (must match src/types.rs)
#[derive(rkyv::Serialize, rkyv::Archive)]
struct Journeys {
    journeys: BTreeMap<u32, Vec<Journey>>,
    stops: Vec<StopDetails>,
    routes: Vec<RouteDetails>,
}

#[derive(rkyv::Serialize, rkyv::Archive)]
struct Journey {
    arrival: u16,
    plan: Vec<(u16, u16)>,
}

#[derive(rkyv::Serialize, rkyv::Archive)]
struct StopDetails {
    name: String,
    id: String,
}

#[derive(rkyv::Serialize, rkyv::Archive)]
struct RouteDetails {
    short_name: String,
    long_name: String,
    id: String,
    color: [u8; 3],
}

impl Journeys {
    fn as_bytes(&self) -> Box<[u8]> {
        rkyv::to_bytes::<rkyv::rancor::Panic>(self)
            .unwrap()
            .into_boxed_slice()
    }
}

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

fn collect_stop_details(gtfs: &Gtfs) -> Vec<StopDetails> {
    let mut stops: Vec<_> = gtfs
        .stops
        .iter()
        .map(|(id, stop)| StopDetails {
            name: stop.name.clone().unwrap_or_default(),
            id: id.clone(),
        })
        .collect();

    stops.sort_by_cached_key(|s| s.id.clone().into_boxed_str());

    stops
}

fn collect_route_details(gtfs: &Gtfs) -> Vec<RouteDetails> {
    let mut routes: Vec<_> = gtfs
        .routes
        .iter()
        .map(|(id, route)| RouteDetails {
            short_name: route.short_name.clone().unwrap_or_default(),
            long_name: route.long_name.clone().unwrap_or_default(),
            id: id.clone(),
            color: {
                let color = route.color.unwrap_or_default();

                [color.r, color.g, color.b]
            },
        })
        .collect();

    routes.sort_by_cached_key(|s| s.id.clone().into_boxed_str());

    routes
}

fn pre_compute(path: impl AsRef<Path>) -> anyhow::Result<Journeys> {
    let path = path.as_ref();
    let gtfs = Gtfs::from_path(path)?;
    let timetable = GtfsTimetable::new(&gtfs);

    let stops = collect_stop_details(&gtfs);
    let routes = collect_route_details(&gtfs);

    // dmrc's gtfs includes nmrc stops as well, for whatever reason.
    // but no trips actually touch these stops, so they just waste space
    let mut actual_stops: Vec<_> = stops.iter().map(|s| s.id.clone()).collect();
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

            let key: u32 = ps_idx << 16 | pt_idx;

            let departure: usize = 19 * 3600 + 15 * 60;

            let plan: Vec<Journey> = timetable
                .raptor(4, departure as _, ps_idx as usize, pt_idx as usize)
                .into_iter()
                .map(|j| Journey {
                    arrival: (j.arrival - departure) as _,
                    plan: j.plan.into_iter().map(|(r, s)| (r as _, s as _)).collect(),
                })
                .collect();

            (key, plan)
        })
        .collect();

    Ok(Journeys {
        stops,
        routes,
        journeys,
    })
}
