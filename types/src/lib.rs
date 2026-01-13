use std::collections::BTreeMap;

#[derive(rkyv::Serialize, Debug, rkyv::Archive)]
#[rkyv(derive(Debug))]
pub struct Journeys {
    pub journeys: BTreeMap<u32, Vec<Journey>>,
    pub stop_ids: Vec<String>,
    pub route_ids: Vec<String>,
}

#[derive(rkyv::Serialize, Debug, rkyv::Archive)]
#[rkyv(derive(Debug))]
pub struct Journey {
    pub arrival: u16,
    pub plan: Vec<(u16, u16)>,
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
