///MATCH (you:Person {name:"elma"})-[:KNOWS]->(n)-[:KNOWS]->(friend:Person {name:"armut"})
//RETURN n

//QNode(
    //   alias="you",
    //    labels=["Person"],
    //    props=dict(name="elma"),
    //    rels=dict(
        //        knows=[QNode(
            //            alias="n",
            //            props=dict(name="armut"),
            //            rels=dict(
                //                knows=[QNode(alias="friend")])
//        )],
//    )
//),


//mod rc_graph;
mod unsafe_graph;
mod vec_graph;

fn main() {
    {
        use vec_graph::Graph;

        let mut graph = Graph::new();
        println!("{}", graph);
        graph.add_node("you");
        println!("{}", graph);

        //cannot borrow data in a `&` reference as mutable: cannot borrow as mutable
        // graph
        //     .get_node_by_alias("you")
        //     .expect("Failed getting node you")
        //     .add_label("Person")
        //     .expect("Failed adding label")
        //     .add_prop("name", "elma")
        //     .expect("Failed adding prop");
        graph.add_node("n");
        println!("{}", graph);
        graph.add_node("friend");
        println!("{}", graph);
        graph.add_edge_by_aliases("knows", "you", "n");
        println!("{}", graph);
        graph.add_edge(
            "knows",
            graph
                .get_id_by_alias("n")
                .expect("Couldn't get n's id by alias")
                .to_owned(),
            graph
                .get_node_by_alias("friend")
                .expect("Couldn't get node friend")
                .id,
        );
        println!("{}", graph);
    }
}
