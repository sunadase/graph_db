[notes](notes.md)

[vec based graph](/src/vec_graph.rs)
- currently most functional

missing:
- search?
- \> find by label/props 
- tests
- some methods only available from graph or node: weird design
    - being able to get/search nodes and retrieve them but not being able to add edges directly from them (need to pass their and targets id/alias to graph.add_edge()) is ugly
- when nodes with the same alias are added: they are added as separate nodes into vec but aliasmap gets overwritten with the id of the last one, making others inaccessible
    - \> either aliasmap could point to vec of nodes and aliases wouldnt have to be unique
    - \> or dont let them insert nodes with the same alias

[rc graph](/src/rc_graph.rs)
- wip

[unsafe graph](/src/unsafe_graph.rs)
- wip