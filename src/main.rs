
// TODO: Fix warnings
// TODO: Accept input from stdin like grep and shit (maybe acting like vim -)
// TODO: better input processing. See the windows x linux smutfic for more info.
// TODO: make the markov chain more effective through:
//       Better tokenization
//       using `n` word chains instead of fixing it to 1.

mod markov_chain;
use std::fs;
use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::stdin;
use std::process;

use markov_chain::MarkovChain;

fn main() {
    // rewrite this to use clap
    let mut args: Vec<String> = env::args().collect();
    println!("{}", args.len());
    args.remove(0);

    if args.len() == 0 {
        // print help and return.
        println!("This application requires arguments.");
        process::exit(0);
    }

    let content = if args.len() == 1 {
        let mut stream: Box<dyn Read> = if args[0] == "-" {
            // Read from stdin in this case.
            Box::new(io::stdin())
        } else {
            Box::new(File::open(&args[0]).unwrap())
            // get the stream of the file here.
        };

        // read the stream into a string here.
        let mut buf = String::new();
        stream.read_to_string(&mut buf).unwrap();
        buf
    } else {
        // use the args as input for the chain.
        args.join(" ")
    };


    let mut chain = MarkovChain::new(content);

    //let mut chain: MarkovChain = MarkovChain::new(r#"What is Lorem Ipsum? Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum. Why do we use it? It is a long established fact that a reader will be distracted by the readable content of a page when looking at its layout. The point of using Lorem Ipsum is that it has a more-or-less normal distribution of letters, as opposed to using 'Content here, content here', making it look like readable English. Many desktop publishing packages and web page editors now use Lorem Ipsum as their default model text, and a search for 'lorem ipsum' will uncover many web sites still in their infancy. Various versions have evolved over the years, sometimes by accident, sometimes on purpose (injected humour and the like)."#);
    chain.train();

    // only show the chain if its shorter than 10
    if chain.len() < 10 {
        println!("{chain}");
    }
    println!("Generated: \n{}", chain.generate(20));
}
