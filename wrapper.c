/* Wrapper functions for batched OPM operations
 * 
 * This file provides batch-processing wrappers around the Nuked-OPM
 * functions to reduce FFI call overhead from Rust.
 */

#include <stddef.h>
#include <stdint.h>
#include "opm.h"

/**
 * Batch process multiple OPM_Clock cycles.
 * 
 * This function calls OPM_Clock multiple times in a tight C loop,
 * reducing FFI overhead from Rust. The output buffer accumulates
 * the result from the last OPM_Clock call.
 * 
 * @param chip Pointer to the OPM chip structure
 * @param output Pointer to output buffer for stereo samples (2 x int32_t)
 * @param cycles Number of clock cycles to execute
 */
void OPM_Clock_Batch(opm_t *chip, int32_t *output, uint32_t cycles) {
    for (uint32_t i = 0; i < cycles; i++) {
        OPM_Clock(chip, output, NULL, NULL, NULL);
    }
}
