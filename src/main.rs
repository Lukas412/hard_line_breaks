use {
    nom::{
        bytes::complete::{tag, take_until},
        error::Error,
        multi::separated_list0,
        IResult,
    },
    std::fs::read_to_string,
};

fn main() {
    let input = read_to_string("testfile.md").unwrap();
    let result = parse_md_paragraphs(input.as_str()).unwrap();
    dbg!(result);
}

fn parse_md_paragraphs(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list0(tag("\n\n"), |input| Ok(parse_md_paragraph(input)))(input.trim())
}

fn parse_md_paragraph(input: &str) -> (&str, &str) {
    take_until::<_, _, Error<_>>("\n\n")(input).unwrap_or(("", input))
}
