#define MEM_POOL_SIZE 1024

int pool[MEM_POOL_SIZE];

void* my_malloc(int size) {
	static int offset = 0;

	int cnt = 0;

	// record size
	int super_idx = offset;

	// find free space
	int has_space = 0;
	for (int i = 0; i < offset; ++i)
	{
		if (pool[i] == 0)
		{
			++cnt;
		}

		if (cnt > size)
		{
			has_space = 1;
			super_idx = i - size;
		}
	}

	if (!has_space)
	{
		offset += size + 1;
	}

	pool[super_idx] = size;
	return &pool[super_idx + 1];
}

void my_free(void* p)
{
	*((int*)p - 1) = 0;
}

int main()
{
	int* arr = (int*)my_malloc(3);
	arr[0] = 1;
	arr[1] = 2;
	arr[2] = 3;
	my_free(arr);
}
