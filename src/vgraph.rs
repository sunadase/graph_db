use std::collections::HashMap;

struct NodeIndex(usize);
impl NodeIndex {
    fn new(index: usize) -> Self {
        NodeIndex(index)
    }
}
impl From<usize> for NodeIndex {
    fn from(value: usize) -> Self {
        NodeIndex(value)
    }
}

struct Link {
    relation: String,
    to: NodeIndex,
}
impl Link {
    fn new(relation: &str, to: NodeIndex) -> Self {
        Link {
            relation: relation.to_owned(),
            to,
        }
    }
}

struct Node {
    pub alias: String,
    labels: Vec<String>,
    props: HashMap<String, String>,
    rels: Vec<Link>, //outgoing node
}
impl Node {
    fn new(alias: String) -> Self {
        Node {
            alias: alias,
            labels: Vec::new(),
            props: HashMap::new(),
            rels: Vec::new(),
        }
    }
    fn add_label(&mut self, label: &str) -> &mut Self {
        self.labels.push(label.to_owned());
        self
    }
    fn remove_label(&mut self, label: &str) -> &mut Self {
        self.labels.retain(|x| x != label);
        self
    }
    fn add_prop(&mut self, key: &str, val: &str) -> &mut Self {
        self.props.insert(key.to_owned(), val.to_owned());
        self
    }
    fn remove_prop(&mut self, key: &str) -> &mut Self {
        self.props.remove(key);
        self
    }
    fn add_relation(&mut self, relation: &str, to: NodeIndex) -> &mut Self {
        self.rels.push(Link::new(relation, to));
        self
    }
    fn remove_relation(&mut self, relation: &str) -> &mut Self {
        self.rels.retain(|x| x.relation != relation);
        self
    }
}

struct Graph {
    nodes: Vec<Node>,
    names: HashMap<String, NodeIndex>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            names: HashMap::new(),
        }
    }
    ///aliases are unique
    fn add_node(&mut self, alias: &str) -> &mut Self {
        if self.names.contains_key(alias) {
            //skips silently?
            self
        } else {
            self.nodes.push(Node::new(alias.to_owned()));
            self.names.insert(alias.to_owned(), self.nodes.len().into());
            self
        }
    }
    fn remove_node(&mut self, alias: &str) -> &mut Self {
        self.names.remove(alias).map(|x| {
            self.nodes.swap_remove(x.0);
            self.nodes
                .get(x.0)
                .map(|n| self.names.insert(n.alias.to_owned(), x));
        });
        self
    }
    fn get_node_mut(&mut self, alias: &str) -> &mut Node {
        self.nodes
            .get_mut(self.names.get(alias).unwrap().0)
            .unwrap()
    }
    fn get_node(&self, alias: &str) -> &Node {
        self.nodes.get(self.names.get(alias).unwrap().0).unwrap()
    }
    fn mut_node<F>(&mut self, alias: &str, mut f: F) -> &mut Graph
    where
        F: FnMut(&mut Node),
    {
        f(self.get_node_mut(alias));
        self
    }
}
