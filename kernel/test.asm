; 临时测试，检测进入保护模式是否成功
[section .text]
global _start
_start:
	add eax,1
	jmp _start