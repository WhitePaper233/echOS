.altmacro

.macro SAVE_SN n
    sd s\n, (\n+2)*8(a0)
.endm

.macro LOAD_SN n
    ld s\n, (\n+2)*8(a1)
.endm

    .section .text
    .global __switch

# __switch(curr_stk_ctx_ptr: *TaskContext, next_stk_ctx_ptr: *TaskContext)
# struct TaskContext:
#   ra: usize,
#   sp: usize,
#   s: [usize; 12],
# curr_stk_ctx_ptr -> a0
# next_stk_ctx_ptr -> a1
__switch:
    # stage 1: save task context of current task
    sd ra, 0(a0)
    sd sp, 8(a0)
    .set n, 0
    .rept 12
        SAVE_SN %n
        .set n, n + 1
    .endr

    # stage 2: restore task context of next task
    ld ra, 0(a1)
    ld sp, 8(a1)
    .set n, 0
    .rept 12
        LOAD_SN %n
        .set n, n + 1
    .endr

    ret
