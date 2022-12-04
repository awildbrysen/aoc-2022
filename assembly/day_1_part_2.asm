global f:function

f:
	push ebx
	push esi
	push edi

	xor eax, eax
	xor ecx, ecx
	xor edx, edx

	xor esi, esi
	xor edi, edi
	xor ebx, ebx

	mov ecx, [input]
	lea edx, [input]

	@@j:
	add edx, 4
	add ecx, [edx]
	cmp dword [edx], 0
	jne @@j

	cmp esi, ecx
	jg @@b2
	xchg esi, ecx

	@@b2:
	cmp edi, ecx
	jg @@b3
	xchg edi, ecx

	@@b3:
	cmp ebx, ecx
	jg @@continue
	xchg ebx, ecx

	@@continue:
	xor ecx, ecx
	cmp edx, end
	jl @@j

	mov eax, esi
	add eax, edi
	add eax, ebx

	pop edi
	pop esi
	pop ebx

	ret

segment .data
	input dd 1000, 2000, 3000, 0, 4000, 0, 5000, 6000, 0, 7000, 8000, 9000, 0, 10000
	end dd 0
