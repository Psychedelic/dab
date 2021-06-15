use crate::storage::BigMapStorageBackend;
use crate::BigMapNode;
use ic_cdk::call;
use ic_cdk::export::candid::{CandidType, Nat};
use ic_cdk::export::Principal;
use serde::de::DeserializeOwned;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use utils::canister::*;

pub struct BigMapIcStorage<
    Key: CandidType + DeserializeOwned,
    Value: CandidType + DeserializeOwned,
    WASM,
>(PhantomData<(Key, Value, WASM)>);

pub trait IcWasmProvider {
    /// The bytes of the WASM binary.
    fn bytes() -> &'static [u8];
}

#[derive(CandidType)]
struct InitArgs<Key, Value> {
    deg: usize,
    tail: Option<Principal>,
    memory_allocation: usize,
    data: Vec<BigMapNode<Key, Value, Principal>>,
}

impl<
        Key: CandidType + DeserializeOwned + Clone + 'static,
        Value: CandidType + DeserializeOwned + Clone + 'static,
        WASM: IcWasmProvider,
    > BigMapStorageBackend<Key, Value, Principal> for BigMapIcStorage<Key, Value, WASM>
{
    fn create_node(
        _parent: &Principal,
        deg: usize,
        memory_allocation: usize,
        tail: Option<&Principal>,
        data: &[BigMapNode<Key, Value, Principal>],
    ) -> Pin<Box<dyn Future<Output = Principal>>> {
        let args = CreateCanisterArgs {
            cycles: 5_000_000_000_000,
            settings: CanisterSettings {
                controller: None,
                compute_allocation: None,
                memory_allocation: Some(Nat::from(memory_allocation)),
                freezing_threshold: None,
            },
        };

        let init_args = InitArgs {
            deg,
            tail: tail.cloned(),
            memory_allocation,
            data: data.iter().cloned().collect(),
        };

        Box::pin(async move {
            let id = create_canister_call(args).await.unwrap().canister_id;
            install_wasm(&id, Vec::from(WASM::bytes())).await.unwrap();

            // init
            call::<(InitArgs<Key, Value>,), (Result<(), String>,)>(
                id.clone(),
                "init",
                (init_args,),
            )
            .await
            .unwrap()
            .0
            .unwrap();

            // store wasm
            call::<(Vec<u8>,), (Result<(), String>,)>(
                id.clone(),
                "store_wasm",
                (Vec::from(WASM::bytes()),),
            )
            .await
            .unwrap()
            .0
            .unwrap();

            id
        })
    }

    fn get(node: &Principal, key: &Key) -> Pin<Box<dyn Future<Output = Option<Value>>>> {
        let id = node.clone();
        let key = key.clone();

        Box::pin(async move {
            call::<(Key,), (Option<Value>,)>(id, "get", (key,))
                .await
                .unwrap()
                .0
        })
    }

    fn insert(node: &Principal, key: &Key, value: &Value) -> Pin<Box<dyn Future<Output = ()>>> {
        let id = node.clone();
        let key = key.clone();
        let value = value.clone();

        Box::pin(async move {
            call::<(Key, Value), (Result<(), String>,)>(id, "insert", (key, value))
                .await
                .unwrap()
                .0
                .unwrap();
            ()
        })
    }

    fn insert_upward(
        node: &Principal,
        hint: &Key,
        key: &Key,
        value: &Value,
        child: &Principal,
    ) -> Pin<Box<dyn Future<Output = ()>>> {
        let id = node.clone();
        let hint = hint.clone();
        let key = key.clone();
        let value = value.clone();
        let child = child.clone();

        Box::pin(async move {
            call::<(Key, Key, Value, Principal), (Result<(), String>,)>(
                id,
                "insert_upward",
                (hint, key, value, child),
            )
            .await
            .unwrap()
            .0
            .unwrap();
            ()
        })
    }
}
