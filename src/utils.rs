use std::{collections::HashMap, io::Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn print_colored(text: String, color: Color) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout
        .set_color(ColorSpec::new().set_fg(Some(color)))
        .expect("unable to render text");
    writeln!(&mut stdout, "{}", text).expect("unable to render text");
    stdout.reset().unwrap();
}

pub fn get_language_aliases() -> HashMap<String, String> {
    let mut language_aliases = HashMap::new();

    language_aliases.insert("ts".into(), "node".into());
    language_aliases.insert("typescript".into(), "node".into());
    language_aliases.insert("js".into(), "node".into());
    language_aliases.insert("javascript".into(), "node".into());
    language_aliases.insert("node.js".into(), "node".into());
    language_aliases.insert("nodejs".into(), "node".into());
    language_aliases.insert("py".into(), "python".into());

    language_aliases
}
