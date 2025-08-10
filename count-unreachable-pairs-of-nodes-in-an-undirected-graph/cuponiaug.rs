use std::collections::{BinaryHeap, HashMap, HashSet};
use std::error::Error;
use std::fmt::Display;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq)]
pub enum GErrorType {
    WrongType,
    MergeFailed,
    DeleteFailed,

    CorruptedData,
}

impl Display for GErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GErrorType::WrongType => write!(f, "WrongType"),
            GErrorType::MergeFailed => write!(f, "MergeFailed"),
            GErrorType::DeleteFailed => write!(f, "DeleteFailed"),
            GErrorType::CorruptedData => write!(f, "CorruptedData"),
        }
    }
}

#[derive(Debug)]
pub struct GError {
    msg: String,
    ty: GErrorType,
}

impl GError {
    pub fn new(kind: GErrorType, msg: &'_ str) -> Self {
        Self {
            msg: msg.to_string(),
            ty: kind,
        }
    }
}

impl Display for GError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.ty, self.msg)
    }
}

impl Error for GError {}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum GraphType {
    Undirected,
    Directed,
}

#[doc = r"value pairs of each nodes in graph"]
#[derive(Debug, Clone)]
pub struct IDValuePiar<ID, V>
where
    V: Ord,
{
    /// when ord is 0 (default), in BinaryHeap<IDValuePiar<ID, V>>
    /// less V will pop first. When it is 1, larger V pop first
    /// yea, I should use enum
    ord: u8,

    inner: (ID, V),
}

impl<ID, V: Ord> IDValuePiar<ID, V> {
    #[must_use]
    fn new(id: ID, v: V) -> Self {
        Self {
            ord: 0,
            inner: (id, v),
        }
    }

    fn ord(mut self, ord: u8) -> Self {
        self.ord = ord;
        self
    }

    pub fn id(&self) -> &ID {
        &self.inner.0
    }

    pub fn v(&self) -> &V {
        &self.inner.1
    }
}

impl<ID, V: Ord> Eq for IDValuePiar<ID, V> {
    fn assert_receiver_is_total_eq(&self) {}
}

impl<ID, V: Ord> Ord for IDValuePiar<ID, V> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.ord == 0 {
            other.inner.1.cmp(&self.inner.1)
        } else {
            self.inner.1.cmp(&other.inner.1)
        }
    }
}

impl<ID, V: Ord> PartialOrd for IDValuePiar<ID, V> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.ord == 0 {
            other.inner.1.partial_cmp(&self.inner.1)
        } else {
            self.inner.1.partial_cmp(&other.inner.1)
        }
    }
}

impl<ID, V: Ord> PartialEq for IDValuePiar<ID, V> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.1 == other.inner.1
    }
}

/// graph for store all nodes
/// V is the weight between the nodes
#[derive(Clone, Debug)]
pub struct Graph<ID, V>
where
    ID: Hash + Eq,
    V: Ord,
{
    ty: GraphType,
    graph: HashMap<ID, BinaryHeap<IDValuePiar<ID, V>>>,
}

impl<'a, ID, V> IntoIterator for &'a Graph<ID, V>
where
    ID: Hash + Clone + Eq,
    V: Ord + Clone,
{
    type Item = (&'a ID, &'a BinaryHeap<IDValuePiar<ID, V>>);

    type IntoIter = std::collections::hash_map::Iter<'a, ID, BinaryHeap<IDValuePiar<ID, V>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.graph.iter()
    }
}

impl<'a, ID, V> IntoIterator for &'a mut Graph<ID, V>
where
    ID: Hash + Clone + Eq,
    V: Ord + Clone,
{
    type Item = (&'a ID, &'a mut BinaryHeap<IDValuePiar<ID, V>>);

    type IntoIter = std::collections::hash_map::IterMut<'a, ID, BinaryHeap<IDValuePiar<ID, V>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.graph.iter_mut()
    }
}

