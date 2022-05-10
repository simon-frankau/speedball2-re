//
// 'custom_tile_map' appears to be a QPAC2-JMP decoder
//

uint custom_tile_map(undefined *source,undefined *dest)

{
    byte bVar1;
    ushort uVar3;
    short sVar4;
    ushort unaff_D2w;
    uint curr_data;
    uint checksum;
    int *piVar7;
    uint *puVar8;
    uint *puVar9;
    uint *data_ptr;
    bool bVar15;
    bool bVar16;
    bool bVar17;
    bool bVar18;
    bool bVar19;
    bool bVar20;
    bool bVar21;
    bool bVar22;

    /* Called by display_splash if a bit is set at the start of the image. Looks
       like it writes it to 0xff1106? */

    /* Check for 16-letter magic "QPAC2-JMP(C)1989". */
    int *magic_chk = (int *)source;
    int *magic_tgt = QPAC_MAGIC_ID;
    for (int uVar3 = 4; uVar3 != 0; uVar3--) {
        if (*magic_tgt++ != *magic_chk++) {
            return 1;
        }
    }

    byte *end_ptr = dest + *magic_chk;  // First int - length
    uint checksum = magic_chk[1]; // Second - checksum
    data_ptr = magic_chk + 2;     // And then the data begins.
    curr_data = 0;
    do {

// In the bitstream:

// 00 XXX .... - Copy X + 1 bytes of data directly to the output stream.
// 01 XXXX XXXX - Look back X + 1 bytes, copy 2 bytes of data.
// 100 XXXX XXXX X - Look back X + 1 bytes, copy 3 bytes of data.
// 101 XXXX XXXX XX - Look back X + 1 bytes, copy 4 bytes of data.
// 110 XXXX XXXX YYYY YYYY YYYY - Look back Y + 1 bytes, copy X + 3 bytes.
// 111 XXXX XXXX - Copy X + 9 bytes of data directly to the output stream.

        while( true ) {
            bVar15 = read_next_bit();
            if (!bVar15) break;
            bVar15 = read_next_bit();
            if (bVar15) {
                bVar15 = read_next_bit();
                if (bVar15) {
                    uVar3 = read_8_bits();
                    unaff_D2w = unaff_D2w + 8;
                    goto LAB_0000f42c;
                }
                read_8_bits();
                unaff_D2w = unaff_D2w + 2;
                sVar4 = 0xb;
            }
            else {
                bVar15 = read_next_bit();
                if (bVar15) {
                    sVar4 = 9;
                    unaff_D2w = 3;
                }
                else {
                    sVar4 = 8;
                    unaff_D2w = 2;
                }
            }
        LAB_0000f504:
            // Read sVar4 + 1 bits into uVar3.
            uVar3 = 0;
            do {
                bVar15 = read_next_bit();
                uVar3 = (ushort)bVar15 + uVar3 * 2;
                sVar4 = sVar4 + -1;
            } while (sVar4 != -1);

            // Copy unaff_D2w + 1 bytes from uVar3 + 1 back.
            byte *lookback_src = dest + (-1 - (short)uVar3);
            do {
                *dest++ = *lookback_src++;
                unaff_D2w = unaff_D2w - 1;
            } while (unaff_D2w != 0xffff);
            if (dest >= end_ptr <= dest) {
                goto end;
            }
        }
        bVar15 = read_next_bit();
        if (bVar15) {
            sVar4 = 7;
            unaff_D2w = 1;
            goto LAB_0000f504;
        }

        byte acc = 0;

        for (int i = 0; i < 3; i++) {
            bVar15 = read_next_bit();
            acc = acc << 1 + bVar15;
        }

        unaff_D2w = (ushort)acc;

    LAB_0000f42c:
        // Copy unaff_D2w bytes.
        do {
            // Looks like old value doesn't matter, as it's a byte we're
            // dealing with, so it's totally replaced with data.
            bVar1 = (char)uVar3;

            for (int i = 0; i < 8; i++) {
                bVar15 = read_next_bit();
                bVar1 = bVar1 << 1 + bVar15;
            }

            uVar3 = (ushort)bVar1;
            *dest++ = bVar1;
            unaff_D2w = unaff_D2w - 1;

        } while (unaff_D2w != 0xffff);
    } while (dest < end_ptr);

end:
    if (dest != end_ptr) {
        return 3;
    }
    if (checksum == 0) {
        return checksum;
    }
    return 2;
}


bool read_next_bit() {
    bool bVar15 = CARRY4(curr_data,curr_data);
    curr_data = curr_data * 2;
    if (curr_data == 0) {
        curr_data = *data_ptr++;
        checksum ^= curr_data;
        bVar15 = CARRY4(curr_data + 1,curr_data);
        curr_data = curr_data + curr_data + 1;
    }
    return bVar15;
}
