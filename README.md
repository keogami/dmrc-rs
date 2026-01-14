# dmrc-rs

Rust SDK-esque crate for Delhi Metro (DMRC) route planning.

## How it works

All possible journeys on the DMRC network are **precomputed during build** using the RAPTOR algorithm and **embedded directly into your binary** via rkyv zero-copy serialization.

- No runtime API calls
- No external data files
- Instant route lookups

## Usage

```rust
use dmrc_rs::load_routes;

// read from the data section of _your_ binary
let data = load_routes();

// Fuzzy search stops by name
let matches = data.fuzzy_search("rajiv");

// Access precomputed journey data
// Key: (source_idx << 16) | target_idx
```

## Data

- Source: DMRC GTFS static feed
- Algorithm: RAPTOR (Range-based Algorithm for Public Transit Optimization)
- Serialization: rkyv (zero-copy deserialization)

## Status

**Work in progress.** API is a chaos, but the proof of concept is ready. Will be open for contributions soon.

## License

Apache-2.0
