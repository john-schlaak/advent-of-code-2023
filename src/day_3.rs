enum SchematicTerm {
    Number { value: u32, start_pos: usize, length: usize },
    Symbol(usize)
}


struct SchematicLine {
    terms: Vec<SchematicTerm>
}

impl SchematicLine {
    fn is_range_part_number(&self, start_pos: usize, length: usize) -> bool {
        self.terms.iter().any(
            |term| if let &SchematicTerm::Symbol(pos) = term {
                pos >= if start_pos == 0 { start_pos } else { start_pos - 1 } && pos <= start_pos + length
            } else {
                false
            }
        )
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
                terms.push(SchematicTerm::Symbol(i))
            }
        }
    }
    if let Some(number) = current_number {
        terms.push(number);
    }
    SchematicLine { terms }
}