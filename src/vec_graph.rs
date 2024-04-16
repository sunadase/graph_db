use core::fmt;
use std::{
    borrow::{Borrow, BorrowMut},
    collections::HashMap,
    iter::zip,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PartialOrd, Ord)]
pub struct NodeIndex(usize);

impl From<usize> for NodeIndex {
    fn from(value: usize) -> Self {
        NodeIndex(value)
    }
}

impl From<i32> for NodeIndex {
    fn from(value: i32) -> Self {
        assert!(
            value >= 0,
            "NodeIndex can't be initialized with a negative integer"
        );
        NodeIndex(value as usize)
    }
}

impl fmt::Display for NodeIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Edge {
    relation: String,
    from: NodeIndex,
    to: NodeIndex,
}

impl Edge {
    fn new(relation: &str, from: NodeIndex, to: NodeIndex) -> Self {
        Edge {
            relation: relation.to_owned(),
            from,
            to,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub id: NodeIndex,
    pub alias: String,
    labels: Vec<String>,
    props: HashMap<String, String>,
    //    rels: Vec<Edge>
}

impl Node {
    pub fn new(id: NodeIndex, alias: String) -> Self {
        Node {
            id,
            alias,
            labels: Vec::new(),
            props: HashMap::new(),
        }
    }
    pub fn add_label<S: AsRef<str>>(&mut self, label: S) -> GraphResult<&mut Self> {
        self.labels.push(label.as_ref().to_owned());
        Ok(self)
    }
    pub fn remove_label<S: AsRef<str>>(&mut self, label: S) -> GraphResult<&mut Self> {
        self.labels.retain(|x| x != label.as_ref());
        Ok(self)
    }
    pub fn add_prop<S: AsRef<str>>(&mut self, key: S, val: S) -> GraphResult<&mut Self> {
        self.props
            .insert(key.as_ref().to_owned(), val.as_ref().to_owned());
        Ok(self)
    }
    pub fn remove_prop<S: AsRef<str>>(&mut self, key: S) -> GraphResult<&mut Self> {
        self.props.remove(key.as_ref());
        Ok(self)
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Node({:03}): \"{}\"\n\tlabels: {:?}\n\tprops:\n",
            self.id, self.alias, self.labels
        )?;
        for (k, v) in self.props.iter() {
            write!(f, "\t\t{}:{}\n", k, v)?;
        }
        Ok(())
    }
}

//struct AliasMap(HashMap<String, NodeIndex>);

#[derive(Debug, PartialEq, Eq)]
pub struct AliasMap {
    inner: HashMap<String, Vec<NodeIndex>>,
}
impl From<HashMap<String, Vec<NodeIndex>>> for AliasMap {
    fn from(value: HashMap<String, Vec<NodeIndex>>) -> Self {
        AliasMap { inner: value }
    }
}

impl AliasMap {
    //better return type?
    fn insert(&mut self, key: &str, value: NodeIndex) -> Option<Vec<NodeIndex>> {
        match self.inner.get_mut(key) {
            None => self.inner.insert(key.to_owned(), vec![value]),
            Some(x) => {
                x.push(value);
                None
            }
        }
    }
    #[inline]
    fn get(&self, key: &str) -> Option<&Vec<NodeIndex>> {
        self.inner.get(key)
    }
    #[inline]
    fn remove_all(&mut self, key: &str) -> Option<Vec<NodeIndex>> {
        self.inner.remove(key)
    }
    #[inline]
    fn remove_id_at(&mut self, key: &str, idx: &NodeIndex) -> Option<()> {
        self.inner.get_mut(key)?.retain(|x| x.0 != idx.0);
        Some(())
    }
    fn change_id_at(
        &mut self,
        key: &str,
        old_idx: &NodeIndex,
        mut new_idx: NodeIndex,
    ) -> Option<()> {
        self.inner
            .get_mut(key)?
            .iter_mut()
            .filter(|idx| *idx == old_idx)
            .for_each(|idx| *idx = new_idx);
        Some(())
    }
    #[inline]
    fn retain_at<F>(&mut self, key: &str, mut f: F) -> Option<()>
    where
        F: FnMut(&NodeIndex) -> bool,
    {
        self.inner.get_mut(key)?.retain(f);
        Some(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Graph {
    pub aliases: AliasMap,
    nodes: Vec<Node>,
    //nodes: HashMap<String, Node>
    edges: Vec<Edge>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            aliases: HashMap::new().into(),
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
    pub fn add_node(&mut self, alias: &str) -> GraphResult<&mut Self> {
        self.aliases.insert(alias, self.nodes.len().into());
        self.nodes
            .push(Node::new(self.nodes.len().into(), alias.to_owned()));
        Ok(self)
    }
    /// TODO:BUG:
    pub fn remove_node_by_id(&mut self, id: &NodeIndex) -> GraphResult<&mut Self> {
        if self.nodes.len() < id.0 {
            Err(format!(
                "There are {} nodes but tried to index at {}",
                self.nodes.len(),
                id.0
            )
            .into())
        } else {
            // let node = self.nodes.remove(id.0);
            //remove shifts over remaining elements so id assigning should be ok
            let node = self.nodes.swap_remove(id.0);
            //swaps id with last, removes and gets id, last is now id
            //remove node from other records
            self.aliases.remove_id_at(&node.alias, id);
            self.remove_all_edges_from(id)?;
            self.remove_all_edges_to(id)?;
            //swap lasts new id
            if let Some(last) = self.nodes.get_mut(id.0) {
                //last's alias will point to its old id
                self.aliases
                    .change_id_at(&last.alias, &last.id, id.to_owned());
                //swap all edges from and to last's old id to its new id
                self.edges.iter_mut().for_each(|edge| {
                    if edge.from == last.id {
                        edge.from = last.id
                    }
                    if edge.to == last.id {
                        edge.to = last.id
                    }
                });
                //change its inner id to its new id
                last.id = id.to_owned();
            } else {
                // no last node -> there was only 1 node?
            };

            Ok(self)
        }
    }
    #[inline]
    pub fn get_node_mut_by_idx(&mut self, idx: &NodeIndex) -> Option<&mut Node> {
        self.nodes.get_mut(idx.0)
    }
    pub fn get_nodes_mut_by_alias(&mut self, alias: &str) -> Option<Vec<&mut Node>> {
        let idxs = self.aliases.get(alias)?;
        let mut mut_node_iter = self.nodes.iter_mut();
        let mut nodes = Vec::with_capacity(idxs.len());

        // idxs.iter().for_each(|id| nodes.push(self.nodes.get_mut(id.0)));
        // idxs.iter().for_each(|id| nodes.push(self.nodes.get_mut(id.0).expect("Idx returned by alias was out of bounds?")));

        //BUG!?: if idxs are not ordered: since nth() consumes the iterator for previous values result will be incorrect if encountered a higher and lower id
        // + nth(0) == next(), nth(0).nth(0) == next().next(), nth(15) == [15], nth(15).nth(15) == [30], not [15],[15] so nth()'s after first id are also incorrect
        for id in idxs {
            nodes.push(mut_node_iter.nth(id.0)?)
        }
        return Some(nodes);
    }
    #[inline]
    pub fn get_last_node(&mut self) -> Option<&Node> {
        self.nodes.last()
    }
    #[inline]
    pub fn get_last_node_mut(&mut self) -> Option<&mut Node> {
        self.nodes.last_mut()
    }
    pub fn mut_last_node<F>(&mut self, mut f: F) -> GraphResult<&mut Self>
    where
        F: FnMut(&mut Node) -> GraphResult<&mut Node>,
    {
        f(self.nodes.last_mut().ok_or("Failed getting last node")?)?;
        Ok(self)
    }
    #[inline]
    pub fn get_node_by_idx(&self, idx: &NodeIndex) -> Option<&Node> {
        self.nodes.get(idx.0)
    }
    pub fn get_nodes_by_alias(&self, alias: &str) -> Option<Vec<&Node>> {
        let id = self.aliases.get(alias)?;
        if id.len() == 0 {
            return None;
        } else {
            let mut nodes = Vec::new();
            for i in id.iter() {
                nodes.push(self.nodes.get(i.0)?) //? dangerous?
            }
            return Some(nodes);
        }
    }
    #[inline]
    pub fn get_alias_by_id(&self, id: &NodeIndex) -> Option<&str> {
        self.get_node_by_idx(id)
            .and_then(|x| Some(x.alias.as_str()))
    }
    #[inline]
    pub fn get_ids_by_alias(&self, alias: &str) -> Option<&Vec<NodeIndex>> {
        self.aliases.get(alias)
    }
    pub fn add_edge(
        &mut self,
        relation: &str,
        from: NodeIndex,
        to: NodeIndex,
    ) -> GraphResult<&mut Self> {
        self.edges.push(Edge::new(relation, from, to));
        Ok(self)
    }
    pub fn add_edges_by_aliases(
        &mut self,
        relation: &str,
        from: &str,
        to: &str,
    ) -> GraphResult<&mut Self> {
        let fid = self
            .get_ids_by_alias(from)
            .ok_or(format!("Failed getting ids with {}", from))?
            .clone();
        let tid = self
            .get_ids_by_alias(to)
            .ok_or(format!("Failed getting ids with {}", from))?
            .clone();
        //need product not zip?
        // fid are all the ids that matched from string [1,2,3]
        // tid are all the ids that matched to string [4,5,6]
        // link all source(from) matches to each target(to) match
        // expected: [(1,4), (1,5), (1,6), (2,4), (2,5), (2,6), (3,4), (3,5), (3,6)]
        // zip:      [(1,4), (2,5), (3,6)]
        // for (f, t) in zip(fid, tid) {
        //     self.edges
        //         .push(Edge::new(relation, f, t))
        // }
        fid.iter().for_each(|f| {
            tid.iter().for_each(|t| 
                self.edges.push(Edge::new(relation, *f, *t)))
        });

        Ok(self)
    }
    pub fn remove_all_edges_from(&mut self, from: &NodeIndex) -> GraphResult<&mut Self> {
        self.edges.retain(|x| x.from.borrow() != from);
        Ok(self)
    }
    pub fn remove_all_edges_to(&mut self, to: &NodeIndex) -> GraphResult<&mut Self> {
        self.edges.retain(|x| x.to.borrow() != to);
        Ok(self)
    }
    pub fn get_outgoing_neighbors(&self, node: &Node) -> GraphResult<Vec<&Node>> {
        Ok(self
            .edges
            .iter()
            .filter(|e| e.from == node.id)
            .map(|e| {
                self.get_node_by_idx(&e.to)
                    .expect("Outgoing edge has invalid target(to) node index")
            })
            .collect())
    }
    pub fn get_incoming_neighbors(&self, node: &Node) -> GraphResult<Vec<&Node>> {
        Ok(self
            .edges
            .iter()
            .filter(|e| e.to == node.id)
            .map(|e| {
                self.get_node_by_idx(&e.from)
                    .expect("Incoming edge has invalid source(from) node index")
            })
            .collect())
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Nodes:\n")?;
        for n in self.nodes.iter() {
            write!(f, "{}", n)?;
        }
        write!(f, "Edges:\n")?;
        for e in self.edges.iter() {
            write!(
                f,
                "{:>10}({:02}) {:-^16}> ({:02}){:<10}\n",
                self.get_alias_by_id(&e.from).expect("Failed getting alias"),
                e.from,
                e.relation,
                e.to,
                self.get_alias_by_id(&e.to).expect("Failed getting alias"),
            )?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Error {
    Text(String),
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::Text(value)
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Error::Text(value.to_owned())
    }
}

pub type GraphResult<T> = Result<T, Error>;
