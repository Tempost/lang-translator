section .data
        A DW 5
        B DW 5
        C DW 5
        D DW 10
        Ans DW 0
section .bss
        temp1 RESW 1
        temp2 RESW 1
        temp3 RESW 1
        temp4 RESW 1
        global _start
section .text
_start: nop
        mov ax,[B]
        mov bx,[C]
        mul bx
        mov [temp1],ax
        mov dx,0
        mov ax,[temp1]
        mov bx,[D]
        div bx
        mov [temp2],ax
        mov ax,[temp2]
        add ax,[5]
        mov [temp3],ax
        mov ax,[A]
        add ax,[temp3]
        mov [Ans],ax
