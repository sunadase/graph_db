[notes](notes.md)

[vec based graph](/src/vec_graph.rs)
- currently most functional

missing:
- search?
- \> find by label/props 
- tests
- some methods only available from graph or node: weird design
- when same nodes with same alias are added: they are added as separate nodes but aliasmap gets overwritten with the last one, making others inaccessible
- \> either aliasmap could point to vec of nodes and aliases wouldnt have to be unique
- \> or dont let them insert nodes with the same alias

[rc graph](/src/rc_graph.rs)
- wip

[unsafe graph](/src/unsafe_graph.rs)
- wip