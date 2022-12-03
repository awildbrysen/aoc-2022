global f:function

f:
	xor eax, eax
	xor ecx, ecx
	xor edx, edx

	mov ecx, [input]
	lea edx, [input]

	@@j:
	add edx, 4
	add ecx, [edx]
	cmp dword [edx], 0
	jne @@j

	cmp eax, ecx
	cmovl eax, ecx
	xor ecx, ecx
	cmp edx, end
	jl @@j

	ret

segment .data
	input dd 1000, 2000, 3000, 0, 4000, 0, 5000, 6000, 0, 7000, 8000, 9000, 0, 10000
	end dd 0
