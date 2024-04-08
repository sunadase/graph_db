use std::collections::HashMap;

type Link = *mut Node;

struct Node {
    id: usize,
    alias: String,
    labels: Vec<String>,
    props: HashMap<String, String>,
    rels: Vec<Edge>,
}

struct Edge {
    to: Link,
    relation: String,
}

struct Graph {
    nodes: Vec<Node>,
}
