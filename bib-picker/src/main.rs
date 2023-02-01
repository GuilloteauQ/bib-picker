use biblatex::Bibliography;
use std::collections::HashMap;
use biblatex::Chunk;
use itertools::Itertools;
use std::env;
use std::fs;


fn main() {
    let file_path = "refs.bib";
    
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    // TODO: minuscule for authors, title    
    let bibliography = Bibliography::parse(&contents).unwrap();
    bibliography.iter().for_each(|entry| {
        let author_field: String = entry.author().unwrap().iter().map(|p| format!("{} {}", p.name, p.given_name)).intersperse(", ".to_string()).collect();
        let title_field: String = entry.title().unwrap().iter().map(|spanned| spanned.as_ref()).map(|x| format!("{}", x.v.get())).intersperse(String::from("")).collect();
        println!("{}: {} ({})", entry.key, title_field, author_field);        
    });
    let values: Vec<_> = bibliography.iter().map(|entry| {
        let author_field: String = entry.author().unwrap().iter().map(|p| format!("{} {}", p.name, p.given_name)).intersperse(", ".to_string()).collect();
        let title_field: String = entry.title().unwrap().iter().map(|spanned| spanned.as_ref()).map(|x| format!("{}", x.v.get())).intersperse(String::from("")).collect();
        (entry.key.clone(), format!("{} ({})", title_field, author_field))
    }).collect();

    println!("{:#?}", values);
    let myhm: HashMap<String, String> = HashMap::from_iter(values.into_iter());
    println!("{:#?}", myhm);

}
