use std::collections::BTreeMap;

#[derive(rkyv::Serialize, Debug, rkyv::Archive)]
#[rkyv(derive(Debug))]
pub struct Journeys {
    pub journeys: BTreeMap<u32, Vec<Journey>>,
    pub stop_ids: Vec<StopDetails>,
    pub route_ids: Vec<RouteDetails>,
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
    /// array of [r, g, b]
    pub color: [u8; 3],
}

impl Journeys {
    pub unsafe fn access_unchecked(bytes: &[u8]) -> &ArchivedJourneys {
        unsafe { rkyv::access_unchecked(bytes) }
    }

    pub fn as_bytes(&self) -> Box<[u8]> {
        // with our implementation of this struct, we can only run into OOM
        // error case, in which case all we can do is panic
        rkyv::to_bytes::<rkyv::rancor::Panic>(self)
            .unwrap()
            .into_boxed_slice()
    }
}
