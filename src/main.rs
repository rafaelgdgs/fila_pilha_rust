use std::io::Write;

enum Operations {
    Add(i32),
    Pop,
    Show,
    Quit,
}

fn extrair_value_fn_valida_entrada(input: &str) -> Result<i32, String> {
    match input.parse::<i32>() {
        Ok(value) => Ok(value),
        Err(_) => Err("Valor inválido. Por favor, insira um número.".to_owned()),
    }
}

fn valida_entrada(input: &String) -> Result<Operations, String> {
    let input_into_slice: &str = &input[..];
    let parts: Vec<&str> = input_into_slice.split(':').collect();

    if parts.len() != 2 {
        return Err("Entrada inválida. Use o formato: <operação>: <valor>".to_owned());
    }

    let operation = parts[0].trim();
    let value_str = parts[1].trim();

    match operation {
        "Add" => match extrair_value_fn_valida_entrada(&value_str) {
            Ok(n) => return Ok(Operations::Add(n)),
            Err(msg) => return Err(msg),
        },
        "Pop" => return Ok(Operations::Pop),
        "Show" => return Ok(Operations::Show),
        "Quit" => return Ok(Operations::Quit),
        _ => return Err("Operação invalida!".to_owned()),
    }
}

fn get_action() -> Option<Operations> {
    let mut user_input_string: String = String::new();
    let _ = std::io::stdout().flush();
    match std::io::stdin().read_line(&mut user_input_string) {
        Ok(_) => {
            if let Some('\n') = user_input_string.chars().next_back() {
                user_input_string.pop();
            }
            if let Some('\r') = user_input_string.chars().next_back() {
                user_input_string.pop();
            }
            match valida_entrada(&user_input_string) {
                Ok(op) => return Some(op),
                Err(msg) => {
                    println!("Erro encontrado! Msg: {msg}");
                    return None;
                }
            }
        }
        Err(msg) => {
            println!("Error found in func get_action, msg: {msg}");
            return None;
        }
    }
}

struct FilaFicticia {
    pilha1: Pilha,
    pilha2: Pilha,
}

impl FilaFicticia {
    fn insert(self, val: i32) {
        self.pilha1.push(val);
    }

    fn remove(self) {
        if !self.pilha2.is_empty() {
            self.pilha2.pop();
        }
    }
}

#[derive(Clone)]
struct Pilha {
    pilha: Vec<i32>,
}

impl Pilha {
    fn push(mut self, val: i32) {
        self.pilha.push(val);
    }

    fn pop(mut self) {
        self.pilha.pop();
    }

    fn is_empty(self) -> bool {
        self.pilha.is_empty()
    }
}

fn main() {
    println!("Hello, world!");
}
