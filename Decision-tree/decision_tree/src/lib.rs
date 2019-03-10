use std::collections::{HashMap, HashSet};

type FeatureName = String;

enum Data {
    String(String),
    RangeData((i64, i64)),
}

struct Entry {
    features: HashSet<FeatureName>,
    data: HashMap<FeatureName, Data>,
}

struct EntrySet(Vec<Entry>);

struct DTree {
    thisFork: FeatureName,
    children: Vec<Box<DTree>>,
}

impl DTree {
    fn new() -> Self {
        DTree {
            thisFork: String::new(),
            children: vec![],
        }
    }
}

type DTreeAlgo = fn(EntrySet, HashSet<FeatureName>, FeatureName) -> DTree;

fn c4_5(datas: EntrySet, rest_features: HashSet<FeatureName>, target: FeatureName) -> DTree {
    DTree::new()
}

fn new_tree(datas: EntrySet, target: FeatureName, algo: DTreeAlgo) -> DTree {
    DTree::new()
}
