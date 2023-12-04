fn main() {
    println!("Part one example: {}", part_one("example"));
    println!("Part one input:   {}", part_one("input"));
}

#[derive(Debug)]
struct PartNo {
    digits: usize,
    row: usize,
    col: usize,
}

fn part_one(filename: &str) -> usize {
    let text = std::fs::read_to_string(filename).expect("Error reading file");

    let parts = extract_numbers(&text);

    let sum = parts
        .iter()
        .filter(|p| is_valid(p, &text))
        .map(|p| p.digits)
        .sum();

    return sum;
}

fn is_valid(part_no: &PartNo, text: &str) -> bool {
    let rows: Vec<_> = text.trim().lines().collect();

    let start_row = part_no.row.saturating_sub(1);
    let end_row = std::cmp::min(part_no.row + 1, rows.len() - 1);

    let rows = &rows[start_row..=end_row];
    let start_col = part_no.col.saturating_sub(1);
    let end_col = std::cmp::min(
        rows[0].len() - 1,
        part_no.col + part_no.digits.to_string().len(),
    );

    rows.iter()
        .find(|row| {
            row[start_col..=end_col]
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
