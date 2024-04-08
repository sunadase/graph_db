// use std::{cell::RefCell, collections::HashMap, default, rc::Rc};

// // this impl is mostly along the lines of "A Bad Safe Deque" or so i tried
// struct Node {
//     id: usize,
//     alias: String,
//     labels: Vec<String>,
//     props: HashMap<String,String>,// might be heavy 2 hashmaps for all the nodes?
//     //vecs of string tuples would be wiser probably if there are not many props per node
    
//     //this or Vec<Rc<RefCell<Node>>>>?
//     rels: HashMap<String, Rc<RefCell<Vec<Node>>>>
//     //when the key is string and vals are vec of references
//     //>easier to group by relation type
//     //when the key is node ref and val is vec of strings
//     //>slightly easier to group neighbour relations
//     //again just a vec of pairs probably better most of the time
// }

// impl Node {
//     fn new(id: usize, alias:String, labels:Option<Vec<String>>, props:Option<HashMap<String,String>>) -> Self {
//         Node {
//             id,
//             alias,
//             labels: labels.unwrap_or_default(),
//             props: props.unwrap_or_default(),
//             rels: HashMap::new()
//         }
//     }
//     fn add_label(&mut self, label:&String){
//         self.labels.push(label.to_owned());
//     }

//     fn remove_label(&mut self, label:&String){
//         self.labels.retain_mut(|x| {x != label})
//     }
//     fn add_prop(&mut self, prop:(&String, &String)){
//         self.props.insert(prop.0.to_owned(), prop.1.to_owned());
//     }
//     fn remove_prop(&mut self, prop:(&String, &String)){
//         self.props.remove(prop.1);
//     }
//     fn add_relation(&mut self, name:&String, node:Node){
//         match self.rels.get_mut(name) {
//             Some(v_ref) => {
//                 v_ref.borrow_mut().push(node);
//             },
//             None => {
//                 self.rels.insert(name.to_owned(), Rc::new(RefCell::new(Vec::from([node]))));
//             },
//         }
//     }

//     fn remove_relation(&mut self, name:&String, node:&Node){
//         match self.rels.get_mut(name) {
//             Some(v_ref) => {
//                 v_ref.borrow_mut().retain(|x| x!=node);
//             },
//             None => {
//             },
//         }
//     }

// }

// impl PartialEq for Node {
//     fn eq(&self, other: &Self) -> bool {
//         self.id == other.id
//     }
// }

// struct Graph {
//     //to allow free nodes/nodes with no relations to exist
//     nodes: Vec<Rc<RefCell<Node>>>,
//     size: usize,
// }

// impl Graph {
//     fn new() -> Self{
//         Graph{
//             nodes: Vec::new(),
//             size: 0,
//         }
//     }

//     fn add_node(&mut self, node:Node){
//         //different(2 different new()s 1 here 1@add_relation()) multiple ref cells??
//         self.nodes.push(Rc::new(RefCell::new(node)));
//         self.size += 1
//     }
// }

// struct Edge {
//     name: String,
//     link: Rc<RefCell<Node>>,
// }