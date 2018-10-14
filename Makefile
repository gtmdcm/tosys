boot.bin: ./bootloader/boot.asm
	nasm ./bootloader/boot.asm -o boot.bin

a.img: boot.bin
	dd if=boot.bin of=a.img bs=512 count=1 conv=notrunc

run: a.img
	bochs -f ./bochsrc.txt