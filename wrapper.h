/* Wrapper functions for batched OPM operations */

#ifndef _WRAPPER_H_
#define _WRAPPER_H_

#include <stdint.h>
#include "opm.h"

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Batch process multiple OPM_Clock cycles.
 * 
 * This function calls OPM_Clock multiple times in a tight C loop,
 * reducing FFI overhead from Rust.
 * 
 * @param chip Pointer to the OPM chip structure
 * @param output Pointer to output buffer for stereo samples (2 x int32_t)
 * @param cycles Number of clock cycles to execute
 */
void OPM_Clock_Batch(opm_t *chip, int32_t *output, uint32_t cycles);

#ifdef __cplusplus
}
#endif

#endif /* _WRAPPER_H_ */
