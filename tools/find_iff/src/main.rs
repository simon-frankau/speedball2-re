//
// IFF finder
//
// Find the addresses that may be the start of an IFF block.
//

use std::collections::HashSet;
use std::fs;
use std::str;

// From previous reversing, we know everything after this address is
// graphics.
const SEARCH_END: usize = 0x02fd0a;

struct IFF {
    tag: String,
    start: usize,
    end: usize,
    len: usize,
}

fn try_find_iff_header(data: &[u8], addr: usize) -> Option<IFF> {
    let candidate = &data[addr..];

    fn is_printable_ascii(c: &u8) -> bool {
        32 <= *c && *c <= 126
    }

    // Tag must be printable ASCII.
    if !candidate.iter().take(4).all(is_printable_ascii) {
        return None;
    }

    // Chunk must not be empty and must fit in ROM.
    let len = (candidate[4] as usize) << 24 | (candidate[5] as usize) << 16 | (candidate[6] as usize) << 8 | (candidate[7] as usize);

    // Length field does not include header.
    let end = addr + len + 8;

    if len == 0 || end > SEARCH_END {
        return None;
    }

    Some(IFF {
        tag: str::from_utf8(&candidate[..4]).unwrap().to_string(),
        start: addr,
        end,
        len,
    })
}

fn main() {
    let data = fs::read("../../speedball2-usa.bin").unwrap();

    // Our algorithm is to mark as high confidence any IFF block
    // that's part of a chain - i.e. the end of one block matches the
    // start of another.

    // Could have done more functional style. Didn't feel like it. :)
    let mut candidates = Vec::new();
    for addr in 0..SEARCH_END {
        if let Some(iff) = try_find_iff_header(&data, addr) {
            candidates.push(iff);
        }
    }

    // Candidates are ordered, so this isn't strictly necessary...
    let starts = candidates.iter().map(|c| c.start).collect::<HashSet<_>>();

    let mut confident = HashSet::new();
    for iff in candidates.iter() {
        if starts.contains(&iff.end) {
            confident.insert(iff.start);
            confident.insert(iff.end);
        }
    }

    // Only print out the tags we're confident in. Eyeballing the rest
    // suggests they're all junk!
    for iff in candidates.iter() {
        if confident.contains(&iff.start) {
            println!("Address: {:06x}, Tag: {}, Length: {:06x}, End: {:06x}",
                iff.start, iff.tag, iff.len, iff.end);
        }
    }
}
