use crate::map::{BigMap, BigMapNode};
use crate::storage::BigMapStorageBackend;
use std::any::Any;
use std::collections::BTreeMap;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::{Mutex, MutexGuard};

type StorageTree = BTreeMap<MockNodeAddress, Box<dyn Any>>;
static mut STORAGE: Option<StorageTree> = None;
static mut LOCK: Option<Mutex<()>> = None;

pub struct BigMapMockStorage<K: Ord + Clone, V: Clone>(PhantomData<(K, V)>);
pub type MockNodeAddress = usize;

pub type MockBigMap<K, V> = BigMap<K, V, MockNodeAddress, BigMapMockStorage<K, V>>;

#[inline]
fn storage() -> &'static mut StorageTree {
    unsafe {
        if let Some(s) = &mut STORAGE {
            s
        } else {
            STORAGE = Some(BTreeMap::new());
            storage()
        }
    }
}

#[inline]
pub fn lock() -> MutexGuard<'static, ()> {
    unsafe {
        if let Some(l) = &LOCK {
            l.lock().unwrap()
        } else {
            LOCK = Some(Mutex::new(()));
            lock()
        }
    }
}

impl<Key: 'static + Ord + Clone, Value: 'static + Clone> BigMapMockStorage<Key, Value> {
    pub fn new_map(deg: usize) -> &'static mut MockBigMap<Key, Value> {
        let storage = storage();
        let address = storage.len();
        let map = MockBigMap::<Key, Value>::new(address.clone(), deg, 0, None);
        storage.insert(address, Box::new(map));

        storage
            .get_mut(&address)
            .unwrap()
            .downcast_mut::<MockBigMap<Key, Value>>()
            .expect("Unexpected value of invalid type")
    }
}

impl<Key: 'static + Ord + Clone, Value: 'static + Clone>
    BigMapStorageBackend<Key, Value, MockNodeAddress> for BigMapMockStorage<Key, Value>
{
    fn create_node<'a>(
        parent: &'a MockNodeAddress,
        deg: usize,
        memory_allocation: usize,
        tail: Option<&'a MockNodeAddress>,
        data: &'a [BigMapNode<Key, Value, MockNodeAddress>],
    ) -> Pin<Box<dyn Future<Output = MockNodeAddress>>> {
        let parent = *parent;
        let tail = tail.copied();
        let data: Vec<BigMapNode<Key, Value, MockNodeAddress>> = data
            .into_iter()
            .map(|node| BigMapNode {
                key: node.key.clone(),
                value: node.value.clone(),
                child: node.child.clone(),
            })
            .collect();

        Box::pin(async move {
            let storage = storage();
            let address = storage.len();

            if let Some(tail) = tail {
                storage
                    .get_mut(&tail)
                    .expect("Invalid address")
                    .downcast_mut::<MockBigMap<Key, Value>>()
                    .expect("Unexpected value of invalid type")
                    .set_parent(address);
            }

            for item in data.iter() {
                if let Some(id) = item.child {
                    storage
                        .get_mut(&id)
                        .expect("Invalid address")
                        .downcast_mut::<MockBigMap<Key, Value>>()
                        .expect("Unexpected value of invalid type")
                        .set_parent(address);
                }
            }

            let map = MockBigMap::<Key, Value>::with_data(
                address,
                deg,
                memory_allocation,
                parent,
                tail,
                data,
            );
            storage.insert(address, Box::new(map));

            address
        })
    }

    fn get(node: &MockNodeAddress, key: &Key) -> Pin<Box<dyn Future<Output = Option<Value>>>> {
        let node = *node;
        let key = key.clone();

        Box::pin(async move {
            let map = storage()
                .get_mut(&node)
                .expect("Invalid address")
                .downcast_mut::<MockBigMap<Key, Value>>()
                .expect("Unexpected value of invalid type");
            map.get(&key).await
        })
    }

    fn insert(
        node: &MockNodeAddress,
        key: &Key,
        value: &Value,
    ) -> Pin<Box<dyn Future<Output = ()>>> {
        let node = *node;
        let key = key.clone();
        let value = value.clone();

        Box::pin(async move {
            let map = storage()
                .get_mut(&node)
                .expect("Invalid address")
                .downcast_mut::<MockBigMap<Key, Value>>()
                .expect("Unexpected value of invalid type");

            map.insert(key, value).await
        })
    }

    fn insert_upward(
        node: &MockNodeAddress,
        _hint: &Key,
        key: &Key,
        value: &Value,
        child: &MockNodeAddress,
    ) -> Pin<Box<dyn Future<Output = ()>>> {
        let node = *node;
        let key = key.clone();
        let value = value.clone();
        let child = *child;

        Box::pin(async move {
            let map = storage()
                .get_mut(&node)
                .expect("Invalid address")
                .downcast_mut::<MockBigMap<Key, Value>>()
                .expect("Unexpected value of invalid type");

            map.insert_upward(key, value, child).await
        })
    }
}

impl<Key: 'static + std::fmt::Debug + Clone + Ord, Value: 'static + std::fmt::Debug + Clone>
    BigMap<Key, Value, MockNodeAddress, BigMapMockStorage<Key, Value>>
{
    pub fn print(&self) {
        println!(
            "node({}): parent = {:?} tail = {:?} data = {:?}",
            self.address, self.parent, self.tail, self.data
        );

        let mut to_print = vec![];

        if let Some(id) = self.tail {
            to_print.push(id);
        }

        for x in &self.data {
            if let Some(id) = x.child {
                to_print.push(id);
            }
        }

        for id in to_print {
            storage()
                .get_mut(&id)
                .expect("Invalid address")
                .downcast_mut::<MockBigMap<Key, Value>>()
                .expect("Unexpected value of invalid type")
                .print();
        }
    }
}
