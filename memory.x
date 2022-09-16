/* Linker script for the nRF52, WITHOUT SoftDevice (#4) */
MEMORY
{
	  /* NOTE K = Ki = 1024 bytes */
	  FLASH : ORIGIN = 0x00000000, LENGTH = 1024K
	  RAM   : ORIGIN = 0x20000000, LENGTH = 256K
}
