use std::fs::File;
use std::io::{BufReader,BufRead};

extern crate regex;
use regex::Regex;

extern crate clap;
use clap::{App,Arg,SubCommand,AppSettings};

struct Chunk {
    name: Option<String>,
    contents: Vec<String>
}

impl Chunk {
    fn new_code_chunk(name: &str) -> Chunk {
        let name = String::from(name);
        Chunk {
            name: Some(name),
            contents: Vec::new()
        }
    }

    fn new_doc_chunk() -> Chunk {
        Chunk {
            name: None,
            contents: Vec::new()
        }
    }
}

/*
fn find<'a>(chunks: &'a Vec<Chunk>, name: &'a String) -> Option<&'a Chunk> {
    for chunk in chunks {
        if chunk.name.is_some() &&
           chunk.name.clone().unwrap() == *name {
            return Some(chunk)
        }
    }
    None
}*/

/*fn find(chunks: &Vec<Chunk>, name: &String) -> Option<Chunk> {
    for chunk in &chunks {
        match chunk.name {
            Some(ref chunk_name) => if chunk_name == name {
                return Some(*chunk.clone())
            },
            None => ()
        }
        /*if chunk.name.is_some() &&
           chunk.name.unwrap() == *name {
            return Some(chunk)
        }*/
    }
    None
}*/

fn tangle(chunks: &Vec<Chunk>, name: String, indent: String) {
    let embedded = Regex::new(r"^(\s*)<<([^>]+)>>\s*$").unwrap();

    /*match find(&chunks, &name) {*/
    match chunks.iter().find(|&c| c.name == Some(name.clone())) {
        Some(chunk) => for line in &chunk.contents {
            if embedded.is_match(&line) {
                let captures   = embedded.captures(&line).unwrap();
                let leading    = captures[1].to_string();
                let chunk_name = captures[2].to_string();

                let new_indent = format!("{}{}", &indent, &leading);

                tangle(&chunks, chunk_name, new_indent);
            } else {
                println!("{}{}", &indent, &line);
            }
        },
        None => {
            println!("chunk {} not found", &name);
        }
    }
}

fn weave(chunks: &Vec<Chunk>) {
    for chunk in chunks {
        match chunk.name.clone() {
            Some(name) => {
                println!("\n<<{}>>=", name);
                println!("```");
                for line in &chunk.contents {
                    println!("{}", line);
                }
                println!("```\n");
            },
            None => for line in &chunk.contents {
                println!("{}", line);
            }
        }
    }
}

fn parse(filename: String) -> Vec<Chunk> {
    let code_chunk = Regex::new(r"^<<([^>]+)>>=$").unwrap();
    let documentation_chunk = Regex::new(r"^@$").unwrap();
    let escaped_chunk = Regex::new(r"^@.+$").unwrap();

    let mut chunks: Vec<Chunk> = Vec::new();

    let file = File::open(filename).unwrap();

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();

        if code_chunk.is_match(&line) {
            // Create a new named (code) chunk
            let captures   = code_chunk.captures(&line).unwrap();
            let chunk_name = captures.get(1).unwrap().as_str();

            chunks.push(Chunk::new_code_chunk(&chunk_name));
        } else if documentation_chunk.is_match(&line) {
            // Create a new unnamed (documentation) chunk
            chunks.push(Chunk::new_doc_chunk());
        } else {
            // Appends line to the last chunk (and if no chunk exists
            // yet, creates a new documentation chunk to append to)
            if chunks.len() == 0 {
                chunks.push(Chunk::new_doc_chunk());
            }

            // The chunk might have been escaped (prefixed with `@`) -
            // this allows one to write `@` by itself inside a code or
            // documentation chunk.
            let line = if escaped_chunk.is_match(&line) {
                escaped_chunk.captures(&line).unwrap()[1].to_string()
            } else {
                String::from(line)
            };

            // Finally, append.
            chunks.last_mut().unwrap().contents.push(line);
        }
    }
    chunks
}

fn main() {
    let app = App::new("noweb-rs")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version("0.1.1")
        .author("mftrhu")
        .about("Parses, tangles and weaves literate programming sources in the noWeb format.")
        .subcommand(SubCommand::with_name("tangle")
            .about("Extracts a code chunk from the given noweb source")
            .arg(Arg::with_name("INPUT")
                .help("The input file to use")
                .required(true)
                .index(1))
            .arg(Arg::with_name("CHUNK")
                .help("The name of the chunk to tangle out")
                .required(true)
                .index(2)))
        .subcommand(SubCommand::with_name("weave")
            .about("Weaves together documentation and code")
            .arg(Arg::with_name("INPUT")
                .help("The input file to use")
                .required(true)
                .index(1)));

    let matches = app.get_matches();

    match matches.subcommand() {
        ("tangle", Some(tangle_matches)) => {
            let infile = tangle_matches.value_of("INPUT").unwrap();
            let infile = String::from(infile);
            let chunk = tangle_matches.value_of("CHUNK").unwrap();
            let chunk = String::from(chunk);
            let chunks = parse(infile);
            tangle(&chunks, chunk, String::from(""));
        },
        ("weave", Some(weave_matches))=> {
            let infile = weave_matches.value_of("INPUT").unwrap();
            let infile = String::from(infile);
            let chunks = parse(infile);

            weave(&chunks);
        },
        ("", None) => (),
        _ => unreachable!()
    }
}
