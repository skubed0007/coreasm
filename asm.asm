SECTION .data
     jnl: db 0x0A
     name db "joy"
     str_556 db "Hello"
     str_557 db "!"
SECTION .text
      global _start
_start:
     mov rsi, str_556
     mov rbx, 1
     mov rdx, 5
     mov rax, 1
     syscall
     mov rsi, name
     mov rbx, 1
     mov rdx, 3
     mov rax, 1
     syscall 
     mov rsi, str_557
     mov rbx, 1
     mov rdx, 1
     mov rax, 1
     syscall
     mov rbx, 1
     mov rsi, jnl
     mov rdx, 1
     mov rax, 1
     syscall
      mov rax, 60        ; Syscall number for exit
      mov rdi, 0         ; Exit status
      mov rbx, 1         ; Exit syscall
      syscall           ; Trigger syscall