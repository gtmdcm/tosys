%define KernelAddress 0x7f00
org 0x7e00

mov si,msg
call print
; 关闭外部中断
call close_pic
; 关闭内部中断
cli
; 我不做分段了！
mov ax,0x0
mov ds,ax
; 加载GDT和IDT
lgdt [gdt_info]
lidt [idt_info]
call wait_keyboard_control_ready
; 告诉键盘控制芯片我们要写数据了
   mov al,0xd1
out 0x64,al
; 此处应有
; call wait_keyboard_control_ready
; 但nasm不这么认为，也许是一个bug
; 不过没有任何糟糕的事情发生

; A20启动！
mov al,0xdf
out 0x60, al
call wait_keyboard_control_ready

; farewell, real mode!
; hello, protect mode!
mov eax,0x0001
mov cr0,eax
; it's kernel time!
jmp 8:KernelAddress

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

; 等待键盘控制器缓冲区空，表示可以写东西了
; 为啥是键盘控制器？
; IBM你个……
wait_keyboard_control_ready:
    times 6 nop
	in  al,0x64
	and al,0x02
	jnz wait_keyboard_control_ready
	ret

; 关闭pic芯片
close_pic:
    mov al,0xff
    ; 关掉master pic
    out 0x21,al
    ; 有些怪怪机器不支持连续out……
    nop
    ; 关掉slave pic
    out 0xa1,al
    ret

msg: db "Loader at your service!",0x0d,0x0a,0x00 ; 0x0d,0x0a = "\r\n"

gdt:
	times 8 db 0
	; 内核数据段
	dw      0x07ff,0x0000,0x9a00,0x00c0
	; 内核代码段
	dw      0x07ff,0x0000,0x9200,0x00c0
	dw      0

gdt_info:
    dw 0x800
    dd gdt

idt_info:
    dw 0
    dw 0,0

end:
    hlt
    jmp end
times 512-($-$$) db 0