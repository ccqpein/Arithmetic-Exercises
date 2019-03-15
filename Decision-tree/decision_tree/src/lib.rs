use std::collections::{HashMap, HashSet};

type FeatureName = String;

#[derive(PartialEq, Eq, Hash)]
enum Data {
    String(String),
    RangeData((i64, i64)),
}

#[derive(PartialEq, Eq)]
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

//only support string now
fn gini(datas: EntrySet, feature: FeatureName, target: FeatureName) -> DTree {
    let mut record = HashMap::new();

    for entry in datas {
        if let split_ = record
            .get_mut(entry.data.get(feature).unwrap())
            .unwrap_or(0)
        {}
    }

    //first, get all value set
    //spilt values
}

fn c4_5(datas: EntrySet, rest_features: HashSet<FeatureName>, target: FeatureName) -> DTree {
    DTree::new()
}

fn new_tree(datas: EntrySet, target: FeatureName, algo: DTreeAlgo) -> DTree {
    DTree::new()
}
