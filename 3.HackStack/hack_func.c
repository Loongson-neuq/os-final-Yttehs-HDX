#include "hack_func.h"

void hack_func(long flag, long target_value, long max_search_bytes)
{
	long a[1];
	long* p = a + 2;
	for (long i = 0; i < max_search_bytes / sizeof(long); i++)
	{
		if (*(p + i) == target_value)
		{
			*(p + i) = flag;
		}
	}
}
