// Copyright 2018 Fredrik Portström <https://portstrom.com>
// Copyright (c) 2023 Olivier ROLAND
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

extern crate parse_mediawiki_dump_reboot;
use parse_mediawiki_dump_reboot::schema::Namespace;

const DUMP: &str = concat!(
    r#"<mediawiki xmlns="http://www.mediawiki.org/xml/export-0.10/">"#,
    "<page>",
    "<ns>0</ns>",
    "<title>alpha</title>",
    "<revision>",
    "<id>1</id>",
    "<format>beta</format>",
    "<model>gamma</model>",
    "<text>delta</text>",
    "</revision>",
    "</page>",
    "<page>",
    "<ns>4</ns>",
    "<title>epsilon</title>",
    "<revision>",
    "<id>2</id>",
    "<text>zeta</text>",
    "</revision>",
    "</page>",
    "</mediawiki>"
);

#[test]
fn main() {
    let mut parser =
        parse_mediawiki_dump_reboot::parse(std::io::BufReader::new(std::io::Cursor::new(DUMP)));
    assert!(match parser.next() {
        Some(Ok(parse_mediawiki_dump_reboot::Page {
            format: Some(format),
            model: Some(model),
            namespace: Namespace::Main,
            text,
            title,
            id,
        })) =>
            format == "beta" && model == "gamma" && text == "delta" && title == "alpha" && id == 1,
        _ => false,
    });
    assert!(match parser.next() {
        Some(Ok(parse_mediawiki_dump_reboot::Page {
            namespace: Namespace::Wikipedia,
            text,
            title,
            id,
            ..
        })) => text == "zeta" && title == "epsilon" && id == 2,
        _ => false,
    });
    assert!(parser.next().is_none());
}
