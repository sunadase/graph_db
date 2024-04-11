use core::fmt;
use std::{
    borrow::{Borrow, BorrowMut},
    collections::HashMap,
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
    pub fn add_label(&mut self, label: &str) -> GraphResult<&mut Self> {
        self.labels.push(label.to_owned());
        Ok(self)
    }
    pub fn remove_label(&mut self, label: &str) -> GraphResult<&mut Self> {
        self.labels.retain(|x| x != label);
        Ok(self)
    }
    pub fn add_prop(&mut self, key: &str, val: &str) -> GraphResult<&mut Self> {
        self.props.insert(key.to_owned(), val.to_owned());
        Ok(self)
    }
    pub fn remove_prop(&mut self, key: &str) -> GraphResult<&mut Self> {
        self.props.remove(key);
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
struct AliasMap {
    inner: HashMap<String, Vec<NodeIndex>>,
}
impl From<HashMap<String, Vec<NodeIndex>>> for AliasMap {
    fn from(value: HashMap<String, Vec<NodeIndex>>) -> Self {
        AliasMap { inner: value }
    }
}

impl AliasMap {
    //better return type?
    fn insert(&mut self, key: &str, value: NodeIndex) -> Option<()> {
        match self.inner.get_mut(key) {
            None => {
                self.inner.insert(key.to_owned(), vec![value]);
                Some(())
            }
            Some(x) => Some(x.push(value)),
        }
    }
    fn get(&self, key: &str) -> Option<&Vec<NodeIndex>> {
        self.inner.get(key)
    }
    fn remove_all(&mut self, key: &str) -> Option<Vec<NodeIndex>> {
        self.inner.remove(key)
    }
    fn remove_at(&mut self, key: &str, idx: &NodeIndex) -> Option<()> {
        self.inner.get_mut(key)?.retain(|x| x.0 != idx.0);
        Some(())
    }
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
    aliases: AliasMap,
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
            let node = self.nodes.remove(id.0);
            //remove shifts over remaining elements so id assigning should be ok
            self.aliases.remove_at(&node.alias, id);
            self.remove_all_edges_from(id)?;
            self.remove_all_edges_to(id)?;
            Ok(self)
        }
    }
    ///TODO: BUG: getting by index is wrong if it depends on
    /// node ids :. ID assigned at node creation; index changes
    /// with node removes
    pub fn get_node_mut_by_idx(&mut self, idx: &NodeIndex) -> Option<&mut Node> {
        self.nodes.get_mut(idx.0)
    }
    pub fn get_nodes_mut_by_alias(&mut self, alias: &str) -> Option<Vec<&mut Node>> {
        //mutable?
        let id = self.aliases.get(alias)?;

        let mut mut_node_iter = self.nodes.iter_mut();
        let mut nodes = Vec::with_capacity(id.len());
        for i in id {
            nodes.push(mut_node_iter.nth(i.0)?)
        }

        return Some(nodes);

        // let mut nodes = Vec::new();
        // for i in id.iter() {
        //     nodes.push(self.nodes.get_mut(i.0)?) //questionmark? dangerous?
        // }
        // if nodes.len() == 0 {
        //     return None
        // } else {
        //     return Some(nodes)
        // }
    }
    pub fn get_last_node(&mut self) -> Option<&Node> {
        self.nodes.last()
    }
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
    pub fn get_node_by_idx(&self, idx: &NodeIndex) -> Option<&Node> {
        self.nodes.get(idx.0)
    }
    pub fn get_nodes_by_alias(&self, alias: &str) -> Option<Vec<&Node>> {
        let id = self.aliases.get(alias)?;
        let mut nodes = Vec::new();
        for i in id.iter() {
            nodes.push(self.nodes.get(i.0)?) //? dangerous?
        }
        if nodes.len() == 0 {
            return None;
        } else {
            return Some(nodes);
        }
    }
    pub fn get_alias_by_id(&self, id: &NodeIndex) -> Option<&str> {
        self.get_node_by_idx(id)
            .and_then(|x| Some(x.alias.as_str()))
    }
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
        for f in fid.iter() {
            for t in tid.iter() {
                self.edges
                    .push(Edge::new(relation, f.to_owned(), t.to_owned()));
            }
        }
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
        let mut nodes = Vec::new();
        for e in self.edges.iter() {
            if e.from == node.id {
                self.get_node_by_idx(&e.to).map(|x| nodes.push(x));
            }
        }
        Ok(nodes)
    }
    pub fn get_incoming_neighbors(&self, node: &Node) -> GraphResult<Vec<&Node>> {
        let mut nodes = Vec::new();
        for e in self.edges.iter() {
            if e.to == node.id {
                self.get_node_by_idx(&e.from).map(|x| nodes.push(x));
            }
        }
        Ok(nodes)
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
