use crate::mock::BigMapMockStorage;
use std::borrow::Borrow;
use std::iter::Iterator;

#[async_std::test]
async fn insert_ordered() {
    let lock = crate::mock::lock();

    let map = BigMapMockStorage::<usize, usize>::new_map(3);

    for i in 0..1000 {
        map.insert(i, i * 3).await;
    }

    for i in 0..1000 {
        assert_eq!(map.get(&i).await, Some(i * 3));
    }

    lock.borrow();
}

#[async_std::test]
async fn insert_reverse() {
    let lock = crate::mock::lock();

    let map = BigMapMockStorage::<usize, usize>::new_map(3);

    for i in (0..1000).rev() {
        map.insert(i, i * 3).await;
    }

    for i in 0..1000 {
        assert_eq!(map.get(&i).await, Some(i * 3));
    }

    lock.borrow();
}
