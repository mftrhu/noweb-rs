# noweb-rs

An implementation of the **[noweb][]** literate programming tool in the
Rust language.

**noweb** was first created in 1989-1999 by Norman Ramsey, to be simple
and language-agnostic: the other available tools, WEB and CWEB, were
neither.  WEB, especially, only supported Pascal and Pascal-like
languages, leading to the creation of CWEB for C-like ones.

[noweb]: <http://www.cs.tufts.edu/~nr/noweb/>

## noweb syntax

A **noweb** source file contains both source code and documentation
chunks, interleaved.  Each chunk is terminated by the beginning of
another chunk.

Code chunks are named, and start with

    <<Chunk name>>=

They in turn can contain references to other code chunks, whose leading
space is preserved when tangling:

    <<Chunk name>>=
    def hello():
        <<Hello body>>

Documentation chunks are unnamed, and start with

    @

## Tangling and weaving

## The program

<<main.rs>>=
<<External crates>>
<<Use declarations>>

<<Tangling>>

<<Weaving>>

<<Parsing>>

fn main() {
    <<Parse command-line arguments>>
    <<Dispatch subcommands>>
}
@

## Command-line arguments parsing

**noweb-rs** will use the [clap][] library for parsing of the command
line arguments.

[clap]:

<<Parse command-line arguments>>=
let app = clap::App::new("noweb-rs")
    .version("0.1.0")
    .author("mftrhu")
    .help("");
@

<<Dispatch subcommands>>=
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
@
