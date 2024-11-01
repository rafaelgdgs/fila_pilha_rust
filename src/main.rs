use std::io::Write;

#[derive(PartialEq)]
enum Operations {
    Add(i32),
    Remove,
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

    if parts.len() != 2 && parts[0] == "Add" {
        return Err("Entrada inválida. Use o formato: <operação>: <valor>".to_owned());
    }

    let operation = parts[0].trim();
    let mut value_str = "";
    if parts.len() > 1 {
        value_str = parts[1].trim();
    }

    match operation {
        "Add" => match extrair_value_fn_valida_entrada(&value_str) {
            Ok(n) => return Ok(Operations::Add(n)),
            Err(msg) => return Err(msg),
        },
        "Remove" => return Ok(Operations::Remove),
        "Show" => return Ok(Operations::Show),
        "Quit" => return Ok(Operations::Quit),
        _ => return Err("Operação invalida!".to_owned()),
    }
}

fn get_action() -> Result<Operations, String> {
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
                Ok(op) => return Ok(op),
                Err(msg) => {
                    return Err(format!("Erro encontrado! Msg: {msg}"));
                }
            }
        }
        Err(msg) => {
            return Err(format!("Error found in func get_action, msg: {msg}"));
        }
    }
}

#[derive(Debug)]
struct FilaFicticia {
    pilha1: Pilha,
    pilha2: Pilha,
}

impl FilaFicticia {
    fn new() -> Self {
        FilaFicticia {
            pilha1: Pilha::new(),
            pilha2: Pilha::new(),
        }
    }

    fn insert(&mut self, val: i32) {
        self.pilha1.push(val);
    }

    fn remove(&mut self) {
        if !self.pilha2.is_empty() {
            self.pilha2.pop();
        } else {
            while !self.pilha1.is_empty() {
                match self.pilha1.pop() {
                    Some(n) => self.pilha2.push(n),
                    None => break,
                }
            }
            self.pilha2.pop();
        }
    }
}

#[derive(Debug)]
struct Pilha {
    pilha: Vec<i32>,
}

impl Pilha {
    fn new() -> Self {
        Pilha { pilha: Vec::new() }
    }

    fn push(&mut self, val: i32) {
        self.pilha.push(val);
    }

    fn pop(&mut self) -> Option<i32> {
        self.pilha.pop()
    }

    fn is_empty(&self) -> bool {
        self.pilha.is_empty()
    }
}

fn main() {
    let mut fila: FilaFicticia = FilaFicticia::new();
    loop {
        let escolha = get_action();
        if escolha == Ok(Operations::Quit) {
            break;
        }
        match escolha {
            Ok(op) => match op {
                Operations::Add(a) => fila.insert(a),
                Operations::Remove => fila.remove(),
                Operations::Show => println!("{:?}", fila),
                Operations::Quit => {
                    println!("Obrigado por testar. ate mais!");
                    break;
                }
            },
            Err(msg) => {
                println!("Erro encontrado, mensagem: {msg}")
            }
        }
    }
}
