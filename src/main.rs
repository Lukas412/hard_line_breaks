use {
    nom::{
        bytes::complete::take_until, character::complete::newline, error::Error,
        multi::separated_list0, sequence::tuple, IResult,
    },
    std::fs::read_to_string,
};

fn main() {
    let input = read_to_string("testfile.md").unwrap();
    let result = parse_md_paragraphs(input.as_str()).unwrap();
    dbg!(result);
}

fn parse_md_paragraphs(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list0(tuple((newline, newline)), |input| {
        Ok(parse_md_paragraph(input))
    })(input.trim())
}

fn parse_md_paragraph(input: &str) -> (&str, &str) {
    take_until::<&str, &str, Error<&str>>("\n\n")(input).unwrap_or(("", input))
}
