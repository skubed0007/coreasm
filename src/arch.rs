use std::collections::HashMap;

#[allow(dead_code)]
pub enum Bit {
    X32,
    X64,
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum Arch {
    X86,
    Arm,
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum OS {
    Linux,
    Windows,
    Mac,
}

pub struct Target {
    pub bit: Bit,
    pub arch: Arch,
    pub os: OS,
}

impl Target {
    pub fn new(bit: Bit, arch: Arch, os: OS) -> Self {
        Target { bit, arch, os }
    }

    pub fn regs(&self) -> Option<HashMap<String, String>> {
        match (&self.os, &self.arch, &self.bit) {
            (OS::Linux, Arch::X86, Bit::X64) => {
                let mut map = HashMap::new();
                map.insert("mov".to_string(), "mov".to_string());
                map.insert("string".to_string(), "rsi".to_string());
                map.insert("Dest Index Register (i64)".to_string(), "rbx".to_string());
                map.insert("Syscall Write".to_string(), "1".to_string());
                map.insert("Syscall".to_string(), "syscall".to_string());
                map.insert("i32".to_string(), "ebx".to_string());
                map.insert("i64".to_string(), "rbx".to_string());
                map.insert("f32".to_string(), "xmm0".to_string());
                map.insert("f64".to_string(), "xmm1".to_string());
                map.insert("rsi".to_string(), "rsi".to_string());
                map.insert("rdi".to_string(), "rdi".to_string());
                map.insert("rsp".to_string(), "rsp".to_string());
                map.insert("rbp".to_string(), "rbp".to_string());
                map.insert("rax".to_string(), "rax".to_string());
                map.insert("rbx".to_string(), "rbx".to_string());
                map.insert("rcx".to_string(), "rcx".to_string());
                map.insert("rdx".to_string(), "rdx".to_string());
                //map.insert("k", v)
                Some(map)
            }
            (OS::Linux, Arch::X86, Bit::X32) => {
                let mut map = HashMap::new();

                map.insert("mov".to_string(), "mov".to_string());
                map.insert("string".to_string(), "esi".to_string());
                map.insert("Dest Index Register (i64)".to_string(), "ebx".to_string());
                map.insert("Syscall Write".to_string(), "1".to_string());
                map.insert("Syscall".to_string(), "syscall".to_string());
                map.insert("i32".to_string(), "ebx".to_string());
                map.insert("i64".to_string(), "ebx".to_string());
                map.insert("f32".to_string(), "xmm0".to_string());
                map.insert("f64".to_string(), "xmm1".to_string());
                map.insert("rsi".to_string(), "esi".to_string());
                map.insert("rdi".to_string(), "edi".to_string());
                map.insert("rsp".to_string(), "esp".to_string());
                map.insert("rbp".to_string(), "ebp".to_string());
                map.insert("rax".to_string(), "eax".to_string());
                map.insert("rbx".to_string(), "ebx".to_string());
                map.insert("rcx".to_string(), "ecx".to_string());
                map.insert("rdx".to_string(), "edx".to_string());
                map.insert("exit code".to_string(), "1".to_string());

                Some(map)
            }
            (OS::Linux, Arch::Arm, Bit::X64) => {
                let mut map = HashMap::new();
                map.insert("mov".to_string(), "mov".to_string());
                map.insert("string".to_string(), "x0".to_string());
                map.insert("Dest Index Register (i64)".to_string(), "x1".to_string());
                map.insert("Syscall Write".to_string(), "64".to_string());
                map.insert("Syscall".to_string(), "svc 0".to_string());
                map.insert("i32".to_string(), "w1".to_string());
                map.insert("i64".to_string(), "x1".to_string());
                map.insert("f32".to_string(), "s0".to_string());
                map.insert("f64".to_string(), "d0".to_string());
                map.insert("rsi".to_string(), "x0".to_string());
                map.insert("rdi".to_string(), "x1".to_string());
                map.insert("rsp".to_string(), "sp".to_string());
                map.insert("rbp".to_string(), "fp".to_string());
                map.insert("rax".to_string(), "x8".to_string());
                map.insert("rbx".to_string(), "x1".to_string());
                map.insert("rdx".to_string(), "x2".to_string());
                map.insert("rcx".to_string(), "x3".to_string());
                map.insert("exit code".to_string(), "60".to_string());

                Some(map)
            }
            (OS::Linux, Arch::Arm, Bit::X32) => {
                let mut map = HashMap::new();
                map.insert("mov".to_string(), "mov".to_string());
                map.insert("string".to_string(), "r0".to_string());
                map.insert("Dest Index Register (i64)".to_string(), "r1".to_string());
                map.insert("Syscall Write".to_string(), "4".to_string());
                map.insert("Syscall".to_string(), "svc 0".to_string());
                map.insert("i32".to_string(), "r1".to_string());
                map.insert("i64".to_string(), "r2".to_string());
                map.insert("f32".to_string(), "s0".to_string());
                map.insert("f64".to_string(), "d0".to_string());
                map.insert("rsi".to_string(), "r0".to_string());
                map.insert("rdi".to_string(), "r1".to_string());
                map.insert("rsp".to_string(), "sp".to_string());
                map.insert("rbp".to_string(), "fp".to_string());
                map.insert("rax".to_string(), "r7".to_string());
                map.insert("rbx".to_string(), "r3".to_string());
                map.insert("rdx".to_string(), "r2".to_string());
                map.insert("rcx".to_string(), "r3".to_string());
                map.insert("exit code".to_string(), "1".to_string());

                Some(map)
            }
            (OS::Windows, Arch::X86, Bit::X64) => {
                let mut map = HashMap::new();
                map.insert("mov".to_string(), "mov".to_string());
                map.insert("string".to_string(), "rsi".to_string());
                map.insert("Dest Index Register (i64)".to_string(), "rbx".to_string());
                map.insert("Syscall Write".to_string(), "0x80".to_string());
                map.insert("Syscall".to_string(), "syscall".to_string());
                map.insert("i32".to_string(), "ebx".to_string());
                map.insert("i64".to_string(), "rbx".to_string());
                map.insert("f32".to_string(), "xmm0".to_string());
                map.insert("f64".to_string(), "xmm1".to_string());
                map.insert("rsi".to_string(), "rsi".to_string());
                map.insert("rdi".to_string(), "rdi".to_string());
                map.insert("rsp".to_string(), "rsp".to_string());
                map.insert("rbp".to_string(), "rbp".to_string());
                map.insert("rax".to_string(), "rax".to_string());
                map.insert("rbx".to_string(), "rbx".to_string());
                map.insert("rcx".to_string(), "rcx".to_string());
                map.insert("rdx".to_string(), "rdx".to_string());
                map.insert("exit code".to_string(), "0x0".to_string());

                Some(map)
            }

            (OS::Windows, Arch::X86, Bit::X32) => {
                let mut map = HashMap::new();
                map.insert("mov".to_string(), "mov".to_string());
                map.insert("string".to_string(), "esi".to_string());
                map.insert("Dest Index Register (i64)".to_string(), "ebx".to_string());
                map.insert("Syscall Write".to_string(), "0x80".to_string());
                map.insert("Syscall".to_string(), "syscall".to_string());
                map.insert("i32".to_string(), "ebx".to_string());
                map.insert("i64".to_string(), "ebx".to_string());
                map.insert("f32".to_string(), "xmm0".to_string());
                map.insert("f64".to_string(), "xmm1".to_string());
                map.insert("rsi".to_string(), "esi".to_string());
                map.insert("rdi".to_string(), "edi".to_string());
                map.insert("rsp".to_string(), "esp".to_string());
                map.insert("rbp".to_string(), "ebp".to_string());
                map.insert("rax".to_string(), "eax".to_string());
                map.insert("rbx".to_string(), "ebx".to_string());
                map.insert("rcx".to_string(), "ecx".to_string());
                map.insert("rdx".to_string(), "edx".to_string());
                map.insert("exit code".to_string(), "0x0".to_string());

                Some(map)
            }

            (OS::Windows, Arch::Arm, Bit::X64) => {
                let mut map = HashMap::new();
                map.insert("mov".to_string(), "mov".to_string());
                map.insert("string".to_string(), "r0".to_string());
                map.insert("Dest Index Register (i64)".to_string(), "r1".to_string());
                map.insert("Syscall Write".to_string(), "0x80".to_string());
                map.insert("Syscall".to_string(), "svc 0".to_string());
                map.insert("i32".to_string(), "r1".to_string());
                map.insert("i64".to_string(), "r2".to_string());
                map.insert("f32".to_string(), "s0".to_string());
                map.insert("f64".to_string(), "d0".to_string());
                map.insert("rsi".to_string(), "r0".to_string());
                map.insert("rdi".to_string(), "r1".to_string());
                map.insert("rsp".to_string(), "sp".to_string());
                map.insert("rbp".to_string(), "fp".to_string());
                map.insert("rax".to_string(), "r7".to_string());
                map.insert("rbx".to_string(), "r3".to_string());
                map.insert("rdx".to_string(), "r2".to_string());
                map.insert("rcx".to_string(), "r3".to_string());
                map.insert("exit code".to_string(), "0x1".to_string());

                Some(map)
            }

            (OS::Windows, Arch::Arm, Bit::X32) => {
                let mut map = HashMap::new();
                map.insert("mov".to_string(), "mov".to_string());
                map.insert("string".to_string(), "r0".to_string());
                map.insert("Dest Index Register (i64)".to_string(), "r1".to_string());
                map.insert("Syscall Write".to_string(), "0x80".to_string());
                map.insert("Syscall".to_string(), "svc 0".to_string());
                map.insert("i32".to_string(), "r1".to_string());
                map.insert("i64".to_string(), "r2".to_string());
                map.insert("f32".to_string(), "s0".to_string());
                map.insert("f64".to_string(), "d0".to_string());
                map.insert("rsi".to_string(), "r0".to_string());
                map.insert("rdi".to_string(), "r1".to_string());
                map.insert("rsp".to_string(), "sp".to_string());
                map.insert("rbp".to_string(), "fp".to_string());
                map.insert("rax".to_string(), "r7".to_string());
                map.insert("rbx".to_string(), "r3".to_string());
                map.insert("rdx".to_string(), "r2".to_string());
                map.insert("rcx".to_string(), "r3".to_string());
                map.insert("exit code".to_string(), "0x1".to_string());

                Some(map)
            }

            _ => None,
        }
    }
}
