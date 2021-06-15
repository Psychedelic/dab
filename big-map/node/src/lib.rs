use ic_cdk::export::candid::CandidType;
use ic_cdk::export::Principal;
use ic_cdk::*;
use ic_cdk_macros::*;
use serde::Deserialize;
use tree;
use tree::BigMapNode;

type Key = Box<[u8]>;
type Value = Box<[u8]>;
type Map = tree::BigMap<Key, Value, Principal, tree::ic::BigMapIcStorage<Key, Value, Wasm>>;

pub struct Data(Option<Map>);
impl Default for Data {
    fn default() -> Self {
        Self(None)
    }
}

pub struct Wasm(Option<Vec<u8>>);
impl Default for Wasm {
    fn default() -> Self {
        Self(None)
    }
}

impl tree::ic::IcWasmProvider for Wasm {
    fn bytes() -> &'static [u8] {
        let wasm = storage::get::<Wasm>();
        wasm.0.as_ref().unwrap().as_slice()
    }
}

#[derive(CandidType, Deserialize)]
pub struct InitArgs {
    deg: usize,
    tail: Option<Principal>,
    memory_allocation: usize,
    data: Vec<tree::BigMapNode<Key, Value, Principal>>,
}

#[update]
fn init(args: InitArgs) -> Result<(), String> {
    let data = storage::get_mut::<Data>();

    if data.0.is_some() {
        return Err("Already initialized.".to_string());
    }

    let map = Map::with_data(
        api::id(),
        args.deg,
        args.memory_allocation,
        caller(),
        args.tail,
        args.data,
    );

    data.0.replace(map);

    Ok(())
}

// No guard, anyone can read anything.
#[update]
async fn get(key: Key) -> Option<Value> {
    let map = storage::get_mut::<Data>().0.as_mut().unwrap();
    map.get(&key).await
}

#[update(guard = "is_parent")]
async fn insert(key: Key, value: Value) -> Result<(), String> {
    let map = storage::get_mut::<Data>().0.as_mut().unwrap();
    map.insert(key, value).await;
    Ok(())
}

#[update]
async fn insert_upward(hint: Key, key: Key, value: Value, child: Principal) -> Result<(), String> {
    let map = storage::get_mut::<Data>().0.as_mut().unwrap();

    if map.get_near_child(&hint).unwrap() != &caller() {
        return Err("Not allowed.".to_string());
    }

    map.insert_upward(key, value, child).await;

    Ok(())
}

#[update(guard = "is_parent")]
async fn update_parent(parent: Principal) {
    let map = storage::get_mut::<Data>().0.as_mut().unwrap();
    map.set_parent(parent);
}

#[update(guard = "is_parent")]
fn store_wasm(wasm: Vec<u8>) -> Result<(), String> {
    storage::get_mut::<Wasm>().0.replace(wasm);
    Ok(())
}

#[derive(CandidType, Deserialize)]
struct GetRawDataArgs {
    from: Option<u64>,
    limit: u16,
}

#[query]
fn get_raw_data(args: GetRawDataArgs) -> &'static [BigMapNode<Key, Value, Principal>] {
    let from = args.from.unwrap_or(0) as usize;
    let limit = args.limit as usize;

    let map = storage::get_mut::<Data>().0.as_mut().unwrap();
    map.get_raw(from, limit)
}

#[query]
fn get_tail() -> Option<Principal> {
    let map = storage::get_mut::<Data>().0.as_mut().unwrap();
    map.get_tail().cloned()
}

fn is_parent() -> Result<(), String> {
    let map = storage::get::<Data>().0.as_ref().unwrap();
    match &caller() == map.parent.as_ref().unwrap() {
        true => Ok(()),
        false => Err("Only the parent can call this method.".to_string()),
    }
}
