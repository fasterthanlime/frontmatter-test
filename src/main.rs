use pulldown_cmark::{Event, Options};

fn main() {
    let input = include_str!("sample.md");

    let parser = pulldown_cmark::Parser::new_ext(input, Options::all());

    for ev in parser {
        match ev {
            Event::Start(t) => {
                println!("Start({t:?})");
            }
            Event::End(t) => {
                println!("End({t:?})");
            }
            Event::InlineMath(s) => {
                println!("InlineMath(s)")
            }
            Event::DisplayMath(s) => {
                println!("DisplayMath(s)")
            }
            Event::Text(t) => {
                println!("{t}");
            }
            _ => {}
        }
    }

    let parser = pulldown_cmark::Parser::new(input);
    let mut s = String::new();
    pulldown_cmark::html::push_html(&mut s, parser);
    println!("HTML:\n\n {s}");

    use std::fs::File;
    use std::io::Write;
    use std::process::Command;

    let mut file = File::create("/tmp/index.html").expect("Unable to create file");
    file.write_all(s.as_bytes()).expect("Unable to write data");

    Command::new("open")
        .arg("/tmp/index.html")
        .output()
        .expect("Failed to open file");
}
