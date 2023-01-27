use std::io;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("Selecione uma opção:");
    println!("1. Adicionar novo usuário e senha");
    println!("2. Mostrar senhas de usuário existente");
    let mut option = String::new();
    io::stdin().read_line(&mut option).unwrap();
    let option: u32 = option.trim().parse().unwrap();

    match option {
        1 => {
            println!("Insira o nome de usuário:");
            let mut username = String::new();
            io::stdin().read_line(&mut username).unwrap();
            println!("Insira a senha:");
            let mut password = String::new();
            io::stdin().read_line(&mut password).unwrap();
            let mut file = File::create("usuarios.csv").unwrap();
            write!(file, "{},{}\n", username.trim(), password.trim()).unwrap();
        },
        2 => {
            println!("Insira o nome de usuário:");
            let mut username = String::new();
            io::stdin().read_line(&mut username).unwrap();
            let mut file = File::open("usuarios.csv").unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            let search_string = format!("{}", username.trim());
            if contents.contains(&search_string) {
                let start_index = contents.find(&search_string).unwrap();
                let end_index = contents[start_index..].find(",").unwrap();
                let start_index_password = start_index + end_index + 1;
                let end_index_password = contents[start_index_password..].find("\n").unwrap();
                println!("Senhas: {}", &contents[start_index_password..start_index_password + end_index_password]);
            } else {
                println!("Usuário não encontrado");
            }
        },
        _ => println!("Opção inválida"),
    }
}
