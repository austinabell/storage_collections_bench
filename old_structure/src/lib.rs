#![feature(asm)]
#![feature(bench_black_box)]

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::near_bindgen;
use std::hint::black_box;

#[derive(
    BorshDeserialize, BorshSerialize, Ord, PartialOrd, Eq, PartialEq, Clone, Debug, Default,
)]
pub struct StackHeapMock {
    a: u128,
    b: Option<[u8; 32]>,
    c: Vec<u8>,
}

type KeyType = StackHeapMock;
type ValueType = StackHeapMock;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct LookupBench {
    map: LookupMap<KeyType, ValueType>,
}

impl Default for LookupBench {
    fn default() -> Self {
        Self {
            map: LookupMap::new(b"m"),
        }
    }
}

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub enum Action<K, V> {
    Insert(K, V),
    Set(K, Option<V>),
    Remove(K),
    Flush,
    Get(K),
}

#[near_bindgen]
impl LookupBench {
    pub fn fuzz(&mut self, #[serializer(borsh)] actions: Vec<Action<KeyType, ValueType>>) {
        let lm = &mut self.map;
        for op in actions {
            match op {
                Action::Insert(k, v) => {
                    let _r = black_box(lm.insert(&k, &v));
                }
                Action::Set(k, v) => {
                    if let Some(v) = v {
                        let _r = black_box(lm.insert(&k, &v));
                    } else {
                        black_box(lm.remove(&k));
                    }
                }
                Action::Remove(k) => {
                    let _r = black_box(lm.remove(&k));
                }
                Action::Flush => {
                    // lm.flush();
                }
                Action::Get(k) => {
                    let _r = black_box(lm.get(&k));
                }
            }
        }
    }
}
