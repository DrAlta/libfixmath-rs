use crate::Fix16;

pub const FOUR_DIV_PI: Fix16 = Fix16(0x145F3);            /* Fix16 value of 4/PI */
pub const _FOUR_DIV_PI2: Fix16 = Fix16(-0x67C0i32); //0xFFFF9840);        /* Fix16 value of -4/PI² */ //todo rename to NEG_FOUR_DIV_PI2
pub const X4_CORRECTION_COMPONENT: Fix16 = Fix16(0x399A); 	/* Fix16 value of 0.225 */
pub const PI_DIV_4: Fix16 = Fix16(0x0000C90F);             /* Fix16 value of PI/4 */
pub const THREE_PI_DIV_4: Fix16 = Fix16(0x00025B2F);       /* Fix16 value of 3PI/4 */

pub const fix16_maximum: Fix16 = Fix16(0x7FFFFFFF); /* the maximum value of fix16_t */
pub const fix16_minimum: Fix16 = Fix16(-0x80000000i32); //0x80000000); /* the minimum value of fix16_t */
pub const fix16_overflow: Fix16 = Fix16(-0x80000000i32); //0x80000000); /* the value used to indicate overflows when FIXMATH_NO_OVERFLOW is not specified */

pub const fix16_pi: Fix16 = Fix16(205887);     /* fix16_t value of pi */
pub const fix16_e: Fix16 = Fix16(178145);     /* fix16_t value of e */
pub const fix16_one: Fix16 = Fix16(0x00010000); /* fix16_t value of 1 */
pub const fix16_eps: Fix16 = Fix16(1);          /* fix16_t epsilon */
