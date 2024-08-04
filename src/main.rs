use ferris_says::say;
use serenity::utils::token::validate;
use std::env;
use std::io::{stdout, BufWriter};

const API_KEY: &str = "API_KEY";

fn main() {
    print_success_run();
    validate_key();
}

fn print_success_run() {
    let stdout = stdout();
    let message = String::from("cumm-bot está rodando!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}

fn validate_key() {
    let token = get_api_token();

    if let Some(ref token) = token {
        match validate(token) {
            Ok(_val) => {
                println!("Chave API validada com sucesso!");
            }
            Err(_e) => println!("Erro ao validar chave API"),
        }
    }
}

fn get_api_token() -> Option<String> {
    match env::var(API_KEY) {
        Ok(token) => Some(token),
        Err(_e) => {
            println!("Erro ao pegar a chave API!");
            println!("Passe como uma variável de ambiente");
            println!("Exemplo:");
            println!(r##"$ API_KEY="dnjsah821u38hundks987g3bf" cargo run"##);
            None
        }
    }
}
