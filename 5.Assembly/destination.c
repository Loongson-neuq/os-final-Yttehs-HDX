unsigned int max_element(unsigned int* vec, long count)
{
	unsigned int max_e = 0;

	int i = 0;

loop:
	if (i == count)
		goto end;
	register unsigned int veci = vec[i];
	if (veci < max_e)
		goto skip;
	max_e = veci;
	
skip:
	++i;
	goto loop;

end:
	return max_e;

}
