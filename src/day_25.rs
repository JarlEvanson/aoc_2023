use fxhash::{FxHashMap, FxHashSet};
use rand::distributions::Distribution;

pub fn solve(input: &str) -> (usize, usize) {
    let nodes = input.lines().map(|line| {
        let (name, ends) = line
            .split_once(':')
            .map(|(name, ends)| (name, ends.split_whitespace()))
            .unwrap();

        (name, ends)
    });

    let mut graph = Vec::new();

    let mut node_names = FxHashMap::default();

    let mut edges = Vec::new();

    for (name, end_nodes) in nodes {
        let index = lookup_base(&mut node_names, &mut graph, name);

        for connection in end_nodes {
            let other = lookup_base(&mut node_names, &mut graph, connection);

            edges.push((index, other));

            graph[index].connections.insert(other);
            graph[other].connections.insert(index);
        }
    }

    loop {
        let (connection_count, result) = min_cut(graph.clone(), edges.clone());

        if connection_count == 3 {
            return (result, 0);
        }
    }
}

fn lookup_base<'node>(
    node_names: &mut FxHashMap<&'node str, usize>,
    nodes: &mut Vec<Node>,
    name: &'node str,
) -> usize {
    if let std::collections::hash_map::Entry::Vacant(e) = node_names.entry(name) {
        e.insert(nodes.len());
        nodes.push(Node {
            status: Status::Container(vec![nodes.len()]),
            connections: FxHashSet::default(),
        });
    }

    *node_names.get(&name).unwrap()
}

fn min_cut(mut graph: Vec<Node>, mut edges: Vec<(usize, usize)>) -> (usize, usize) {
    let mut random = rand::thread_rng();

    let mut active_nodes = Vec::from_iter(0..graph.len());

    while active_nodes.len() > 2 {
        let selected_edge = rand::distributions::Uniform::new(0, edges.len()).sample(&mut random);

        let (og_a_index, og_b_index) = edges[selected_edge];

        edges.swap_remove(selected_edge);

        let a_index = resolve(&graph, og_a_index);
        let b_index = resolve(&graph, og_b_index);

        let Ok([a, b]) = graph.get_many_mut([a_index, b_index]) else {
            if a_index == b_index {
                continue;
            }
            panic!();
        };

        let all_contained = match (&mut a.status, &mut b.status) {
            (Status::Container(a), Status::Container(b)) => {
                let mut vec = core::mem::take(a);

                vec.extend(core::mem::take(b));

                vec
            }
            _ => unreachable!(),
        };

        let mut all_connections = core::mem::take(&mut a.connections);

        all_connections.extend(core::mem::take(&mut b.connections));

        graph[a_index].status = Status::Redirect(graph.len());
        graph[b_index].status = Status::Redirect(graph.len());

        graph.push(Node {
            status: Status::Container(all_contained),
            connections: all_connections,
        });

        active_nodes.retain_mut(|&mut index| index != a_index && index != b_index);
        active_nodes.push(graph.len() - 1);
    }

    let mut connection_count = 0;

    let b_set;

    if let Status::Container(c) = &mut graph[active_nodes[1]].status {
        b_set = FxHashSet::from_iter(c.iter().copied());
    } else {
        b_set = FxHashSet::default();
    }

    for &connection in graph[active_nodes[0]].connections.iter() {
        if b_set.contains(&connection) {
            connection_count += 1;
        }
    }

    let product = active_nodes
        .iter()
        .map(|index| {
            if let Status::Container(c) = &graph[*index].status {
                c.len()
            } else {
                0
            }
        })
        .product::<usize>();

    (connection_count, product)
}

fn resolve(graph: &[Node], mut index: usize) -> usize {
    while let Status::Redirect(new) = graph[index].status {
        index = new;
    }

    index
}

#[derive(Clone, Debug)]
struct Node {
    status: Status,
    connections: FxHashSet<usize>,
}

#[derive(Clone, Debug)]
enum Status {
    Redirect(usize),
    Container(Vec<usize>),
}
