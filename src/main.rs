use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use base64;

const SECRET_KEY: &str = "LINGUAGENSDEPROGRAMACAO";

fn encrypt_xor(plaintext: &[u8], key: &[u8]) -> Vec<u8> {
    plaintext.iter().zip(key.iter().cycle()).map(|(a, b)| a ^ b).collect()
}

fn decrypt_xor(ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
    encrypt_xor(ciphertext, key)
}

fn main() {
    println!("1. Cadastrar usuário");
    println!("2. Autenticar usuário");
    println!("3. Deletar usuário");
    println!("0. Sair");

    let mut opcao = String::new();
    std::io::stdin().read_line(&mut opcao).unwrap();

    match opcao.trim().parse::<i32>().unwrap() {
        1 => {
            println!("Insira o nome de usuário:");
            let mut username = String::new();
            std::io::stdin().read_line(&mut username).unwrap();

            println!("Insira a senha:");
            let mut password = String::new();
            std::io::stdin().read_line(&mut password).unwrap();

            let ciphertext = encrypt_xor(password.as_bytes(), SECRET_KEY.as_bytes());
            let encoded = base64::encode(ciphertext);

            let mut file = OpenOptions::new().append(true).open("usuarios.csv").unwrap();
            write!(file, "{},{}\n", username.trim(), encoded).unwrap();
        },
        2 => {
            println!("Insira o nome de usuário:");
            let mut username = String::new();
            std::io::stdin().read_line(&mut username).unwrap();

            println!("Insira a senha:");
            let mut password = String::new();
            std::io::stdin().read_line(&mut password).unwrap();

            let mut file = File::open("usuarios.csv").unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            for line in contents.lines() {
                let parts: Vec<&str> = line.split(",").collect();
                let user = parts[0];
                let encoded_password = parts[1];
                let ciphertext = base64::decode(encoded_password).unwrap();
                let decrypted = decrypt_xor(&ciphertext, SECRET_KEY.as_bytes());

                if username.trim() == user && password.as_bytes() == &decrypted[..] {
                    println!("Autenticação bem-sucedida");
                    return;
                }
            }
            println!("Autenticação falhou");
        },
        3 => {
            println!("Insira o nome de usuário:");
            let mut username = String::new();
            std::io::stdin().read_line(&mut username).unwrap();

            println!("Insira a senha:");
            let mut password = String::new();
            std::io::stdin().read_line(&mut password).unwrap();

            let mut file = File::open("usuarios.csv").unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            let mut lines = contents.lines().collect::<Vec<&str>>();
            let mut found = false;
            for i in 0..lines.len() {
                let parts: Vec<&str> = lines[i].split(",").collect();
                let user = parts[0];
                let encoded_password = parts[1];
                let ciphertext = base64::decode(encoded_password).unwrap();
                let decrypted = decrypt_xor(&ciphertext, SECRET_KEY.as_bytes());

                if username.trim() == user && password.as_bytes() == &decrypted[..] {
                    found = true;
                    lines.remove(i);
                    break;
                }
            }
            if found {
                let new_contents = lines.join("\n");
                let mut file = File::create("usuarios.csv").unwrap();
                file.write_all(new_contents.as_bytes()).unwrap();
                println!("Usuário deletado com sucesso");
            } else {
                println!("Autenticação falhou");
            }
        },
        0 => {
            println!("Saindo...");
            return;
        },
        _=> println!("Opção inválida")
        }
}

