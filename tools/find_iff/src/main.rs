//
// IFF finder
//
// Find the addresses that may be the start of an IFF block.
//

use std::fs;

// From previous reversing, we know everything after this address is
// graphics.
const SEARCH_END: usize = 0x02fd0a;

fn looks_like_iff_header(data: &[u8], addr: usize) -> bool {
    let candidate = &data[addr..];

    fn is_printable_ascii(c: &u8) -> bool {
        32 <= *c && *c <= 126
    }

    // Tag must be printable ASCII.
    if !candidate.iter().take(4).all(is_printable_ascii) {
        return false;
    }

    // Chunk must not be empty and must fit in ROM.
    let len = (candidate[4] as usize) << 24 | (candidate[5] as usize) << 16 | (candidate[6] as usize) << 8 | (candidate[7] as usize);

    // Length field does not include header.
    let end = addr + len + 8;

    if len == 0 || end > SEARCH_END {
        return false;
    }

    println!("Candidate at {:06x}, length {:06x}, finishes {:06x}",
        addr, len, end);

    true
}

fn main() {
    let data = fs::read("../../speedball2-usa.bin").unwrap();

    for addr in 0..SEARCH_END {
        if looks_like_iff_header(&data, addr) {
            // Nothing for now.
        }
    }
}
