ENTRY(start)

SECTIONS
{
	. = 1M;

	.text :
	{
		*(.header)
		*(.text)
	}
}
