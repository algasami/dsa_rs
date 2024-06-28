use dsa_rs::graph::Graph;
use text_io::read;

fn main() {
    let mut g: Graph<i32> = Graph::new();

    println!("Input <nodes> <edges> on the next line");
    let nodes: usize = read!();
    let edges: usize = read!();
    for _i in 0..nodes {
        g.add_node(0);
    }
    for _e in 0..edges {
        let key1: usize = read!();
        let key2: usize = read!();
        match g.add_edge(key1, key2) {
            Err(_) => {
                println!("Add edge exception!");
            }
            Ok(()) => {}
        }
    }
    println!("{}", g);
}
