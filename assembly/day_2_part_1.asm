global f:function

f:
    xor eax, eax

    lea edx, [input]

@@c:
    mov ecx, [edx]
    test ecx, ecx
    jz @@gg

    add edx, 4
    mov ebx, [edx]

    sub ecx, "A"
    sub ebx, "X"

    mov esi, ecx
    shl esi, 2
    add esi, ebx

    add eax, [scoring+esi*4]
    add edx, 4
    jmp @@c
    
@@gg:
    ret

segment .data
    input dd "A", "Y", "B", "X", "C", "Z", 0
    scoring dd 4, 8, 3, 0, 1, 5, 9, 0, 7, 2, 6, 0
