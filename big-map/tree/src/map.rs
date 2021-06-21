use crate::storage::BigMapStorageBackend;
use ic_cdk::export::candid::CandidType;
use serde::Deserialize;
use std::fmt::Debug;
use std::marker::PhantomData;

pub struct BigMap<Key, Value, Address, Storage: BigMapStorageBackend<Key, Value, Address>> {
    pub tail: Option<Address>,
    pub data: Vec<BigMapNode<Key, Value, Address>>,
    pub address: Address,
    pub parent: Option<Address>,
    pub deg: usize,
    pub memory_allocation: usize,
    storage: PhantomData<Storage>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct BigMapNode<Key, Value, Address> {
    pub key: Key,
    pub value: Value,
    pub child: Option<Address>,
}

impl<
        Key: Ord + Clone,
        Value: Clone,
        Address,
        Storage: BigMapStorageBackend<Key, Value, Address>,
    > BigMap<Key, Value, Address, Storage>
{
    /// Create a new instance of a big-map node.
    pub fn new(
        address: Address,
        deg: usize,
        memory_allocation: usize,
        parent: Option<Address>,
    ) -> Self {
        Self {
            tail: None,
            data: Vec::with_capacity(deg),
            address,
            parent,
            deg,
            memory_allocation,
            storage: PhantomData::default(),
        }
    }

    /// Create a new node with initial data.
    pub fn with_data(
        address: Address,
        deg: usize,
        memory_allocation: usize,
        parent: Address,
        tail: Option<Address>,
        mut data: Vec<BigMapNode<Key, Value, Address>>,
    ) -> Self {
        let len = data.len();
        if deg > len {
            data.reserve_exact(deg - len);
        }
        Self {
            tail,
            data,
            address,
            parent: Some(parent),
            deg,
            memory_allocation,
            storage: PhantomData::default(),
        }
    }

    /// Returns true if the current node is the root of the b-tree.
    #[inline]
    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    #[inline(always)]
    fn binary_search(&self, key: &Key) -> Result<usize, usize> {
        self.data.binary_search_by(|node| node.key.cmp(key))
    }

    /// Try to find the given key inside the current node, or return the next node that we need to
    /// search.
    #[inline]
    fn search(&self, key: &Key) -> Result<&Value, Option<&Address>> {
        match self.binary_search(key) {
            Ok(index) => Ok(&self.data[index].value),
            Err(0) => Err(self.tail.as_ref()),
            Err(index) => Err(self.data[index - 1].child.as_ref()),
        }
    }

    /// Return the value associated with the given key from the map.
    pub async fn get(&self, key: &Key) -> Option<Value> {
        match self.search(key) {
            Ok(value) => Some(value.clone()),
            Err(None) => None,
            Err(Some(address)) => Storage::get(address, key).await,
        }
    }

    /// This is a low-level pai to optimize value updates, only use it when you know what you're doing.
    /// split must be called after calling this method.
    pub async fn update(&mut self, key: &Key, default: Value) -> Result<&mut Value, &Address> {
        self.split().await; // Recovery.

        match self.binary_search(key) {
            Ok(index) => Ok(&mut self.data[index].value),
            Err(0) if self.tail.is_none() => {
                let node = BigMapNode {
                    key: key.clone(),
                    value: default,
                    child: None,
                };

                self.data.insert(0, node);

                Ok(&mut self.data[0].value)
            }
            Err(0) => Err(self.tail.as_ref().unwrap()),
            // Is self.data[index - 1] always defined? YES
            // yes if { index - 1 < len() }
            //        { index < len() + 1 }
            //        { index <= len()    }
            // from BS we know 0 <= index <= len(), so the condition is met, and we proved it.
            Err(index) if self.data[index - 1].child.is_none() => {
                let node = BigMapNode {
                    key: key.clone(),
                    value: default,
                    child: None,
                };

                self.data.insert(index, node);

                Ok(&mut self.data[index].value)
            }
            Err(index) => {
                let node = self.data[index - 1].child.as_ref().unwrap();
                Err(node)
            }
        }
    }

    pub async fn insert(&mut self, key: Key, value: Value) {
        match self.update(&key, value.clone()).await {
            Ok(v) => {
                *v = value;
            }
            Err(node) => {
                Storage::insert(node, &key, &value).await;
            }
        };

        self.split().await;
    }

    pub async fn insert_upward(&mut self, key: Key, value: Value, child: Address) {
        self.split().await; // Recovery.

        let element = BigMapNode {
            key,
            value,
            child: Some(child),
        };

        match self.binary_search(&element.key) {
            Ok(_) => {
                unreachable!()
            }
            Err(index) => {
                self.data.insert(index, element);
            }
        }

        self.split().await;
    }

    pub fn get_raw(&self, from: usize, limit: usize) -> &[BigMapNode<Key, Value, Address>] {
        &self.data[from..from + limit]
    }

    pub fn get_tail(&self) -> Option<&Address> {
        self.tail.as_ref()
    }

    pub fn set_parent(&mut self, address: Address) {
        self.parent = Some(address);
    }

    pub async fn split(&mut self) {
        if self.data.len() != self.deg {
            return;
        }

        if self.is_root() {
            return self.split_root().await;
        }

        let mid_index = self.deg / 2;
        let mid = &self.data[mid_index];

        let data = &self.data[mid_index + 1..];
        let tail = mid.child.as_ref();

        // This node is gonna be inserted upward, so the actual parent is not the current node, but
        // actually the parent of the current node.
        let parent = self.parent.as_ref().unwrap();
        let node = Storage::create_node(parent, self.deg, self.memory_allocation, tail, data).await;

        let hint = &self.data[0].key;

        Storage::insert_upward(parent, hint, &mid.key, &mid.value, &node).await;

        self.data.truncate(mid_index);
    }

    /// Special case, handles splitting the root node.
    async fn split_root(&mut self) {
        let mid_index = self.deg / 2;

        let mid_key = self.data[mid_index].key.clone();
        let mid_value = self.data[mid_index].value.clone();
        let mid_child = self.data[mid_index].child.as_ref();

        let left_slice = &self.data[0..mid_index];
        let right_slice = &self.data[mid_index + 1..];

        let left = Storage::create_node(
            &self.address,
            self.deg,
            self.memory_allocation,
            self.tail.as_ref(),
            left_slice,
        )
        .await;
        let right = Storage::create_node(
            &self.address,
            self.deg,
            self.memory_allocation,
            mid_child,
            right_slice,
        )
        .await;

        self.tail = Some(left);
        let node = BigMapNode {
            key: mid_key,
            value: mid_value,
            child: Some(right),
        };

        self.data.clear();
        self.data.push(node);
    }

    pub fn get_near_child(&self, key: &Key) -> Option<&Address> {
        match self.binary_search(key) {
            Ok(_) => unreachable!(),
            Err(0) => self.tail.as_ref(),
            Err(index) => self.data[index - 1].child.as_ref(),
        }
    }
}
