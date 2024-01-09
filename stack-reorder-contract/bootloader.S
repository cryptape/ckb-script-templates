.global __ckb_bootloader_start
__ckb_bootloader_start:
  li t4, (0x400000 - 0x80000) /* Difference between new & old stack region */
  mv t1, sp
  sub t2, t1, t4 /* t2 points to new stack, while t1 points to old stack */
  ld t0, 0(t1)
  sd zero, 0(t1)
  sd t0, 0(t2)
  addi t1, t1, 8
  addi t2, t2, 8
  /* Load and correct argv pointers */
1:
  beqz t0, 2f
  ld t6, 0(t1)
  sub t6, t6, t4
  sd zero, 0(t1)
  sd t6, 0(t2)
  addi t1, t1, 8
  addi t2, t2, 8
  addi t0, t0, -1
  j 1b
2:
  /*
   * Copy remaining data over to new stack, this is essentially a combination
   * of memcpy & memset, we do it this way so stack is not used at all.
   */
  li t5, 0x400000
3:
  bge t1, t5, 4f
  ld t6, 0(t1)
  sd zero, 0(t1)
  sd t6, 0(t2)
  addi t1, t1, 8
  addi t2, t2, 8
  j 3b
4:
  /* Set new sp, clear used registers */
  sub sp, sp, t4
  li t0, 0
  li t1, 0
  li t2, 0
  li t4, 0
  li t5, 0
  li t6, 0
  /* TODO: setup the first page starting as address 0 as executable */
  /* Boot as usual */
  j _start
