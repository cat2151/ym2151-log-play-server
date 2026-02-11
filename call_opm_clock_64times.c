#include <stdint.h>
#include "opm.h"

// Include opm.c so this wrapper shares the same translation unit, ensuring optimizer visibility.
#include "opm.c"

void call_opm_clock_64times(opm_t *chip, int32_t *output)
{
    for (int i = 0; i < 64; i++)
    {
        OPM_Clock(chip, output, NULL, NULL, NULL);
    }
}
