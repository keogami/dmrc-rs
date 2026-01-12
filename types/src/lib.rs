use std::collections::BTreeMap;

#[derive(rkyv::Serialize, Debug, rkyv::Archive)]
#[rkyv(derive(Debug))]
pub struct TestStruct {
    pub text: String,
    pub tree: BTreeMap<u32, Vec<(u16, u16)>>,
}

impl TestStruct {
    pub fn new(text: String) -> Self {
        let tree = (0..136000).map(|i| (i, (('A' as _)..).zip(('a' as _)..).take(12).collect()));
        Self {
            text,
            tree: tree.collect(),
        }
    }

    pub unsafe fn access_unchecked(bytes: &[u8]) -> &ArchivedTestStruct {
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
