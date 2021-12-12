use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

const INPUT: &str = include_str!("./day12.txt");
type NodeId = u8;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum NodeKind {
    Small,
    Big,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Node(NodeId, NodeKind);

#[derive(Debug)]
struct Graph {
    nodes: Vec<Node>,
    node_id: NodeId,

    neighbors: HashMap<NodeId, Vec<NodeId>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            node_id: 0,

            neighbors: HashMap::new(),
        }
    }

    fn create_node(&mut self, kind: NodeKind) -> Node {
        let node = Node(self.node_id, kind);
        self.node_id += 1;
        self.nodes.push(node);
        node
    }

    fn add_link(&mut self, from: Node, to: Node) {
        self.neighbors.entry(from.0).or_default().push(to.0);
        self.neighbors.entry(to.0).or_default().push(from.0);
    }
}

#[derive(Debug, Clone)]
struct Path(Vec<NodeId>, bool);

impl Path {
    fn current(&self) -> NodeId {
        *self.0.last().unwrap()
    }
}

fn get_node_kind(name: &str) -> NodeKind {
    if name.chars().all(|ch| ch.is_uppercase()) {
        NodeKind::Big
    } else {
        NodeKind::Small
    }
}

fn load_graph() -> (
    Graph,
    HashMap<&'static str, Node>,
    HashMap<NodeId, &'static str>,
) {
    let mut graph = Graph::new();
    let mut name_to_node = HashMap::new();
    let mut node_to_name = HashMap::new();

    let input = INPUT
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .collect_vec();
    for (from, to) in input {
        name_to_node
            .entry(from)
            .or_insert_with(|| graph.create_node(get_node_kind(from)));

        name_to_node
            .entry(to)
            .or_insert_with(|| graph.create_node(get_node_kind(to)));

        let from_node = name_to_node[&from];
        let to_node = name_to_node[&to];

        node_to_name.insert(from_node.0, from);
        node_to_name.insert(to_node.0, to);

        graph.add_link(from_node, to_node);
    }

    (graph, name_to_node, node_to_name)
}

pub fn a() {
    let (graph, name_to_node, _) = load_graph();

    let start = name_to_node["start"];
    let end = name_to_node["end"];

    let mut finished_paths = Vec::new();

    let mut path_queue = VecDeque::from([Path(vec![start.0], false)]);

    while !path_queue.is_empty() {
        let current_path = path_queue.pop_front().unwrap();
        let current_node = current_path.current();
        let current_node = graph.nodes[current_node as usize];

        if current_node == end {
            finished_paths.push(current_path);
        } else {
            for &neighbor in &graph.neighbors[&current_node.0] {
                let neighbor_node = graph.nodes[neighbor as usize];
                if neighbor_node.1 == NodeKind::Big || !current_path.0.contains(&neighbor) {
                    let mut new_path = current_path.0.clone();
                    new_path.push(neighbor);
                    path_queue.push_back(Path(new_path, false));
                }
            }
        }
    }

    println!("Day12a: {}", finished_paths.len());
}

pub fn b() {
    let (graph, name_to_node, _node_to_name) = load_graph();

    let start = name_to_node["start"];
    let end = name_to_node["end"];

    let mut finished_paths = Vec::new();

    let mut path_queue = VecDeque::from([Path(vec![start.0], true)]);

    while !path_queue.is_empty() {
        let current_path = path_queue.pop_front().unwrap();
        let current_node = current_path.current();
        let current_node = graph.nodes[current_node as usize];
        let can_visit_twice = current_path.1;

        if current_node == end {
            finished_paths.push(current_path);
        } else {
            for &neighbor in &graph.neighbors[&current_node.0] {
                let neighbor_node = graph.nodes[neighbor as usize];

                if neighbor_node == start {
                    continue;
                }

                if neighbor_node.1 == NodeKind::Big {
                    let mut new_path = current_path.0.clone();
                    new_path.push(neighbor);
                    path_queue.push_back(Path(new_path, can_visit_twice));
                } else {
                    let visited_neighbor = current_path.0.contains(&neighbor);
                    let is_second_visit = visited_neighbor && can_visit_twice;

                    if !visited_neighbor || is_second_visit {
                        let mut new_path = current_path.0.clone();
                        new_path.push(neighbor);
                        path_queue.push_back(Path(
                            new_path,
                            if is_second_visit {
                                false
                            } else {
                                can_visit_twice
                            },
                        ));
                    }
                }
            }
        }
    }

    /*for path in &finished_paths {
        for (i, node) in path.0.iter().enumerate() {
            if i > 0 {
                print!(",");
            }
            let name = node_to_name[node];
            print!("{}", name);
        }
        println!();
    }*/

    println!("Day12b: {}", finished_paths.len());
}
