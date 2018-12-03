use regex::Regex;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let input = input();
    let tower = Tower::from_input(&input);
    tower.root().name.clone()
}

fn answer_two() -> String {
    let input = input();
    let tower = Tower::from_input(&input);
    tower.imbalanced_node_corrected_weight().to_string()
}

#[derive(Debug)]
struct Tower {
    nodes: HashSet<Node>,
}

impl Tower {
    fn from_input(input: &str) -> Tower {
        let mut nodes = HashMap::new();
        let node_regex = Regex::new(r"^(\w+) \((\d+)\)( -> (.*))?$").unwrap();
        let mut children_strings = HashMap::new();

        let lines: Vec<_> = input.lines().collect();
        for line in lines {
            let captures = node_regex.captures(line).unwrap();
            let node_name = captures.get(1).unwrap().as_str().trim();
            let node_weight = captures.get(2).unwrap().as_str().parse().unwrap();
            let node_children = captures.get(4).map(|c| c.as_str());

            nodes.insert(node_name, Node::new(node_name, node_weight));
            if let Some(c) = node_children {
                children_strings.insert(node_name, c);
            }
        }

        for (parent_name, child_string) in children_strings {
            for child_name in child_string.split(", ") {
                let child_node_name = nodes.get(child_name).unwrap().name.to_owned();
                nodes
                    .get_mut(parent_name)
                    .unwrap()
                    .children
                    .push(child_node_name);
            }
        }

        Tower {
            nodes: nodes.into_iter().map(|(_, v)| v).collect(),
        }
    }

    fn node_by_name(&self, name: &str) -> Option<&Node> {
        self.nodes.iter().find(|n| n.name == name)
    }

    fn parent(&self, node: &Node) -> Option<&Node> {
        self.nodes.iter().find(|n| n.children.contains(&node.name))
    }

    fn children(&self, node: &Node) -> Vec<&Node> {
        node.children
            .iter()
            .map(|n| self.node_by_name(n).unwrap())
            .collect()
    }

    fn siblings(&self, node: &Node) -> Vec<&Node> {
        self.parent(node)
            .map(|p| {
                self.children(p)
                    .into_iter()
                    .filter(|&c| c != node)
                    .collect()
            }).unwrap_or(vec![])
    }

    fn root(&self) -> &Node {
        self.nodes
            .iter()
            .find(|n| self.parent(&n).is_none())
            .unwrap()
    }

    fn imbalanced_node_corrected_weight(&self) -> u32 {
        let mut imbalanced_node = self.root();
        while let Some(imbalanced_child) =
            frequencies_by(self.children(imbalanced_node), |c| self.total_weight(c))
                .get(&1)
                .map(|c| c[0])
        {
            imbalanced_node = imbalanced_child;
        }
        let imbalanced_node_total_weight = self.total_weight(imbalanced_node);
        let sibling_total_weight = self.total_weight(self.siblings(imbalanced_node)[0]);
        imbalanced_node.weight + sibling_total_weight - imbalanced_node_total_weight
    }

    fn total_weight(&self, node: &Node) -> u32 {
        node.weight + self
            .children(node)
            .iter()
            .map(|c| self.total_weight(c))
            .sum::<u32>()
    }
}

fn frequencies_by<I, T, U, F>(items: I, mut f: F) -> HashMap<usize, Vec<T>>
where
    I: IntoIterator<Item = T>,
    U: Ord + Eq,
    F: FnMut(&T) -> U,
{
    let mut values_items = BTreeMap::new();
    for item in items {
        let i = values_items.entry(f(&item)).or_insert(vec![]);
        i.push(item);
    }

    let mut result = HashMap::new();
    for (_, items) in values_items {
        let r = result.entry(items.len()).or_insert(vec![]);
        r.extend(items);
    }

    result
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Node {
    name: String,
    weight: u32,
    children: Vec<String>,
}

impl Node {
    fn new(name: &str, weight: u32) -> Node {
        Node {
            name: name.to_owned(),
            weight: weight,
            children: vec![],
        }
    }
}

fn input() -> String {
    let mut file = File::open("src/year_2017/input/input_day_7").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string.trim().to_owned()
}
