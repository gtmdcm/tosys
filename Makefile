build:
	make a.img

run: a.img
	bochs -f ./bochsrc.txt

boot.bin: ./bootloader/boot.asm
	nasm ./bootloader/boot.asm -o boot.bin

loader.bin: ./bootloader/loader.asm
	nasm ./bootloader/loader.asm -o loader.bin

kernel.bin: ./kernel/test.asm
	nasm ./kernel/test.asm -o kernel.bin

a.img: boot.bin loader.bin kernel.bin
	dd if=boot.bin of=a.img bs=512 count=1 conv=notrunc
	dd if=loader.bin of=a.img bs=512 seek=1 conv=sync
	dd if=kernel.bin of=a.img bs=512 seek=2 conv=sync

clean:
	rm *.bin