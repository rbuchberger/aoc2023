use std::collections::HashMap;

fn main() {
    println!("Part one example: {}", part_one("example"));
    println!("Part one input:   {}", part_one("input"));

    println!("Part two example: {}", part_two("example"));
    println!("Part two example: {}", part_two("input"));
}

fn part_one(filename: &str) -> usize {
    let text = std::fs::read_to_string(filename).unwrap();
    let parts = extract_numbers(&text);

    return parts
        .iter()
        .filter(|p| is_valid(p, &text))
        .map(|p| p.digits)
        .sum();
}

fn part_two(filename: &str) -> usize {
    let text = std::fs::read_to_string(filename).unwrap();
    let parts = extract_numbers(&text);

    let gears = extract_gears(&parts, &text);

    let sum = gears
        .iter()
        .filter(|(_, g)| g.len() == 2)
        .map(|(_, g)| g.iter().fold(1, |prod, el| prod * el))
        .sum();

    return sum;
}

#[derive(Debug)]
struct PartNo {
    digits: usize,
    row: usize,
    col: usize,
}

struct Bounds {
    start_row: usize,
    end_row: usize,
    start_col: usize,
    end_col: usize,
}

fn get_bounds(part_no: &PartNo, rows: &Vec<&str>) -> Bounds {
    let start_row = part_no.row.saturating_sub(1);
    let end_row = std::cmp::min(part_no.row + 1, rows.len() - 1);

    let start_col = part_no.col.saturating_sub(1);
    let end_col = std::cmp::min(
        rows[0].len() - 1,
        part_no.col + part_no.digits.to_string().len(),
    );

    return Bounds {
        start_row,
        end_row,
        start_col,
        end_col,
    };
}

fn is_valid(part_no: &PartNo, text: &str) -> bool {
    let rows: Vec<_> = text.trim().lines().collect();

    let bounds = get_bounds(part_no, &rows);

    let rows = &rows[bounds.start_row..=bounds.end_row];

    rows.iter()
        .find(|row| {
            row[bounds.start_col..=bounds.end_col]
                .chars()
                .find(|c| !c.is_digit(10) && c != &'.')
                .is_some()
        })
        .is_some()
}

fn extract_numbers(text: &str) -> Vec<PartNo> {
    let mut current_digits: Option<(usize, String)> = None;
    let mut parts: Vec<PartNo> = Vec::new();

    text.trim().lines().enumerate().for_each(|(line_no, line)| {
        line.chars().enumerate().for_each(|(col_no, c)| {
            match (c.is_digit(10), current_digits.clone()) {
                (true, None) => current_digits = Some((col_no, c.to_string())),
                (true, Some((i, mut existing))) => {
                    existing.push(c);
                    current_digits = Some((i, existing));
                }
                (false, Some((i, existing))) => {
                    parts.push(PartNo {
                        digits: existing.parse().unwrap(),
                        row: line_no,
                        col: i,
                    });
                    current_digits = None;
                }
                (false, None) => (),
            }
        });

        // End of line
        if let Some((i, existing)) = current_digits.clone() {
            parts.push(PartNo {
                digits: existing.parse().unwrap(),
                row: line_no,
                col: i,
            });

            current_digits = None;
        }
    });

    return parts;
}

// HashMap keys are X,Y positions of gears (star characters), values are a vector of part numbers
// touching.
fn extract_gears(parts: &Vec<PartNo>, text: &str) -> HashMap<(usize, usize), Vec<usize>> {
    let mut gears: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    let rows: Vec<_> = text.trim().lines().collect();

    parts.iter().for_each(|part_no| {
        let bounds = get_bounds(part_no, &rows);

        rows[bounds.start_row..=bounds.end_row]
            .iter()
            .enumerate()
            .for_each(|(row_offset, row)| {
                row[bounds.start_col..=bounds.end_col]
                    .chars()
                    .enumerate()
                    .filter(|(_, c)| c == &'*')
                    .for_each(|(col_offset, _)| {
                        gears
                            .entry((row_offset + bounds.start_row, col_offset + bounds.start_col))
                            .and_modify(|e| e.push(part_no.digits))
                            .or_insert(vec![part_no.digits]);
                    });
            });
    });

    return gears;
}
