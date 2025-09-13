.altmacro

.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm

    .section .text
    .global __alltraps
    .global __restore
    .align 2

__all_traps:
    # exchange kernel stack ptr and user stack ptr
    # now sp -> kernel stack ptr, sscratch -> user stack ptr
    csrrw sp, sscratch, sp

    # allocate trap context on kernel stack
    addi sp, sp, -34 * 8

    # save x1 ~ x32 (exclude tp(x4))
    sd x1, 1*8(sp)
    sd x3, 3*8(sp)
    .set n, 5
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr

    # save sstatus to t0 then to stk
    csrr t0, sstatus
    sd t0, 32*8(sp)
    # save sepc to t1 then to stk
    csrr t1, sepc
    sd t1, 33*8(sp)

    # save user stack to kernel stack
    # 2*8(sp) is the place for sp(x2) register in kernel stack, now sscratch register saves user stack ptr
    csrr t2, sscratch
    sd t2, 2*8(sp)

    # set input arg of trap_handler(ctx: &mut TrapContext)
    mv a0, sp
    call trap_handler

__restore:
    # restore sstatus, sepc, sscratch(which saves value of user stack ptr)
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    ld t2, 2*8(sp)
    csrw sstatus, t0
    csrw sepc, t1
    csrw sscratch, t2

    # restore x1 ~ x32 registers (exclude tp(x4))
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr

    # release trap context on kernel stack
    addi sp, sp, 34*8

    # exchange sp and sscratch
    csrrw sp, sscratch, sp
    # now sp->user stack, sscratch->kernel stack

    sret