impl<ID, V> Graph<ID, V>
where
    ID: Hash + Clone + Eq,
    V: Ord + Clone,
{
    #[must_use]
    pub fn new(ty: GraphType) -> Self {
        Self {
            ty,
            graph: HashMap::new(),
        }
    }

    /// insert neighbour node (one direction)
    pub fn insert(&mut self, id: ID, other_id: ID, v: V) {
        // default ord is 0
        self.insert_with_ord(id, other_id, v, 0);
    }

    /// Insert with the order of id-value pair
    pub fn insert_with_ord(&mut self, id: ID, other_id: ID, v: V, ord: u8) {
        match self.ty {
            GraphType::Undirected => {
                self.graph
                    .entry(id.clone())
                    .or_insert(BinaryHeap::new())
                    .push(IDValuePiar::new(other_id.clone(), v.clone()).ord(ord));

                self.graph
                    .entry(other_id)
                    .or_insert(BinaryHeap::new())
                    .push(IDValuePiar::new(id, v).ord(ord));
            }
            GraphType::Directed => {
                self.graph
                    .entry(id.clone())
                    .or_insert(BinaryHeap::new())
                    .push(IDValuePiar::new(other_id, v).ord(ord));
            }
        }
    }

    pub fn get(&self, k: &ID) -> Option<&BinaryHeap<IDValuePiar<ID, V>>> {
        self.graph.get(k)
    }

    pub fn get_mut(&mut self, k: &ID) -> Option<&mut BinaryHeap<IDValuePiar<ID, V>>> {
        self.graph.get_mut(k)
    }

    /// get the number of nodes of this graph
    pub fn len(&self) -> usize {
        self.graph.len()
    }

    /// return all ids of this graph
    pub fn all_ids(&self) -> impl Iterator<Item = &ID> {
        self.graph.keys()
    }

    /// delete the node from graph. only for undirected graph so far.
    /// because find which node direct to this node is expensive by now (2024-01-07).
    pub fn delete_node(&mut self, id: &ID) -> Result<(), GError> {
        if self.ty != GraphType::Undirected {
            return Err(GError::new(
                GErrorType::WrongType,
                "delete only for undirection graph",
            ));
        }

        let all_connects = match self.graph.get(id) {
            Some(h) => h.iter().map(|bh| bh.id().clone()).collect::<Vec<_>>(),
            None => {
                return Err(GError::new(
                    GErrorType::CorruptedData,
                    "Cannot find the delete node",
                ));
            }
        };

        // remove all leaves nodes' records of this node
        for n in all_connects {
            match self.graph.get_mut(&n) {
                Some(heap) => heap.retain(|e| e.id() != id),
                None => {
                    return Err(GError::new(
                        GErrorType::CorruptedData,
                        "Cannot find the node of delete node",
                    ));
                }
            }
        }

        self.graph.remove(id);
        Ok(())
    }

    /// get the ID the neighbours
    pub fn get_neighbours(&self, id: &ID) -> Option<impl IntoIterator<Item = &ID>> {
        match self.get(id) {
            Some(ns) => Some(ns.as_slice().iter().map(|n| n.id())),
            None => None,
        }
    }

    /// return id groups, those ids can visted from each other is a group
    pub fn groups(&self) -> Vec<HashSet<&ID>> {
        let mut all_ids = self.all_ids().into_iter().collect::<HashSet<_>>();
        let mut groups = vec![];
        let mut next_round = vec![];

        loop {
            // give the random start
            match all_ids.iter().next() {
                Some(x) => next_round.push(*x),
                None => return groups,
            }

            let mut group = HashSet::new();
            while !next_round.is_empty() {
                let this = next_round.pop().unwrap();
                if group.contains(this) {
                    continue;
                }

                if let Some(ns) = self.get_neighbours(this) {
                    next_round.append(&mut ns.into_iter().collect::<Vec<_>>());
                }
                group.insert(this);
                all_ids.remove(this);
            }

            if group.is_empty() {
                break;
            } else {
                groups.push(group)
            }
        }

        groups
    }
}

pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
    let mut mm = Graph::new(GraphType::Undirected);

    edges.into_iter().for_each(|pp| {
        mm.insert(pp[0], pp[1], 1);
    });

    let all_ids = mm.all_ids().map(|d| *d).collect::<HashSet<_>>();
    for i in 0..n {
        if !all_ids.contains(&i) {
            mm.insert(i, i, 1);
        }
    }

    let gg = mm.groups().into_iter().map(|g| g.len()).collect::<Vec<_>>();
    if gg.len() <= 1 {
        return 0;
    }

    dbg!(&gg);

    let mut res = 0;
    for i in 0..gg.len() - 1 {
        for j in i + 1..gg.len() {
            res += (gg[i] * gg[j]) as i64
        }
    }
    res
}

fn main() {
    assert_eq!(0, count_pairs(3, vec![vec![0, 1], vec![0, 2], vec![1, 2]]));
    assert_eq!(
        14,
        count_pairs(
            7,
            vec![vec![0, 2], vec![0, 5], vec![2, 4], vec![1, 6], vec![5, 4]]
        )
    );
    assert_eq!(0, count_pairs(100000, vec![]));
}
