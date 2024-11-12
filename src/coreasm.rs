use std::collections::HashMap;

pub struct CoreAsm {
    pub data: Data,
    pub prints: Vec<Print>,
    pub exit: bool,
}
#[allow(dead_code)]

impl CoreAsm {
    pub fn exit(&mut self, exit: bool) {
        self.exit = exit;
    }

    pub fn new() -> Self {
        CoreAsm {
            data: Data::new(),
            prints: Vec::new(),
            exit: false,
        }
    }

    pub fn add_print(&mut self) -> &mut Print {
        let print = Print::new();
        self.prints.push(print);
        self.prints.last_mut().unwrap()
    }

    pub fn add_text_to_last_print(&mut self, text: String) {
        if let Some(print) = self.prints.last_mut() {
            print.add_token(PrintToken::Text(text));
        }
    }

    pub fn add_var_to_last_print(&mut self, var_name: String) {
        if let Some(print) = self.prints.last_mut() {
            print.add_token(PrintToken::Variable(var_name));
        }
    }

    pub fn add_newline_to_last_print(&mut self) {
        if let Some(print) = self.prints.last_mut() {
            print.add_token(PrintToken::Newline);
        }
    }
}

pub struct Data {
    pub variables: HashMap<String, Var>,
}

pub struct Print {
    pub tokens: Vec<PrintToken>,
}

pub enum PrintToken {
    Text(String),
    Variable(String),
    Newline,
}
#[allow(dead_code)]

impl Print {
    pub fn new() -> Self {
        Print { tokens: Vec::new() }
    }

    pub fn add_token(&mut self, token: PrintToken) {
        self.tokens.push(token);
    }
}

impl Data {
    pub fn new() -> Self {
        Data {
            variables: HashMap::new(),
        }
    }

    pub fn mkvar(&mut self, name: String, var_type: Types, value: VarValue) {
        let var = Var { var_type, value };
        self.variables.insert(name, var);
    }
}

pub struct Var {
    pub var_type: Types,
    pub value: VarValue,
}

#[allow(dead_code)]
#[derive(PartialEq, Eq, Debug)]
pub enum Types {
    I32,
    I64,
    F32,
    F64,
    String,
}
#[allow(dead_code)]

pub enum VarValue {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    String(String),
}

#[allow(dead_code)]
pub struct Entry {
    pub code: Vec<String>,
}
#[allow(dead_code)]

impl Entry {
    pub fn new() -> Self {
        Entry { code: Vec::new() }
    }

    pub fn push_code(&mut self, code: String) {
        self.code.push(code);
    }
}
