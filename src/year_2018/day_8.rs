use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let input = input();

    get_nodes(&input)
        .values()
        .flat_map(|(_, m)| m)
        .sum::<usize>()
        .to_string()
}

fn answer_two() -> String {
    let input = input();
    let nodes = get_nodes(&input);
    get_node_value(0, &nodes).to_string()
}

fn get_nodes(input: &str) -> HashMap<usize, (Vec<usize>, Vec<usize>)> {
    let mut numbers: Vec<_> = input
        .split(' ')
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    let mut nodes = HashMap::new();
    fill_nodes(&mut numbers, &mut 0, &mut nodes, None, 1, 0);

    nodes
}

fn fill_nodes(
    mut numbers: &mut Vec<usize>,
    current_index: &mut usize,
    mut nodes: &mut HashMap<usize, (Vec<usize>, Vec<usize>)>,
    parent_id: Option<usize>,
    parent_child_count: usize,
    parent_metadata_count: usize,
) {
    for _ in 0..parent_child_count {
        let child_count = numbers[*current_index];
        let metadata_count = numbers[*current_index + 1];
        *current_index += 2;

        let node_id = nodes.len();
        nodes.insert(node_id, (vec![], vec![]));
        if let Some(parent_id) = parent_id {
            (*nodes.get_mut(&parent_id).unwrap()).0.push(node_id);
        }

        fill_nodes(
            &mut numbers,
            current_index,
            &mut nodes,
            Some(node_id),
            child_count,
            metadata_count,
        );
    }

    if let Some(parent_id) = parent_id {
        let metadata = numbers
            .iter()
            .skip(*current_index)
            .take(parent_metadata_count)
            .cloned()
            .collect();
        (*nodes.get_mut(&parent_id).unwrap()).1 = metadata;
        *current_index += parent_metadata_count;
    }
}

fn get_node_value(node_id: usize, nodes: &HashMap<usize, (Vec<usize>, Vec<usize>)>) -> usize {
    let children = &nodes[&node_id].0;
    let metadata = &nodes[&node_id].1;

    if children.is_empty() {
        return metadata.iter().sum();
    }

    metadata
        .iter()
        .filter_map(|&m| {
            children
                .get(m.wrapping_sub(1))
                .map(|&c| get_node_value(c, &nodes))
        })
        .sum()
}

fn input() -> String {
    let mut file = File::open("src/year_2018/input/input_day_8").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string.trim().to_owned()
}
