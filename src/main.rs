use std::{
    cmp::min,
    collections::{HashMap, HashSet},
};

use dsa_rs::graph::{Graph, NodeT};
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
    let aps = tarjan(&g);
    print!("aps: ");
    for x in &aps {
        print!("{},", x);
    }
    println!();
}

struct NodeDescriptor {
    lowpoint: usize,
    discovery_time: usize,
}

impl Clone for NodeDescriptor {
    fn clone(&self) -> Self {
        Self {
            lowpoint: self.lowpoint,
            discovery_time: self.discovery_time,
        }
    }
}
type NodeDescriptors = HashMap<NodeT, NodeDescriptor>;

fn tarjan<T>(g: &Graph<T>) -> HashSet<NodeT> {
    let mut aps: HashSet<NodeT> = HashSet::new();
    let mut node_descriptors: NodeDescriptors = HashMap::new();

    for (node_id, _node) in &g.nodes {
        if node_descriptors.get(node_id).is_some() {
            continue;
        }
        tarjan_helper(g, node_id, node_id, &mut node_descriptors, &mut aps, &mut 0);
    }

    aps
}

fn tarjan_helper<T>(
    g: &Graph<T>,
    current: &NodeT,
    last: &NodeT,
    descriptors: &mut NodeDescriptors,
    aps_out: &mut HashSet<NodeT>,
    global_time: &mut usize,
) {
    let Some(node_data) = g.nodes.get(current) else {
        return;
    };
    let is_root = current == last;
    *global_time += 1;
    descriptors.insert(
        current.clone(),
        NodeDescriptor {
            lowpoint: global_time.clone(),
            discovery_time: global_time.clone(),
        },
    );

    let mut children: usize = 0;

    for next_node in &node_data.edges {
        if descriptors.get(next_node).is_none() {
            children += 1;
            tarjan_helper(g, next_node, current, descriptors, aps_out, global_time);
            let child_desc = (*descriptors.get(next_node).unwrap()).clone();
            let node_desc = descriptors.get_mut(current).unwrap();
            node_desc.lowpoint = min(node_desc.lowpoint, child_desc.lowpoint);

            if !is_root && node_desc.discovery_time <= child_desc.lowpoint {
                aps_out.insert(*current);
            }
        } else {
            let child_desc = (*descriptors.get(next_node).unwrap()).clone();
            let node_desc = descriptors.get_mut(current).unwrap();
            node_desc.lowpoint = min(node_desc.lowpoint, child_desc.lowpoint);
        }
    }

    if is_root && children >= 2 {
        aps_out.insert(*current);
    }
}
