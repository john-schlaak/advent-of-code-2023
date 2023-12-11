use std::collections::{HashMap, HashSet};

enum Direction {
    Left,
    Right
}


struct Node {
    left: String,
    right: String
}


struct Map {
    directions: Vec<Direction>,
    nodes: HashMap<String, Node>
}

impl Map {
    fn count_moves(&self) -> usize {
        let mut move_count = 0;
        let mut directions_iter = self.directions.iter().cycle();
        let mut current_node = self.nodes.get(&String::from("AAA")).unwrap();
        loop {
            let next_node_name = match directions_iter.next().unwrap() {
                Direction::Left => &current_node.left,
                Direction::Right => &current_node.right
            };
            move_count += 1;
            if next_node_name.eq(&String::from("ZZZ")) {
                break;
            }
            current_node = self.nodes.get(next_node_name).unwrap();
        }
        return move_count;
    }

    fn get_cycle_info(&self, start: &String) -> (usize, usize, usize) {
        let cycle_start;
        let cycle_length;
        let mut move_count = 0;
        let mut z_pos = 0;    // in practice, it seems there's only 1 Z for each A
        let mut first_visits: HashMap<(&String, usize), usize> = HashMap::new();
        let mut directions_iter = self.directions.iter().enumerate().cycle();
        let mut current_node = self.nodes.get(start).unwrap();
        let mut node_name = start;
        loop {
            let (index, direction) = directions_iter.next().unwrap();
            if let Some(&first_visit) = first_visits.get(&(node_name, index)) {
                cycle_start = first_visit;
                cycle_length = move_count - cycle_start;
                break;
            }
            if node_name.ends_with('Z') {
                z_pos = move_count;
            }
            first_visits.insert((node_name, index), move_count);
            node_name = match direction {
                Direction::Left => &current_node.left,
                Direction::Right => &current_node.right
            };
            current_node = self.nodes.get(node_name).unwrap();
            move_count += 1;
        }
        return (cycle_start, cycle_length, z_pos)
    }

    fn calculate_moves_from_any_a(&self) -> usize {
        let mut cycle_infos = self.nodes.iter().filter_map(
            |(name, _)| if name.ends_with('A') {
                Some(name)
            } else {
                None
            }
        ).map(|name| self.get_cycle_info(name)).collect::<Vec<(usize, usize, usize)>>();
        cycle_infos.sort_by(|(_, cycle_length_a, _), (_, cycle_length_b, _)| cycle_length_b.cmp(cycle_length_a));
        let (_, _, final_z_pos) = cycle_infos.into_iter().reduce(
            /* cycle_length_a >= cycle_length_b */
            |(cycle_start_a, cycle_length_a, abs_z_pos_a), (cycle_start_b, cycle_length_b, abs_z_pos_b)| {
                let rel_z_pos_a_on_a = abs_z_pos_a - cycle_start_a;
                let rel_z_pos_b_on_a = if abs_z_pos_b >= cycle_start_a {
                    (abs_z_pos_b - cycle_start_a) % cycle_length_a
                } else {
                    cycle_length_a - (cycle_start_a - abs_z_pos_b) % cycle_length_a
                };
                let rel_z_pos_b_on_b = abs_z_pos_b - cycle_start_b;
                let rel_z_pos_a_on_b = if abs_z_pos_a >= cycle_start_b {
                    (abs_z_pos_a - cycle_start_b) % cycle_length_b
                } else {
                    cycle_length_b - (cycle_start_b - abs_z_pos_a) % cycle_length_b
                };
                let normalized_z_pos_a_on_b = if rel_z_pos_a_on_b >= rel_z_pos_b_on_b {
                    rel_z_pos_a_on_b - rel_z_pos_b_on_b
                } else {
                    cycle_length_b - (rel_z_pos_b_on_b - rel_z_pos_a_on_b) % cycle_length_b
                };
                let normalized_z_pos_b_on_a = if rel_z_pos_b_on_a >= rel_z_pos_a_on_a {
                    rel_z_pos_b_on_a - rel_z_pos_a_on_a
                } else {
                    cycle_length_a - (rel_z_pos_a_on_a - rel_z_pos_b_on_a) % cycle_length_a
                };
                let mut times_to_nullify_offset = 0;
                loop {
                    if (normalized_z_pos_b_on_a + cycle_length_b * times_to_nullify_offset) % cycle_length_a == 0 {
                        break;
                    } else {
                        times_to_nullify_offset += 1;
                    }
                }
                let additional_moves_offset = times_to_nullify_offset * cycle_length_b;
                let mut times_to_loop = 1;
                loop {
                    if (cycle_length_b * times_to_loop) % cycle_length_a == 0 {
                        break;
                    } else {
                        times_to_loop += 1;
                    }
                }
                (cycle_start_b + additional_moves_offset, cycle_length_b * times_to_loop, abs_z_pos_b + additional_moves_offset)
            }
        ).unwrap();
        final_z_pos
    }
}


pub fn count_moves_for_map(map_text: String) -> usize {
    parse_map(map_text).count_moves()
}


pub fn count_moves_from_any_a_for_map(map_text: String) -> usize {
    parse_map(map_text).calculate_moves_from_any_a()
}


fn parse_map(map_text: String) -> Map {
    let mut lines = map_text.split('\n').map(|line_text| line_text.trim());
    if let (Some(directions_line), _, lines) = (lines.next(), lines.next(), lines) {
        Map {
            directions: directions_line.chars().filter_map(
                |c| match c {
                    'L' => Some(Direction::Left),
                    'R' => Some(Direction::Right),
                    _ => None
                }
            ).collect(),
            nodes: HashMap::from_iter(
                lines.filter_map(
                    |line_text| if let Some([node_title, left_right]) = line_text.split('=').map(|piece| piece.trim()).collect::<Vec<&str>>().get(0..2) {
                        if let Some([left, right]) = left_right.split(',').map(|piece| piece.trim()).collect::<Vec<&str>>().get(0..2) {
                            if let (Some(left), Some(right)) = (left.strip_prefix('('), right.strip_suffix(')')) {
                                Some((
                                    String::from(node_title.clone()),
                                    Node {
                                        left: String::from(left),
                                        right: String::from(right)
                                    }
                                ))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                )
            )
        }
    } else {
        panic!("Could not parse map")
    }
}