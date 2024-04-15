use graph_db::vec_graph::*;

fn check_alias(target: &Node, expected: &str) {
    assert_eq!(target.alias, expected);
}
fn check_id(target: &Node, expected: NodeIndex) {
    assert_eq!(target.id, expected)
}

#[test]
fn create_node() {
    let mut graph = Graph::new();
    let target = graph
        .add_node("test")
        .expect("Failed adding node")
        .get_last_node()
        .expect("Failed getting last node");
    check_alias(target, "test");
    check_id(target, NodeIndex::from(0));
}

#[test]
fn create_node_twice() {
    let mut graph = Graph::new();
    let first = graph
        .add_node("first")
        .expect("Failed adding first node")
        .get_last_node()
        .expect("Failed getting last node: first");
    check_alias(first, "first");
    check_id(first, NodeIndex::from(0));
    let second = graph
        .add_node("second")
        .expect("Failed adding second node")
        .get_last_node()
        .expect("Failed getting last node: second");
    check_alias(second, "second");
    check_id(second, NodeIndex::from(1));
    let first_again = graph
        .get_node_by_idx(&NodeIndex::from(0))
        .expect("Couldnt get first node by index: 0");
    check_alias(first_again, "first");
    check_id(first_again, NodeIndex::from(0));
}

#[test]
fn create_node_same_name() {
    let mut graph = Graph::new();
    let target = graph
        .add_node("test")
        .expect("Failed adding node")
        .get_last_node()
        .expect("Failed getting last node");
    check_alias(target, "test");
    check_id(target, NodeIndex::from(0));
    let target2 = graph
        .add_node("test")
        .expect("Failed adding node")
        .get_last_node()
        .expect("Failed getting last node");
    check_alias(target2, "test");
    check_id(target2, NodeIndex::from(1));
    let target_again = graph
        .get_node_by_idx(&NodeIndex::from(0))
        .expect("Failed getting first node again at idx:0");
    check_alias(target_again, "test");
    check_id(target_again, NodeIndex::from(0));
}

#[test]
fn check_node_eq() {
    let mut graph1 = Graph::new();
    let mut graph2 = Graph::new();

    graph1
        .add_node("test")
        .expect("Error adding node to graph1");
    graph2
        .add_node("test")
        .expect("Error adding node to graph2");

    ///equality derived -> Checks if all fields of all structs are equal
    assert_eq!(graph1, graph2);
    assert_eq!(
        graph1
            .get_last_node()
            .expect("Graph1 failed getting first node"),
        graph2
            .get_last_node()
            .expect("Graph2 failed getting last node")
    );
}

#[test]
fn node_remove() {
    let mut graph1 = Graph::new();
    let mut graph2 = Graph::new();

    graph1
        .add_node("ali")
        .unwrap()
        .add_node("veli")
        .unwrap()
        .add_node("deli")
        .unwrap()
        .add_node("kkk")
        .unwrap()
        .add_node("sss")
        .unwrap();

    let did = graph1
        .get_ids_by_alias("deli")
        .unwrap()
        .first()
        .unwrap()
        .clone();

    println!("pre remove graph:\n{}",graph1);
    println!("{:#?}",graph1.aliases);
    graph1.remove_node_by_id(&did).unwrap();
    
    println!("post remove graph:\n{}",graph1);
    println!("{:#?}",graph1.aliases);

    let veli = *graph1.get_nodes_by_alias("veli").unwrap().first().unwrap();
    let kkk = *graph1.get_nodes_by_alias("kkk").unwrap().first().unwrap();
    let sss = *graph1.get_nodes_by_alias("sss").unwrap().first().unwrap();
    assert_eq!(veli, &Node::new(1.into(), "veli".to_owned()));
    assert_eq!(kkk, &Node::new(3.into(), "kkk".to_owned()));
    assert_eq!(sss, &Node::new(2.into(), "sss".to_owned()));
}
