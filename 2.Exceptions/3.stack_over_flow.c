/* 栈溢出，OS杀死进程
 * make: *** [Makefile:10: run] Segmentation fault (core dumped)
 */

#include <stdio.h>

void recursive_fn()
{
	int a = 0;
	printf("current stack: %p\n", &a);
	recursive_fn();
}

int main()
{
	recursive_fn();
}
