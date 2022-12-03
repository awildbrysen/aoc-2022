Thanks @yugecin for teaching me

nasm -f elf32 day_1_part_1.asm -o asm.o && gcc -Wall -fno-pic -m32 -x c -c c.c -o c.o && gcc -Wall -m32 -o binary asm.o c.o && ./binary

Yes this is a rust repo but this seemed fun to try :shrug:
