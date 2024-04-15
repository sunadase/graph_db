[notes](notes.md)

[vec based graph](/src/vec_graph.rs)
- currently most functional

missing:
- search?
- \> find by label/props 
- tests
    - \> more + edges relations removes + same alias
- some methods only available from graph or node: weird design
    - being able to get/search nodes and retrieve them but not being able to add edges directly from them (need to pass their and targets id/alias to graph.add_edge()) is ugly
        - \> Weak ref to parent?
        - \> Edges are stored as vec Node Indices at Node like NextNodes(to_id, relation), instead of Edge(from,to,relation) at graph: weird locality?
- when nodes with the same alias are added: they are added as separate nodes into vec but aliasmap gets overwritten with the id of the last one, making others inaccessible
    - \> either aliasmap could point to vec of nodes and aliases wouldnt have to be unique
        - i choose to go with this since it's more sensible, searches etc also will return a group of nodes, getting by alias is kind of like a search and i dont think word "alias" is used for neccessarily unique values
    - \> or dont let them insert nodes with the same alias

- bug: node_get uses index but index changes with remove, the rest(aliasmap, nodes) use ids which are equal to index at node creation but diverge after indexes slide by a node remove
    - \> either dont slide after remove (swap_remove), then readjust ids for the swap (vec<edge(id,id)>, hashmap<string, id>, node(id,...))
        - \> i chose this 
    - \> or check ids on node lookup (since index only slides left on removes? if the id at index doesnt match iterate backwards till find the match?)
        - \> no point in using indexes if not utilizing its hash-likeness

[vgraph](/src/vgraph.rs)
- vector graph but relations are stored in nodes as vec of outgoing edges(to, relation:str)
- wip

[rc graph](/src/rc_graph.rs)
- wip

[unsafe graph](/src/unsafe_graph.rs)
- wip