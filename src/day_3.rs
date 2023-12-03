enum SchematicTerm {
    Number { value: u32, start_pos: usize, length: usize },
    Symbol { value: char, pos: usize }
}


struct SchematicLine {
    terms: Vec<SchematicTerm>
}

impl SchematicLine {
    fn is_range_part_number(&self, start_pos: usize, length: usize) -> bool {
        self.terms.iter().any(
            |term| if let &SchematicTerm::Symbol { value: _, pos } = term {
                is_pos_adjacent_to_range(pos, start_pos, length)
            } else {
                false
            }
        )
    }

    fn get_numbers_bordering_symbol(&self, pos: usize) -> Vec<u32> {
        self.terms.iter().filter_map(
            |term| if let &SchematicTerm::Number { value, start_pos, length} = term {
                if is_pos_adjacent_to_range(pos, start_pos, length) {
                    Some(value)
                } else {
                    None
                }
            } else {
                None
            }
        ).collect()
    }
}


pub fn sum_part_numbers(schematic: String) -> u32 {
    let schematic_lines: Vec<SchematicLine> = schematic.split('\n').map(|line_text| parse_schematic_line(line_text)).collect();
    schematic_lines.iter().enumerate().map(
        |(i, line)| {
            let (last, next) = (
                if i > 0 { schematic_lines.get(i - 1) } else { None },
                schematic_lines.get(i + 1)
            );
            line.terms.iter().map(
                |term| if let &SchematicTerm::Number { value, start_pos, length} = term {
                    if [last, Some(line), next].iter().any(
                        |line| if let Some(line) = line {
                            line.is_range_part_number(start_pos, length)
                        } else {
                            false
                        }
                    ) {
                        value
                    } else {
                        0
                    }
                } else {
                    0
                }
            ).sum::<u32>()
        }
    ).sum()
}


pub fn sum_gear_ratios(schematic: String) -> u32 {
    let schematic_lines: Vec<SchematicLine> = schematic.split('\n').map(|line_text| parse_schematic_line(line_text)).collect();
    schematic_lines.iter().enumerate().map(
        |(i, line)| {
            let (last, next) = (
                if i > 0 { schematic_lines.get(i - 1) } else { None },
                schematic_lines.get(i + 1)
            );
            line.terms.iter().map(
                |term| if let &SchematicTerm::Symbol { value: '*', pos} = term {
                    let neighboring_numbers: Vec<u32> = [last, Some(line), next].iter().flat_map(
                        |line| if let Some(line) = line {
                            line.get_numbers_bordering_symbol(pos)
                        } else {
                            Vec::new()
                        }
                    ).collect();
                    if neighboring_numbers.len() == 2 {
                        neighboring_numbers.get(0).unwrap() * neighboring_numbers.get(1).unwrap()
                    } else {
                        0
                    }
                } else {
                    0
                }
            ).sum::<u32>()
        }
    ).sum()
}


fn is_pos_adjacent_to_range(pos: usize, start_pos: usize, length: usize) -> bool {
    pos >= if start_pos == 0 { start_pos } else { start_pos - 1 } && pos <= start_pos + length
}


fn parse_schematic_line(line_text: &str) -> SchematicLine {
    let mut terms = Vec::new();
    let mut current_number: Option<SchematicTerm> = None;
    for (i, c) in line_text.trim().char_indices() {
        if c.is_digit(10) {
            let digit = c.to_digit(10).unwrap();
            current_number = Some(
                if let Some(SchematicTerm::Number { value, start_pos, length }) = current_number {
                    SchematicTerm::Number {
                        value: value * 10 + digit,
                        start_pos: start_pos,
                        length: length + 1
                    }
                } else {
                    SchematicTerm::Number {
                        value: digit,
                        start_pos: i,
                        length: 1
                    }
                }
            )
        } else {
            if let Some(number) = current_number {
                terms.push(number);
            }
            current_number = None;
            if c != '.' {
                terms.push(SchematicTerm::Symbol { value: c, pos: i })
            }
        }
    }
    if let Some(number) = current_number {
        terms.push(number);
    }
    SchematicLine { terms }
}