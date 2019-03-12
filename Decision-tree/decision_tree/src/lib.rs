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
fn gini(datas: &EntrySet, feature: &FeatureName, target: FeatureName) -> DTree {
    let mut record: HashMap<&Data, HashMap<&Data, u32>> = HashMap::new();
    let EntrySet(data_set) = datas;

    for data in data_set {
        let this_entry_of_target = data.data.get(&target);
        if let Some(record.get_mut(data.data))
        record.insert(
            data.data.get(feature).unwrap(),
            if data.data.get(target)
            record.get(data.data.get(feature).unwrap()).unwrap_or(&0) + 1,
        );
    }
}

fn c4_5(datas: EntrySet, rest_features: HashSet<FeatureName>, target: FeatureName) -> DTree {
    DTree::new()
}

fn new_tree(datas: EntrySet, target: FeatureName, algo: DTreeAlgo) -> DTree {
    DTree::new()
}
