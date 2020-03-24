use itertools::Itertools;
use std::collections::HashMap;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let graph = build_graph(&input());
    graph
        .nodes()
        .iter()
        .permutations(2)
        .map(|nodes| {
            graph
                .all_simple_paths(nodes[0], nodes[1])
                .iter()
                .filter(|p| p.len() == graph.nodes().len() - 1)
                .map(|p| p.iter().map(|&(_, _, e)| e).sum::<usize>())
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
        .to_string()
}

fn answer_two() -> String {
    let graph = build_graph(&input());
    graph
        .nodes()
        .iter()
        .permutations(2)
        .map(|nodes| {
            graph
                .all_simple_paths(nodes[0], nodes[1])
                .iter()
                .filter(|p| p.len() == graph.nodes().len() - 1)
                .map(|p| p.iter().map(|&(_, _, e)| e).sum::<usize>())
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
        .to_string()
}

fn input() -> String {
    std::fs::read_to_string("src/year_2015/input/input_day_9").unwrap()
}

fn build_graph(input: &str) -> Graph<String, usize> {
    let edges = input.lines().map(|l| {
        let mut parts = l.split(" ");
        let first = parts.next().unwrap();
        parts.next();
        let second = parts.next().unwrap();
        parts.next();
        let distance = parts.next().unwrap();
        (
            first.to_owned(),
            second.to_owned(),
            distance.parse::<usize>().unwrap(),
        )
    });
    Graph::from_edges(edges)
}

#[derive(Debug)]
pub struct Graph<N, E> {
    nodes: Vec<N>,
    edges: HashMap<(usize, usize), E>,
}

impl<N, E> Graph<N, E>
where
    N: PartialEq,
{
    pub fn new() -> Graph<N, E> {
        Graph {
            nodes: vec![],
            edges: HashMap::new(),
        }
    }

    pub fn from_edges<I>(edges: I) -> Graph<N, E>
    where
        I: IntoIterator<Item = (N, N, E)>,
    {
        let mut graph = Graph::new();

        for (n1, n2, e) in edges {
            let index1 = graph
                .nodes
                .iter()
                .position(|n| n == &n1)
                .unwrap_or_else(|| {
                    graph.nodes.push(n1);
                    graph.nodes.len() - 1
                });
            let index2 = graph
                .nodes
                .iter()
                .position(|n| n == &n2)
                .unwrap_or_else(|| {
                    graph.nodes.push(n2);
                    graph.nodes.len() - 1
                });
            graph.edges.insert((index1, index2), e);
        }

        graph
    }

    pub fn nodes(&self) -> &[N] {
        &self.nodes
    }

    fn node_at_index(&self, index: usize) -> Option<&N> {
        self.nodes.get(index)
    }

    pub fn neighbours(&self, node: &N) -> Vec<(&N, &E)> {
        self.edges
            .iter()
            .map(|(&(f, t), w)| {
                (
                    self.node_at_index(f).unwrap(),
                    self.node_at_index(t).unwrap(),
                    w,
                )
            })
            .filter(|&(f, t, _)| f == node || t == node)
            .map(|(f, t, w)| if f == node { (t, w) } else { (f, w) })
            .collect()
    }

    pub fn all_simple_paths<'a>(
        &'a self,
        from: &'a N,
        to: &'a N,
    ) -> Vec<Vec<(&'a N, &'a N, &'a E)>> {
        self.all_simple_paths_impl(from, to, &mut vec![from])
    }

    fn all_simple_paths_impl<'a>(
        &'a self,
        from: &'a N,
        to: &'a N,
        visited: &mut Vec<&'a N>,
    ) -> Vec<Vec<(&'a N, &'a N, &'a E)>> {
        let mut paths = vec![];

        for (neighbour, weight) in self.neighbours(from) {
            if neighbour == to {
                paths.push(vec![(from, to, weight)]);
            } else if !visited.contains(&neighbour) {
                visited.push(neighbour);
                let mut neighbour_paths = self.all_simple_paths_impl(neighbour, to, visited);
                for p in neighbour_paths.iter_mut() {
                    p.insert(0, (from, neighbour, weight));
                }
                paths.append(&mut neighbour_paths);
                visited.pop().unwrap();
            }
        }

        paths
    }
}
