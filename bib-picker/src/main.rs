use biblatex::Bibliography;
use itertools::Itertools;
use skim::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::io::Cursor;
use std::env;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let clean_contents = contents.replace("AUTHOR", "author").replace("TITLE", "title");
    // TODO: minuscule for authors, title
    let bibliography = Bibliography::parse(&clean_contents).unwrap();
    let values: Vec<_> = bibliography
        .iter()
        .filter(|e| e.author().is_ok())
        .map(|entry| {
            let author_field: String = entry
                .author()
                .unwrap()
                .iter()
                .map(|p| format!("{} {}", p.name, p.given_name))
                .intersperse(", ".to_string())
                .collect();
            let title_field: String = entry
                .title()
                .unwrap()
                .iter()
                .map(|spanned| spanned.as_ref())
                .map(|x| format!("{}", x.v.get()))
                .intersperse(String::from(""))
                .collect();
            (
                format!("{} ({})", title_field, author_field),
                entry.key.clone(),
            )
        })
        .collect();

    let myhm: HashMap<String, String> = HashMap::from_iter(values.clone().into_iter());
    let options = SkimOptionsBuilder::default()
        .height(Some("100%"))
        .multi(true)
        .build()
        .unwrap();

    let input = values.iter().map(|(k, _v)| k.clone()).join("\n");

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));

    let selected_items = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    for item in selected_items.iter() {
        let key: String = item.output().to_string();
        let value = myhm.get(&key).unwrap();
        print!("{}", value);
    }
}
