
(probably) not working dummy impl. (self reference at rels hashmap val)
-> RefCell?
-> Arc/Rc?
-> Weak?

```rust
struct Node {
    alias: String,
    labels: Vec<String>,
    props: HashMap<String,String>,
    rels: HashMap<String, Vec<Node>>
}
```

RefCell vs Cell:
RefCell<T> borrows, returns the reference to T 
Cell<U> copies, returns a copy: type U must implement Copy trait

RefCell's references are checked at *`runtime`*: 
+ small perf. cost
+ might panic if borrowing logic is wrong
  ↳ interior mutability




design/patterns/structure

- "... you should always prefer using the borrowed type over borrowing the owned type. Such as &str over &String, &[T] over &Vec<T>, or &T over &Box<T>." - (https://rust-unofficial.github.io/patterns/idioms/coercion-arguments.html)

- https://rust-unofficial.github.io/patterns/patterns/structural/compose-structs.html


https://github.com/pnevyk/rusty-graphs

all rust graph libraries seem to share the same/similiar way on how to store edges&nodes
they all in some way store edges and nodes as vecs seperately within the Graph struct
```rust
struct Graph{
  nodes: Vec<Node>
  edges: Vec<Edge>
}
struct Edge{
  from: id/alias,
  to: id/alias,
  relation/weigth/... : String/Float/...
}
```
If we try to store edges not with identifiers like id/index/alias but with Rcs, in the situation of nodes: 
a,b,c a->b, c->b 
We need to have 2 Rc's to node b, one at node a, one at node c: 
- if we make the add_link a method of nodes we dont have the a's Rc to clone at b' or vice versa, then we have to create another rc of c which is forbidden(?)(two different rc's of c).
- Then we need to make add_link a method of Graph, store the Rc's as Vec there -> Graph owns all the nodes(enabling seperate/unlinked nodes to exist) and no nodes are dropped until They are discarded from all the node links and finally from the graph
: i dont like not being able to make add_link a method of nodes, is there a way?
: id like to be able to do
: graph.get_node("ali").add_link("knows", Node::new("veli"))
: maybe?
: graph.get_node("ali").add_link("knows", graph.add_node("veli").get_link())

storing edges as a Vec within Graph with from, to, relation information feels trivial



srcs:
og
https://rust-unofficial.github.io/too-many-lists/fourth.html


advanced + cool + proper
https://aminb.gitbooks.io/rust-for-c/content/graphs/index.html


https://badboi.dev/rust/2020/07/17/cell-refcell.html


https://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/


[youtube/Are Graphs Hard in Rust?](https://www.youtube.com/watch?v=kGaU5kU-5rw)
