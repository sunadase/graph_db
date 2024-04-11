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
pub mod vec_graph;

fn main() -> vec_graph::GraphResult<()> {
    {
        use vec_graph::Graph;

        let mut graph = Graph::new();
        graph
            .add_node("sisli")?
            .get_last_node_mut()
            .and_then(|sisli| sisli.add_label("sehir").ok()?.add_prop("tur", "ilce").ok())
            .ok_or("err")?;

        println!("{}", graph);

        graph
            .add_node("mcdkoy")?
            .mut_last_node(|mcd| {
                mcd.add_label("mahalle")?
                    .add_label("bolge")?
                    .add_prop("tur", "mahalle")
            })?
            .add_node("merkez")?
            .mut_last_node(|merkez| {
                merkez
                    .add_label("mahalle")?
                    .add_label("bolge")?
                    .add_prop("tur", "mahalle")
            })?
            .add_edges_by_aliases("includes", "sisli", "merkez")?
            .add_edges_by_aliases("includes", "sisli", "mcdkoy")?
            .add_node("merkez")?
            .mut_last_node(|merkez| {
                merkez
                    .add_label("mahalle")?
                    .add_label("diger")?
                    .add_prop("tur", "mahalle")?
                    .add_prop("test", "alternatif")?
                    .add_prop("test", "alt")
            })?
            .add_edges_by_aliases("ayni", "merkez", "merkez")?
            .add_edges_by_aliases("komsu", "merkez", "mcdkoy")?
            .add_edges_by_aliases("komsu", "mcdkoy", "merkez")?;

        println!("{}", graph);

        let target_alias = "sisli";
        println!(
            "outgoings neighbors of {} ->\n{:#?}",
            target_alias,
            graph.get_outgoing_neighbors(
                graph
                    .get_nodes_by_alias(target_alias)
                    .expect(format!("cant find {}?", target_alias).as_str())
                    .first()
                    .expect("no first in vec?")
            )
        );

        println!(
            "incoming neighbors of {} ->\n{:#?}",
            target_alias,
            graph.get_outgoing_neighbors(
                graph
                    .get_nodes_by_alias(target_alias)
                    .expect(format!("cant find {}?", target_alias).as_str())
                    .first()
                    .expect("no first in vec?")
            )
        );

        Ok(())
    }
}
