section .data
	Ans   DW 1
	A     DW 1
	B     DW 1
	C     DW 1
	D     DW 1
section .bss
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
	mov [temp4],ax
	mov ax,[temp4]
	mov [Ans],ax
