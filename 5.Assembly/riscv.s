	.section .text
	.globl __max_element

__max_element:
	# ptr          1 8
	# long         1 8
	# unsigned int 3 4
	addi sp, sp, -8
	sw ra, 0(sp)

	mv t0, a0 # unsigned int* vec
	mv t1, a1 # long count
	li t2, 0  # unsigned int max_e = 0
	li t3, 0  # unsigned int i = 0

__loop:
	beq t3, t1, __end
	lw t4, 0(t0) # register unsigned int veci = vec[i]
	blt t4, t2, __skip
	mv t2, t4

__skip:
	addi t3, t3, 1
	addi t0, t0, 4
	j __loop

__end:
	mv a0, t2

	lw ra, 0(sp)
	addi sp, sp, 8
	ret
