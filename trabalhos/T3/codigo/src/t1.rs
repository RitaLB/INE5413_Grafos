use std::env;
use graph_lib::*;
mod T1;
use crate::T1::*;
use crate ::T1::q1::*;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let questao = &args[1];
    let filename = &args[2];
    let file_path = format!("src/pla_examples/{}", filename);

    if questao =="1"{
        println!("q1");
    } else if questao =="2"{
        println!("q2");
    }
}
