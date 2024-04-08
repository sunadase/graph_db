[notes](notes.md)

[vec based graph](/src/vec_graph.rs)
- currently most functional

missing:
- search?
- \> find by label/props
- Errors/Result
- tests
- some methods only available from graph or node: weird design
- [couldnt fix this](/src/vec_graph.rs#L127) seemed to work? when i wrap HashMap in another struct based on [this](https://rust-unofficial.github.io/patterns/patterns/structural/compose-structs.html): AliasMap(Hashmap<..>) but i need to impl/wrap all used hashmap funcs?
- can't get a node from graph and mutate it and chain funcs?
```rust 
//cannot borrow data in a `&` reference as mutable: cannot borrow as mutable
graph
    .get_node_by_alias("you")//Option<&Node>
    .expect("Failed getting node you")//&Node
    .add_label("Person")//Option<&Node>  <fails here
    .expect("Failed adding label")//&Node
    .add_prop("name", "elma")//Option<&Node>
    .expect("Failed adding prop");//&Node
```

[rc graph](/src/rc_graph.rs)
wip
[unsafe graph](/src/unsafe_graph.rs)
wip