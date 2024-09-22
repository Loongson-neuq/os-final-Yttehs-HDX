/* 强制使用高特权级寄存器，OS 终止程序
 * gcc -Wall -g -o 2.reach_high_privilege.out 2.reach_high_privilege.c
 * 2.reach_high_privilege.c: Assembler messages:
 * 2.reach_high_privilege.c:10: Error: bad register name `%%rip'
 * make: *** [Makefile:9: run] Error 1
 */

int main()
{
	asm volatile ("movl $114, %%rip");
}
