mov ax [B]
mul [C]
mov [temp1] ax
mov dx 0
mov ax [temp1]
mov bx [D]
div bx
mov [temp2] ax
mov ax [temp2]
add ax [5]
mov [temp3] ax
mov ax [A]
add ax [temp3]
mov [temp4] ax
mov [Ans] [temp4]
