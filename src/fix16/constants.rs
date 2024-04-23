use super::Fix16;
impl Fix16{
pub const FOUR_DIV_PI: Fix16 = Fix16(0x145F3);            /* Fix16 value of 4/PI */
pub const NEG_FOUR_DIV_PI2: Fix16 = Fix16(-0x67C0i32); //0xFFFF9840);        /* Fix16 value of -4/PI² */
pub const X4_CORRECTION_COMPONENT: Fix16 = Fix16(0x399A); 	/* Fix16 value of 0.225 */
pub const PI_DIV_4: Fix16 = Fix16(0x0000C90F);             /* Fix16 value of PI/4 */
pub const THREE_PI_DIV_4: Fix16 = Fix16(0x00025B2F);       /* Fix16 value of 3PI/4 */

pub const MAXIMUM: Fix16 = Fix16(0x7FFFFFFF); /* the maximum value of fix16_t */
pub const MINIMUM: Fix16 = Fix16(-0x80000000i32); //0x80000000); /* the minimum value of fix16_t */
pub const OVERFLOW: Fix16 = Fix16(-0x80000000i32); //0x80000000); /* the value used to indicate overflows when FIXMATH_NO_OVERFLOW is not specified */

pub const FRAC_PI_4: Fix16 = Fix16(205887 / 4);     /* fix16_t value of quarter of pi */
pub const FRAC_PI_2: Fix16 = Fix16(205887 / 2);     /* fix16_t value of half of pi */
pub const PI: Fix16 = Fix16(205887);     /* fix16_t value of pi */
pub const E: Fix16 = Fix16(178145);     /* fix16_t value of e */
pub const NEG_ONE: Fix16 = Fix16(-0x00010000); /* fix16_t value of 1 */
pub const ZERO: Fix16 = Fix16(0); /* fix16_t value of 0 */
pub const HALF: Fix16 = Fix16(0x8000); /* fix16_t value of 1/2 */
pub const ONE: Fix16 = Fix16(0x00010000); /* fix16_t value of 1 */
pub const TWO: Fix16 = Fix16(0x00020000); /* fix16_t value of 2 */
pub const THREE: Fix16 = Fix16(0x00030000); /* fix16_t value of 3 */
pub const FOUR: Fix16 = Fix16(0x00040000); /* fix16_t value of 3 */
pub const EPS: Fix16 = Fix16(1);          /* fix16_t epsilon */
}