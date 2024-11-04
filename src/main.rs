use clap::Parser;
use rustyline::DefaultEditor;
use std::{collections::HashMap, fs::read_to_string, process::exit};

const VERSION: &str = "0.1.0";

#[derive(Parser, Debug)]
#[command(
    name = "Stack NT",
    version = VERSION,
    author = "梶塚太智 <kajizukataichi@outlook.jp>",
    about = "A new type of Stack programming language with functional",
)]
struct Cli {
    /// Run the script file
    #[arg(index = 1)]
    file: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let mut rl = DefaultEditor::new().unwrap();
    let mut stacknt = Core::new();

    if let Some(path) = cli.file {
        if let Ok(code) = read_to_string(path) {
            stacknt.eval(Core::parse(code));
        } else {
            eprintln!("Error! it fault to open the file");
        }
    } else {
        println!("Stack NT");
        loop {
            let mut code = String::new();
            loop {
                let enter = rl.readline("> ").unwrap_or_default();
                if enter.is_empty() {
                    break;
                }
                rl.add_history_entry(&enter).unwrap_or_default();
                code += &format!("{enter}\n");
            }

            let program = Core::parse(code.to_string());
            stacknt.eval(program);

            println!("Stack: {:?}", stacknt.stack);
        }
    }
}

#[derive(Clone, Debug)]
enum Type {
    Number(f64),
    String(String),
    Bool(bool),
    Symbol(String),
    Function(Function),
    Block(Vec<Type>),
    Null,
}

impl Type {
    fn get_number(&self) -> f64 {
        match self {
            Type::Number(n) => n.to_owned(),
            _ => 0.0,
        }
    }

    fn get_string(&self) -> String {
        match self {
            Type::String(s) | Type::Symbol(s) => s.to_owned(),
            Type::Number(n) => n.to_string(),
            _ => String::new(),
        }
    }

    fn get_bool(&self) -> bool {
        match self {
            Type::Bool(n) => n.to_owned(),
            _ => false,
        }
    }

    fn get_block(&self) -> Vec<Type> {
        match self {
            Type::Block(b) => b.to_owned(),
            other => vec![other.to_owned()],
        }
    }
}

#[derive(Clone, Debug)]
enum Function {
    BuiltIn(fn(&mut Core)),
    UserDefined(Vec<Type>),
}

#[derive(Clone, Debug)]
struct Core {
    stack: Vec<Type>,
    memory: HashMap<String, Type>,
    returns: usize,
}

impl Core {
    fn new() -> Core {
        Core {
            stack: vec![],
            memory: HashMap::from([
                (
                    "+".to_string(),
                    Type::Function(Function::BuiltIn(|core| {
                        let b = core.pop().get_number();
                        let a = core.pop().get_number();
                        core.stack.push(Type::Number(a + b))
                    })),
                ),
                (
                    "-".to_string(),
                    Type::Function(Function::BuiltIn(|core| {
                        let b = core.pop().get_number();
                        let a = core.pop().get_number();
                        core.stack.push(Type::Number(a - b))
                    })),
                ),
                (
                    "*".to_string(),
                    Type::Function(Function::BuiltIn(|core| {
                        let b = core.pop().get_number();
                        let a = core.pop().get_number();
                        core.stack.push(Type::Number(a * b))
                    })),
                ),
                (
                    "/".to_string(),
                    Type::Function(Function::BuiltIn(|core| {
                        let b = core.pop().get_number();
                        let a = core.pop().get_number();
                        core.stack.push(Type::Number(a / b))
                    })),
                ),
                (
                    "%".to_string(),
                    Type::Function(Function::BuiltIn(|core| {
                        let b = core.pop().get_number();
                        let a = core.pop().get_number();
                        core.stack.push(Type::Number(a % b))
                    })),
                ),
                (
                    "^".to_string(),
                    Type::Function(Function::BuiltIn(|core| {
                        let b = core.pop().get_number();
                        let a = core.pop().get_number();
                        core.stack.push(Type::Number(a.powf(b)))
                    })),
                ),
                (
                    "=".to_string(),
                    Type::Function(Function::BuiltIn(|core| {
                        let b = core.pop().get_string();
                        let a = core.pop().get_string();
                        core.stack.push(Type::Bool(a == b))
                    })),
                ),
                (
                    "!=".to_string(),
                    Type::Function(Function::BuiltIn(|core| {
                        let b = core.pop().get_string();
                        let a = core.pop().get_string();
                        core.stack.push(Type::Bool(a != b))
                    })),
                ),
                (
                    "&".to_string(),
                    Type::Function(Function::BuiltIn(|core| {
                        let b = core.pop().get_bool();
                        let a = core.pop().get_bool();
                        core.stack.push(Type::Bool(a && b))
                    })),
                ),
                (
                    "|".to_string(),
                    Type::Function(Function::BuiltIn(|core| {
                        let b = core.pop().get_bool();
                        let a = core.pop().get_bool();
                        core.stack.push(Type::Bool(a || b))
                    })),
                ),
                (
                    "concat".to_string(),
                    Type::Function(Function::BuiltIn(|core| {
                        let b = core.pop().get_string();
                        let a = core.pop().get_string();
                        core.stack.push(Type::String(a + &b))
                    })),
                ),
                (
                    "print".to_string(),
                    Type::Function(Function::BuiltIn(|core| {
                        let a = core.pop().get_string();
                        print!("{a}");
                    })),
                ),
                (
                    "println".to_string(),
                    Type::Function(Function::BuiltIn(|core| {
                        let a = core.pop().get_string();
                        println!("{a}");
                    })),
                ),
                (
                    "if-else".to_string(),
                    Type::Function(Function::BuiltIn(|core| {
                        let code_false = core.pop().get_block();
                        let code_true = core.pop().get_block();
                        let condition = core.pop().get_bool();
                        if condition {
                            core.eval(code_true);
                        } else {
                            core.eval(code_false);
                        }
                    })),
                ),
                (
                    "when".to_string(),
                    Type::Function(Function::BuiltIn(|core| {
                        let code_true = core.pop().get_block();
                        let condition = core.pop().get_bool();
                        if condition {
                            core.eval(code_true);
                        }
                    })),
                ),
                (
                    "while".to_string(),
                    Type::Function(Function::BuiltIn(|core| {
                        let code_true = core.pop().get_block();
                        let condition = core.pop().get_block();
                        while {
                            core.eval(condition.clone());
                            core.pop().get_bool()
                        } {
                            core.eval(code_true.clone());
                        }
                    })),
                ),
                (
                    "return".to_string(),
                    Type::Function(Function::BuiltIn(|core| {
                        let level = core.pop().get_number();
                        core.returns = level as usize;
                    })),
                ),
                (
                    "let".to_string(),
                    Type::Function(Function::BuiltIn(|core| {
                        let name = core.pop().get_string();
                        let value = core.pop();
                        core.memory.insert(name, value);
                    })),
                ),
                (
                    "eval".to_string(),
                    Type::Function(Function::BuiltIn(|core| {
                        let code = core.pop().get_block();
                        core.eval(code);
                    })),
                ),
                (
                    "defun".to_string(),
                    Type::Function(Function::BuiltIn(|core| {
                        let name = core.pop().get_string();
                        let func = core.pop().get_block();
                        core.memory
                            .insert(name, Type::Function(Function::UserDefined(func)));
                    })),
                ),
                (
                    "exit".to_string(),
                    Type::Function(Function::BuiltIn(|core| {
                        exit(core.pop().get_number() as i32)
                    })),
                ),
                (
                    "pop".to_string(),
                    Type::Function(Function::BuiltIn(|core| {
                        core.pop();
                    })),
                ),
                ("new-line".to_string(), Type::String("\n".to_string())),
                ("double-quote".to_string(), Type::String("\"".to_string())),
                ("tab".to_string(), Type::String("\t".to_string())),
            ]),
            returns: 0,
        }
    }

