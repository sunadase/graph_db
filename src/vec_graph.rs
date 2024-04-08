use core::fmt;
use std::{borrow::{Borrow, BorrowMut}, collections::HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PartialOrd, Ord)]
pub struct NodeIndex(usize);

impl From<usize> for NodeIndex {
    fn from(value: usize) -> Self {
        NodeIndex(value)
    }
}

impl fmt::Display for NodeIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

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

pub struct Node {
    pub id: NodeIndex,
    pub alias: String,
    labels: Vec<String>,
    props: HashMap<String, String>,
    //    rels: Vec<Edge>
}

impl Node {
    fn new(id: NodeIndex, alias: String) -> Self {
        Node {
            id,
            alias,
            labels: Vec::new(),
            props: HashMap::new(),
        }
    }
    pub fn add_label(&mut self, label: &str) -> Result<&Self, String> {
        self.labels.push(label.to_owned());
        Ok(self)
    }
    pub fn remove_label(&mut self, label: &str) -> Result<&Self, String> {
        self.labels.retain(|x| x != label);
        Ok(self)
    }
    pub fn add_prop(&mut self, key: &str, val: &str) -> Result<&Self, String> {
        self.props.insert(key.to_owned(), val.to_owned());
        Ok(self)
    }
    pub fn remove_prop(&mut self, key: &str) -> Result<&Self, String> {
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

pub struct Graph {
    aliases: HashMap<String, NodeIndex>,
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            aliases: HashMap::new(),
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
    pub fn add_node(&mut self, alias: &str) {
        self.aliases
            .insert(alias.to_owned(), self.nodes.len().into());
        self.nodes
            .push(Node::new(self.nodes.len().into(), alias.to_owned()));
    }
    pub fn remove_node_by_id(&mut self, id: &NodeIndex) {
        self.remove_alias_by_id(id);
        //alias removeing has to be before removing of the node
        //because function looks up the alias thru the node at nodes
        self.nodes.retain(|x| x.id.borrow() != id);
        self.remove_all_edges_from(id);
        self.remove_all_edges_to(id);
    }
    pub fn get_node_by_idx(&self, idx: &NodeIndex) -> Option<&Node> {
        self.nodes.get(idx.0)
    }
    pub fn get_node_by_alias(&self, alias: &str) -> Option<&Node> {
        self.aliases
            .get(alias)
            .and_then(|x| self.get_node_by_idx(x))
    }
    pub fn get_alias_by_id(&self, id: &NodeIndex) -> Option<&str> {
        self.get_node_by_idx(id)
            .and_then(|x| Some(x.alias.as_str()))
    }
    pub fn get_id_by_alias(&self, alias: &str) -> Option<&NodeIndex> {
        self.aliases.get(alias)
    }
    fn remove_alias_by_id(&mut self, id: &NodeIndex) {
        let a = self.get_alias_by_id(id).expect("Failed getting alias by id").to_owned();
        self.aliases.remove(&a);
    }
    pub fn add_edge(&mut self, relation: &str, from: NodeIndex, to: NodeIndex) {
        self.edges.push(Edge::new(relation, from, to))
    }
    pub fn add_edge_by_aliases(&mut self, relation: &str, from: &str, to: &str) {
        let fid = self.get_id_by_alias(from);
        let tid = self.get_id_by_alias(to);
        match (fid, tid) {
            (Some(f), Some(t)) => self
                .edges
                .push(Edge::new(relation, f.to_owned(), t.to_owned())),
            _ => {}
        }
    }
    pub fn remove_all_edges_from(&mut self, from: &NodeIndex) {
        self.edges.retain(|x| x.from.borrow() != from)
    }
    pub fn remove_all_edges_to(&mut self, to: &NodeIndex) {
        self.edges.retain(|x| x.to.borrow() != to)
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Nodes:\n")?;
        for n in self.nodes.iter() {
            write!(f, "{}", n);
        }
        write!(f, "Edges:\n")?;
        for e in self.edges.iter() {
            write!(
                f,
                "{:>10}({:02}) {:-^16}> {:<10}[{:02}]\n",
                self.get_alias_by_id(&e.from).expect("Failed getting alias"),
                e.from,
                e.relation,
                self.get_alias_by_id(&e.to).expect("Failed getting alias"),
                e.to
            )?;
        }
        Ok(())
    }
}
