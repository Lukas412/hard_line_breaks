use {
    nom::{
        character::complete::{newline, not_line_ending},
        multi::separated_list0,
        IResult,
    },
    std::fs::read_to_string,
};

fn main() {
    let input = read_to_string("testfile.md").unwrap();
    let (_, result) = hard_break_paragraphs(input.as_str()).unwrap();
    dbg!(result);
}

fn hard_break_paragraphs(input: &str) -> IResult<&str, Vec<String>> {
    let (input, lines) = separated_list0(newline, not_line_ending)(input.trim())?;

    let output = lines.iter().fold(vec![String::from("")], |mut acc, item| {
        if item.is_empty() {
            acc.push("".to_string());
        } else if acc.last().is_some_and(|s| s.is_empty()) {
            acc.last_mut().unwrap().push_str(item);
        } else {
            acc.last_mut().unwrap().push_str(" ");
            acc.last_mut().unwrap().push_str(item);
        }
        acc
    });
    Ok((input, output))
}
