use std::{cell::RefCell, collections::HashMap, rc::Rc};

type Link<T> = Rc<RefCell<T>>;

struct Edge {
    relation: String,
    link: Link<Node>,
}

impl Edge {
    fn new(name: String, node: Node) -> Self {
        Edge {
            relation: name,
            link: Rc::new(RefCell::new(node)),
        }
    }

    fn new_to(name: String, node: Link<Node>) -> Self {
        Edge {
            relation: name,
            link: node,
        }
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.relation == other.relation) && (self.link.borrow().id == other.link.borrow().id)
    }
}

struct Node {
    id: usize,
    alias: String,
    labels: Vec<String>,
    props: HashMap<String, String>,
    rels: Vec<Edge>,
}

impl Node {
    fn new(id: usize, alias: String) -> Self {
        Node {
            id,
            alias,
            labels: Vec::new(),
            props: HashMap::new(),
            rels: Vec::new(),
        }
    }

    fn add_label(&mut self, label: String) {
        self.labels.push(label);
    }
    fn remove_label(&mut self, label: &String) {
        self.labels.retain(|x| x != label)
    }
    fn add_prop(&mut self, key: String, value: String) {
        self.props.insert(key, value);
    }
    fn remove_prop(&mut self, key: &String) {
        self.props.remove(key);
    }
    fn add_relation(&mut self, relation: String, node: &Node) {
        self.rels.push(Edge::new(relation, node))
    }
    fn remove_relation(&mut self, relation: &Edge) {
        self.rels.retain(|x| x != relation)
    }
}

struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    fn new() -> Self {
        Graph { nodes: Vec::new() }
    }
    fn add_node(&mut self, alias: String) {
        self.nodes.push(Node::new(self.nodes.len(), alias))
    }
    fn remove_node_by_alias(&mut self, alias: &String) {
        self.nodes.retain(|x| &x.alias != alias)
    }
    fn remove_node_by_id(&mut self, id: &usize) {
        self.nodes.retain(|x| &x.id != id)
    }
    fn get_node_by_id(&self, id: &usize) -> Option<&Node> {
        self.nodes.iter().find(|x| &x.id == id)
    }
    fn get_node_by_alias(&self, alias: &String) -> Option<&Node> {
        self.nodes.iter().find(|x| &x.alias == alias)
    }
}
