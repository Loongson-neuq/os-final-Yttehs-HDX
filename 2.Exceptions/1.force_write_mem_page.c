/* 强制对没有 W(rite) 权限的页面进行写入，OS 终止程序运行
 * gcc -Wall -g -o 1.force_write_mem_page.out 1.force_write_mem_page.c
 * ./1.force_write_mem_page.out
 * readonly: 0x58f6eef0b004, rw: 0x58f6eef0d018
 * make: *** [Makefile:10: run] Segmentation fault (core dumped)
 */

#include <stdio.h>

const int realonly = 114514;
int rw = 114514;

void force_write()
{
	*(int*)&realonly = 1919810;
}

int main()
{
	printf("readonly: %p, rw: %p\n", &realonly, &rw);
	force_write();
}
