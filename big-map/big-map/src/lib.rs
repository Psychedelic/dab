use ic_cdk::export::candid::de::decode_args;
use ic_cdk::export::candid::ser::encode_args;
use ic_cdk::export::candid::CandidType;
use ic_cdk::export::Principal;
use serde::de::Deserialize;
use std::marker::PhantomData;
use tree;

const NODE_WASM: &'static [u8] =
    include_bytes!("../../../target/wasm32-unknown-unknown/release/node-opt.wasm");

struct WasmProvider;

impl tree::ic::IcWasmProvider for WasmProvider {
    fn bytes() -> &'static [u8] {
        NODE_WASM
    }
}

type MapKey = Box<[u8]>;
type MapValue = Box<[u8]>;
type Map = tree::BigMap<
    MapKey,
    MapValue,
    Principal,
    tree::ic::BigMapIcStorage<MapKey, MapValue, WasmProvider>,
>;

pub struct BigMap<Key, Value> {
    tree: Map,
    kv: PhantomData<(Key, Value)>,
}

impl<Key: CandidType, Value: CandidType + for<'a> Deserialize<'a>> BigMap<Key, Value> {
    pub fn new(deg: usize, memory_allocation: usize) -> Self {
        BigMap {
            tree: Map::new(ic_cdk::api::id(), deg, memory_allocation, None),
            kv: PhantomData::default(),
        }
    }

    /// Return the value associated with the given key.
    pub async fn get(&self, key: Key) -> Option<Value> {
        let key_raw = encode_args((key,))
            .expect("Failed to encode key.")
            .into_boxed_slice();
        let value_raw = self.tree.get(&key_raw).await;
        value_raw.map(|v| {
            decode_args::<(Value,)>(&v)
                .expect("Failed to decode value.")
                .0
        })
    }

    /// Insert a key-value pair into the map.
    pub async fn insert(&mut self, key: Key, value: Value) {
        let key_raw = encode_args((key,))
            .expect("Failed to encode key.")
            .into_boxed_slice();

        let value_raw = encode_args((value,))
            .expect("Failed to encode value.")
            .into_boxed_slice();

        self.tree.insert(key_raw, value_raw).await;
    }
}
