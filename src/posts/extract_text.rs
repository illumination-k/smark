use pulldown_cmark::{Event, Options, Parser};

pub fn extract_text(markdown_text: &str) -> String {
    let mut s = vec![];
    let parser = Parser::new_ext(markdown_text, Options::empty());

    for e in parser {
        match e {
            Event::Text(text) => s.push(text.to_string()),
            _ => continue,
        }
    }

    s.join("\n")
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::io::read_string;
    use crate::posts::frontmatter::split_frontmatter_and_content;

    #[test]
    fn test_extract_text() {
        let markdown_text = read_string(&"test/posts/ja/c1/test_post.md".to_string()).unwrap();
        let (_, markdown_text) = split_frontmatter_and_content(&markdown_text);

        let text = extract_text(&markdown_text);
        assert_eq!(
            &text,
            r#"TEST
これはテストです。
リスト1
リスト2
self
fn main() {
    println!("Hello World")
}

Some Codes"#
        )
    }
}