    fn parse(source: String) -> Vec<Type> {
        let mut result = vec![];
        for token in Core::tokenize_expr(source) {
            let mut token = token.trim().to_string();
            if let Ok(n) = token.parse::<f64>() {
                result.push(Type::Number(n));
            } else if token.starts_with('"') && token.ends_with('"') {
                token.remove(token.find('"').unwrap_or_default());
                token.remove(token.rfind('"').unwrap_or_default());
                result.push(Type::String(token));
            } else if token.starts_with("{") && token.ends_with("}") {
                token.remove(token.find('{').unwrap_or_default());
                token.remove(token.rfind('}').unwrap_or_default());
                result.push(Type::Block(Core::parse(token)));
            } else {
                result.push(Type::Symbol(token));
            }
        }
        result
    }

    fn tokenize_expr(input: String) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut current_token = String::new();
        let mut in_parentheses: usize = 0;
        let mut in_quote = false;

        for c in input.chars() {
            match c {
                '{' if !in_quote => {
                    in_parentheses += 1;
                    current_token.push(c);
                }
                '}' if !in_quote => {
                    if in_parentheses != 0 {
                        current_token.push(c);
                        in_parentheses -= 1;
                        if in_parentheses == 0 {
                            tokens.push(current_token.clone());
                            current_token.clear();
                        }
                    }
                }
                '"' => {
                    if in_parentheses == 0 {
                        if in_quote {
                            current_token.push(c);
                            in_quote = false;
                            tokens.push(current_token.clone());
                            current_token.clear();
                        } else {
                            in_quote = true;
                            current_token.push(c);
                        }
                    } else {
                        current_token.push(c);
                    }
                }
                ' ' | '　' | '\n' | '\t' | '\r' => {
                    if in_parentheses != 0 || in_quote {
                        current_token.push(c);
                    } else if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                }
                _ => {
                    current_token.push(c);
                }
            }
        }

        if !(in_parentheses != 0 || in_quote || current_token.is_empty()) {
            tokens.push(current_token);
        }
        tokens
    }

    fn eval(&mut self, program: Vec<Type>) {
        for order in program {
            match order {
                Type::Symbol(name) => {
                    if let Some(value) = self.memory.get(&name) {
                        if let Type::Function(Function::BuiltIn(func)) = value {
                            func(self)
                        } else if let Type::Function(Function::UserDefined(func)) = value {
                            self.eval(func.to_vec());
                        } else {
                            self.stack.push(value.to_owned());
                        }
                    } else {
                        self.stack.push(Type::Symbol(name));
                    }
                }
                other => self.stack.push(other),
            }
            if self.returns != 0 {
                self.returns -= 1;
                return;
            }
        }
    }

    fn pop(&mut self) -> Type {
        if let Some(value) = self.stack.pop() {
            value
        } else {
            Type::Null
        }
    }
}
