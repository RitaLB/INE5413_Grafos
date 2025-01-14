#![allow(non_snake_case)]
use std::env;
use graph_lib::*;
mod T2;
use crate::T2::*;
use crate::q1::q1;
use crate::q2::q2;
use crate::q3::q3;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let questao = &args[1];
    let filename = &args[2];
    let file_path = format!("src/grafos_teste/{}", filename);

    if questao =="1"{
        q1(file_path);
    } else if questao =="2"{
        q2(file_path);
    } else if questao =="3"{
        q3(file_path);
    } 
}
