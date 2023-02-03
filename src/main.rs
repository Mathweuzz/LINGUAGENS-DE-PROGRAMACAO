// Importação dos módulos necessários para trabalhar com arquivos no sistema de arquivos e manipulação de bytes
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use base64;

// Uma constante que armazena a chave secreta que será usada para cifrar e decifrar os dados
const SECRET_KEY: &str = "LINGUAGENSDEPROGRAMACAO";

// Uma função que realiza a cifragem dos dados usando a chave XOR
fn encrypt_xor(plaintext: &[u8], key: &[u8]) -> Vec<u8> {
    // Itera sobre os bytes do texto claro e da chave secreta
    // Realiza a operação XOR entre cada byte do texto claro e da chave secreta
    // Coleta os resultados em um vetor de bytes
    plaintext.iter().zip(key.iter().cycle()).map(|(a, b)| a ^ b).collect()
}

// Uma função que realiza a decifragem dos dados usando a chave XOR
fn decrypt_xor(ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
    // Reutiliza a função `encrypt_xor` para decifrar os dados, já que a cifragem e a decifragem são a mesma operação
    encrypt_xor(ciphertext, key)
}

fn main() {
    println!("1. Cadastrar usuário"); // Exibe a opção de cadastro
    println!("2. Autenticar usuário"); // Exibe a opção de autenticação
    println!("3. Deletar usuário"); // Exibe a opção de deleção
    println!("0. Sair"); // Exibe a opção de sair

    let mut opcao = String::new(); // Cria uma variável para armazenar a opção escolhida pelo usuário
    std::io::stdin().read_line(&mut opcao).unwrap(); // Lê a entrada do usuário e remove o caractere de nova linha

    match opcao.trim().parse::<i32>().unwrap() { // Converte a entrada do usuário para inteiro e verifica qual opção foi escolhida
        // Casos de opções escolhidas
        // Aqui é a opção 1 da aplicação, onde é realizado o cadastro do usuário
        1 => {
            // Pede para o usuário inserir o nome de usuário
            println!("Insira o nome de usuário:");
            let mut username = String::new();
            // Lê a entrada do usuário e armazena na variável "username"
            std::io::stdin().read_line(&mut username).unwrap();

            // Pede para o usuário inserir a senha
            println!("Insira a senha:");
            let mut password = String::new();
            // Lê a entrada do usuário e armazena na variável "password"
            std::io::stdin().read_line(&mut password).unwrap();

            // Criptografa a senha usando o algoritmo XOR com a chave "SECRET_KEY"
            let ciphertext = encrypt_xor(password.as_bytes(), SECRET_KEY.as_bytes());
            // Codifica o texto cifrado em base64
            let encoded = base64::encode(ciphertext);

            // Abre o arquivo "usuarios.csv" em modo de adição (append), se ele não existir será criado
            let mut file = OpenOptions::new().append(true).open("usuarios.csv").unwrap();
            // Escreve no arquivo "usuarios.csv" a linha com o nome de usuário e a senha codificada
            write!(file, "{},{}\n", username.trim(), encoded).unwrap();
        },
        2 => {
            // Pede o nome de usuário para autenticar
            println!("Insira o nome de usuário:");
            let mut username = String::new();
            // Lê o nome de usuário digitado pelo usuário
            std::io::stdin().read_line(&mut username).unwrap();

            // Pede a senha para autenticar
            println!("Insira a senha:");
            let mut password = String::new();
            // Lê a senha digitada pelo usuário
            std::io::stdin().read_line(&mut password).unwrap();

            // Abre o arquivo de usuários registrados
            let mut file = File::open("usuarios.csv").unwrap();
            // Lê o conteúdo do arquivo para a memória
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            // Percorre cada linha do arquivo (cada linha representa um usuário)
            for line in contents.lines() {
                // Divide a linha pelo caractere "," para obter o nome de usuário e a senha cifrada
                let parts: Vec<&str> = line.split(",").collect();
                let user = parts[0];
                let encoded_password = parts[1];
                // Decodifica a senha cifrada do base64
                let ciphertext = base64::decode(encoded_password).unwrap();
                // Decifra a senha
                let decrypted = decrypt_xor(&ciphertext, SECRET_KEY.as_bytes());

                // Verifica se o nome de usuário e a senha são os mesmos digitados pelo usuário
                if username.trim() == user && password.as_bytes() == &decrypted[..] {
                    // Autenticação bem-sucedida
                    println!("Autenticação bem-sucedida");
                    // Sai da função main()
                    return;
                }
            }
            println!("Autenticação falhou");
        },
        3 => {
            // Pede o nome de usuário ao usuário
            println!("Insira o nome de usuário:");
            let mut username = String::new();
            // Lê a entrada do usuário para a variável username
            std::io::stdin().read_line(&mut username).unwrap();

            // Pede a senha ao usuário
            println!("Insira a senha:");
            let mut password = String::new();
            // Lê a entrada do usuário para a variável password
            std::io::stdin().read_line(&mut password).unwrap();

            // Abre o arquivo "usuarios.csv"
            let mut file = File::open("usuarios.csv").unwrap();
            let mut contents = String::new();
            // Lê o conteúdo do arquivo para a variável contents
            file.read_to_string(&mut contents).unwrap();

            // Divide o conteúdo do arquivo em linhas e as armazena em lines
            let mut lines = contents.lines().collect::<Vec<&str>>();
            let mut found = false;
            // Loop para percorrer as linhas do arquivo
            for i in 0..lines.len() {
                // Divide cada linha em partes separadas por "," e as armazena em parts
                let parts: Vec<&str> = lines[i].split(",").collect();
                let user = parts[0];
                let encoded_password = parts[1];
                // Decodifica a senha criptografada
                let ciphertext = base64::decode(encoded_password).unwrap();
                let decrypted = decrypt_xor(&ciphertext, SECRET_KEY.as_bytes());

                // Verifica se o nome de usuário e a senha combinam com um registro no arquivo
                if username.trim() == user && password.as_bytes() == &decrypted[..] {
                    found = true;
                    // Remove a linha correspondente ao registro encontrado
                    lines.remove(i);
                    break;
                }
            }
            if found {
                // Junta as linhas restantes em uma única string separada por quebra de linha
                let new_contents = lines.join("\n");
                // Cria um novo arquivo "usuarios.csv"
                let mut file = File::create("usuarios.csv").unwrap();
                // Escreve o novo conteúdo no arquivo
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

