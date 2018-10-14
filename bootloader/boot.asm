org 07c00h

start:
    call cls
    mov si,msg
    call real_mode_print
    jmp end

cls:
    mov ah,0x06
    mov al,0
    mov bh,0x07
    mov ch,0
    mov cl,0
    mov dh,24
    mov dl,79
    int 0x10
    ret

real_mode_print:
    mov di,ax
    mov ah,0x0e
    print_string:
        mov al,[SI]
        cmp al,0
        je end_print
        int 0x10
        add SI,1
    jmp	print_string
    end_print:
    ret
msg: db "Hello,BIOS!"
end:
times 510-($-$$) db 0
dw 0xaa55