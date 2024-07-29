#![allow(non_snake_case)]
use std::env;
use graph_lib::*;
mod T3;
use crate::T3::*;
use crate::q1::q1;
use crate::q2::q2;
use crate::q3::q3;



fn main() {
    
    // Coletando dados da linha de comando
    let args: Vec<String> = env::args().collect();
    let questao = &args[1];
    let filename = &args[2];
    let file_path = format!("src/grafos_teste/{}", filename);

    if questao =="1"{
        let s = args[3].parse::<i32>().unwrap();
        let t = args[4].parse::<i32>().unwrap();
       
        q1(file_path, s, t);
    }
    else if questao =="2"{
        q2(file_path);
    }
    else if questao =="3"{
        q3(file_path);
    }
    else{
        println!("Questão inválida");
    }
}
