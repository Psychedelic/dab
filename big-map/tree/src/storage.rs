use crate::map::BigMapNode;
use std::future::Future;
use std::pin::Pin;

pub trait BigMapStorageBackend<Key, Value, Address> {
    /// Create a new node with all these given information and data.
    fn create_node(
        parent: &Address,
        deg: usize,
        memory_allocation: usize,
        tail: Option<&Address>,
        data: &[BigMapNode<Key, Value, Address>],
    ) -> Pin<Box<dyn Future<Output = Address>>>;
    /// Ask the value associated with key from the given node.
    fn get(node: &Address, key: &Key) -> Pin<Box<dyn Future<Output = Option<Value>>>>;
    /// Tell the given node to insert this value.
    fn insert(node: &Address, key: &Key, value: &Value) -> Pin<Box<dyn Future<Output = ()>>>;
    /// Insert the given data to a node, the data flows upward.
    /// The hint is used to locate the caller in order to verify access control, it is the first
    /// key in the child bucket.
    fn insert_upward(
        node: &Address,
        hint: &Key,
        key: &Key,
        value: &Value,
        child: &Address,
    ) -> Pin<Box<dyn Future<Output = ()>>>;
}
