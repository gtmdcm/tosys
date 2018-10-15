%define LoaderSegment 0x7e0
org 0x7c00

call cls
mov si,msg
call print
mov si,loader_loaded
call load_loader
call print
jmp LoaderSegment:0x0

; 清屏
cls:
    ; 滚屏一页
    mov ah,0x06
    mov al,0
    mov bh,0x07
    mov ch,0
    mov cl,0
    mov dh,24
    mov dl,79
    int 0x10

    ; 将光标置于(0,0)
    mov ah,0x02
    mov bh,0x00
    mov dh,0x00
    mov dl,0x00
    int 0x10
    ret

; 打印字符串
; @param SI 要打印的字符串的首地址
print:
    mov ah,0x0e
    print_next_char:
        mov al,[si]
        cmp al,0
        je end_print
        int 0x10
        add si,1
    jmp	print_next_char
    end_print:
    ret

load_loader:
    mov ax,LoaderSegment
    mov es,ax
    mov bx,0
    mov ah,0x02
    ; 读0x10个扇区，装我们的内核暂时绰绰由于
    mov al,0x10
    mov ch,0x00
    mov cl,0x02
    mov dh,0x00
    mov dl,0x00
    int 0x13
    ret

msg: db "Hello,BIOS!",0x0d,0x0a,0x00                ; 0x0d,0x0a = "\r\n"
loader_loaded: db "Loader loaded!",0x0d,0x0a,0x00   ; 0x0d,0x0a = "\r\n"
; 用0填充
times 510-($-$$) db 0
; 启动扇区标记
dw 0xaa55