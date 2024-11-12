use crate::{
    arch::Target,
    coreasm::{self, CoreAsm, PrintToken, VarValue},
};
use std::collections::HashMap;

pub fn maker(core_asm: &CoreAsm, arch: Target) -> String {
    let arch_map: HashMap<String, String> = arch.regs().unwrap();
    let mut asm_code = String::new();
    let mut data_section = String::new();
    let mut entry_section = String::new();
    let mut cvs: HashMap<String, String> = HashMap::new();
    let mut str_index = 555;
    data_section.push_str("jnl: db 0x0A\n");

    for (name, var) in &core_asm.data.variables {
        let var_type = match &var.var_type {
            crate::coreasm::Types::I32 => "dd",
            crate::coreasm::Types::I64 => "dq",
            crate::coreasm::Types::F32 => "dd",
            crate::coreasm::Types::F64 => "dq",
            crate::coreasm::Types::String => "db",
        };
        let value = match &var.value {
            VarValue::I32(i) => i.to_string(),
            VarValue::I64(i) => i.to_string(),
            VarValue::F32(f) => f.to_string(),
            VarValue::F64(f) => f.to_string(),
            VarValue::String(s) => format!("\"{}\"", s),
        };
        data_section.push_str(&format!("{} {} {}\n", name, var_type, value));
    }

    let mut pindex = 0;
    for ps in core_asm.prints.iter() {
        pindex += 1;
        for token in ps.tokens.iter() {
            match token {
                PrintToken::Text(text) => {
                    let var_name = format!("str_{}", str_index + pindex);
                    str_index += 1;
                    cvs.insert(var_name.clone(), text.clone());
                    data_section.push_str(&format!("{} db \"{}\"\n", var_name, text));
                    let string_register = arch_map.get("string").unwrap();
                    let string_length = text.len().to_string();

                    entry_section.push_str(&format!(
                        "{} {}, {}\n",
                        arch_map.get("mov").unwrap(),
                        string_register,
                        var_name
                    ));

                    entry_section.push_str(&format!(
                        "{} {}, {}\n",
                        arch_map.get("mov").unwrap(),
                        arch_map.get("Dest Index Register (i64)").unwrap(),
                        arch_map.get("Syscall Write").unwrap()
                    ));

                    entry_section.push_str(&format!(
                        "{} {}, {}\n",
                        arch_map.get("mov").unwrap(),
                        arch_map.get("rdx").unwrap(),
                        string_length
                    ));

                    entry_section.push_str(&format!(
                        "{} {}, {}\n",
                        arch_map.get("mov").unwrap(),
                        arch_map.get("rax").unwrap(),
                        arch_map.get("Syscall Write").unwrap()
                    ));

                    entry_section.push_str(&format!("{}\n", arch_map.get("Syscall").unwrap()));
                }
                PrintToken::Variable(var_name) => {
                    if let Some(var) = core_asm.data.variables.get(var_name) {
                        let var_type = &var.var_type;
                        let register = match var_type {
                            coreasm::Types::I32 => arch_map.get("i32").unwrap(),
                            coreasm::Types::I64 => arch_map.get("i64").unwrap(),
                            coreasm::Types::F32 => arch_map.get("f32").unwrap(),
                            coreasm::Types::F64 => arch_map.get("f64").unwrap(),
                            coreasm::Types::String => arch_map.get("string").unwrap(),
                        };
                        let value = match &var.value {
                            VarValue::I32(i) => i.to_string(),
                            VarValue::I64(i) => i.to_string(),
                            VarValue::F32(f) => f.to_string(),
                            VarValue::F64(f) => f.to_string(),
                            VarValue::String(s) => format!("\"{}\"", s),
                        };

                        entry_section.push_str(&format!(
                            "{} {}, {}\n",
                            arch_map.get("mov").unwrap(),
                            register,
                            var_name
                        ));

                        entry_section.push_str(&format!(
                            "{} {}, {}\n",
                            arch_map.get("mov").unwrap(),
                            arch_map.get("Dest Index Register (i64)").unwrap(),
                            arch_map.get("Syscall Write").unwrap()
                        ));

                        entry_section.push_str(&format!(
                            "{} {}, {}\n",
                            arch_map.get("mov").unwrap(),
                            arch_map.get("rdx").unwrap(),
                            (value.len() - 2).to_string()
                        ));

                        entry_section.push_str(&format!(
                            "{} {}, {}\n",
                            arch_map.get("mov").unwrap(),
                            arch_map.get("rax").unwrap(),
                            arch_map.get("Syscall Write").unwrap()
                        ));

                        entry_section.push_str(&format!("{}\n", arch_map.get("Syscall").unwrap()));
                    }
                }
                PrintToken::Newline => {
                    entry_section.push_str(&format!(
                        "{} {}, {}\n",
                        arch_map.get("mov").unwrap(),
                        arch_map.get("Dest Index Register (i64)").unwrap(),
                        arch_map.get("Syscall Write").unwrap()
                    ));
                    entry_section.push_str(&format!(
                        "{} {}, jnl\n",
                        arch_map.get("mov").unwrap(),
                        arch_map.get("rsi").unwrap()
                    ));
                    entry_section.push_str(&format!(
                        "{} {}, 1\n",
                        arch_map.get("mov").unwrap(),
                        arch_map.get("rdx").unwrap()
                    ));
                    entry_section.push_str(&format!(
                        "{} {}, {}\n",
                        arch_map.get("mov").unwrap(),
                        arch_map.get("rax").unwrap(),
                        arch_map.get("Syscall Write").unwrap()
                    ));
                    entry_section.push_str(&format!("{}\n", arch_map.get("Syscall").unwrap()));
                }
            }
        }
    }

    asm_code.push_str("SECTION .data\n");
    for line in data_section.lines() {
        asm_code.push_str(&format!("     {}\n", line));
    }
    asm_code.push_str("SECTION .text\n      global _start\n_start:\n");
    for line in entry_section.lines() {
        asm_code.push_str(&format!("     {}\n", line));
    }
    //println!("exit : {}", core_asm.exit);
    if core_asm.exit {
        // Dynamically retrieve registers and values from the architecture map
        let rax_reg = arch_map.get("rax").unwrap();
        let rdi_reg = arch_map.get("rdi").unwrap();
        let exit_syscall = arch_map.get("exit code").unwrap(); // 60 is the syscall number for exit on x86_64
        let exit_code = "0";

        asm_code.push_str(
            format!(
                "     {} {}, {}\n",
                arch_map.get("mov").unwrap(),
                rax_reg,
                exit_syscall
            )
            .as_str(),
        );

        asm_code.push_str(
            format!(
                "     {} {}, {}\n",
                arch_map.get("mov").unwrap(),
                rdi_reg,
                exit_code
            )
            .as_str(),
        );

        // Trigger the syscall
        asm_code.push_str(format!("     {}\n", arch_map.get("Syscall").unwrap()).as_str());
    }

    asm_code
}
