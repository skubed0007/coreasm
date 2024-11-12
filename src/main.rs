use arch::Target;
use coreasm::{CoreAsm, Print, PrintToken};
use maker::maker;

mod arch;
mod coreasm;
mod maker;

fn main() {
    // Initialize CoreAsm and add variables
    let mut asm = CoreAsm::new();
    // add variables to data section
    asm.data.mkvar(
        "name".to_string(),
        coreasm::Types::String,
        coreasm::VarValue::String("joy".to_string()),
    );
    //print
    let mut print = Print::new();
    print.tokens.push(PrintToken::Text("Hello ".to_string()));
    print.tokens.push(PrintToken::Variable("name".to_string()));
    print.tokens.push(PrintToken::Text("!".to_string()));
    print.tokens.push(PrintToken::Newline);
    asm.prints.push(print);
    //generate assembly code
    asm.exit(true);
    let asmcode = maker(
        &asm,
        Target::new(arch::Bit::X64, arch::Arch::X86, arch::OS::Windows),
    );
    println!("{}", asmcode);
}
