use std::collections::BTreeMap;

use nucleo_matcher::{Config, Matcher, pattern::Pattern};

#[derive(rkyv::Serialize, Debug, rkyv::Archive)]
#[rkyv(derive(Debug))]
pub struct Journeys {
    pub journeys: BTreeMap<u32, Vec<Journey>>,
    pub stops: Vec<StopDetails>,
    pub routes: Vec<RouteDetails>,
}

#[derive(rkyv::Serialize, Debug, rkyv::Archive)]
#[rkyv(derive(Debug))]
pub struct Journey {
    pub arrival: u16,
    pub plan: Vec<(u16, u16)>,
}

#[derive(rkyv::Serialize, Debug, rkyv::Archive)]
#[rkyv(derive(Debug))]
pub struct StopDetails {
    pub name: String,
    pub id: String,
}

#[derive(rkyv::Serialize, Debug, rkyv::Archive)]
#[rkyv(derive(Debug))]
pub struct RouteDetails {
    pub short_name: String,
    pub long_name: String,
    pub id: String,
    pub color: [u8; 3],
}

impl Journeys {
    pub unsafe fn access_unchecked(bytes: &[u8]) -> &ArchivedJourneys {
        unsafe { rkyv::access_unchecked(bytes) }
    }
}

impl ArchivedJourneys {
    /// Fuzzy search stops by name, returning (index, score) pairs
    pub fn fuzzy_search(&self, pattern: &str) -> Vec<(usize, u32)> {
        let mut matcher = Matcher::new(Config::DEFAULT.match_paths());
        let pattern = Pattern::parse(
            pattern,
            nucleo_matcher::pattern::CaseMatching::Smart,
            nucleo_matcher::pattern::Normalization::Smart,
        );

        let matches = pattern.match_list(
            self.stops.iter().map(|s| s.name.as_str()),
            &mut matcher,
        );

        matches
            .into_iter()
            .filter_map(|(name, score)| {
                self.stops.iter().position(|s| s.name == name).map(|i| (i, score))
            })
            .collect()
    }
}
