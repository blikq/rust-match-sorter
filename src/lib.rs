pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

use std::collections::HashMap;
use std::any::Any;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Ranking {
    CaseSensitiveEqual = 7,
    Equal = 6,
    StartsWith = 5,
    WordStartsWith = 4,
    Contains = 3,
    Acronym = 2,
    Matches = 1,
    NoMatch = 0,
}

pub struct KeyAttributes {
    pub treshoold: Option<Ranking>,
    pub min_ranking: Option<Ranking>,
    pub max_ranking: Option<Ranking>,
}

pub struct RankingInfo {
    pub ranked_value: String,
    pub rank: Ranking,
    pub key_index: usize,
    pub key_threshold: Option<Ranking>,
}

pub struct IndexedItem<ItemType> {
    pub item: ItemType,
    pub index: usize,
}

pub struct RankedItem<ItemType> {
    pub ranked_value: String,
    pub rank: Ranking,
    pub key_index: usize,
    pub key_threshold: Option<Ranking>,
    pub item: ItemType,
    pub index: usize,
}

pub enum StringOrArray {
    String(String),
    Array(Vec<String>),
}

pub trait ValueGetterKey<ItemType> {
    fn get_value(&self) -> StringOrArray;
}

// TODO: check constraint and implementation
impl<ItemType> ValueGetterKey<ItemType> for ItemType
where
    ItemType: ToString, // Constraint: ItemType must implement ToString
{
    fn get_value(&self) -> StringOrArray {
        StringOrArray::String(self.to_string())
    }
}

pub type BaseSorter<ItemType> = fn(
    a: RankedItem<ItemType>, 
    b: RankedItem<ItemType>
) -> usize;

pub type Sorter<ItemType> = fn(
    match_items: Vec<RankedItem<ItemType>>,
) -> Vec<RankedItem<ItemType>>;

pub enum KeyOptionsOption<ItemType> {
    String(String),
    ValueGetter(Box<dyn ValueGetterKey<ItemType>>),
}

pub struct KeyAttributesOptions<ItemType> {
    pub key: KeyOptionsOption<ItemType>,
    pub treshoold: Option<Ranking>,
    pub min_ranking: Option<Ranking>,
    pub max_ranking: Option<Ranking>,
}

pub enum KeyOption<ItemType> {
    KeyAttributesOptions(KeyAttributesOptions<ItemType>),
    ValueGetter(Box<dyn ValueGetterKey<ItemType>>),
    String(String),
}

pub type IndexableByString = HashMap<String, Box<dyn Any>>;

pub fn default_base_sorter<ItemType>(a: &RankedItem<ItemType>, b: &RankedItem<ItemType>) -> std::cmp::Ordering {
    a.ranked_value.cmp(&b.ranked_value)
}

