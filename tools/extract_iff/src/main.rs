//
// IFF extractor
//
// Just pulls out sub-ranges of the ROM to separate files. Easy!
//

use std::fs;

const OUT_DIR: &str = "out";

fn main() -> anyhow::Result<()> {
    let data = fs::read("../../speedball2-usa.bin")?;

    fs::create_dir_all(OUT_DIR)?;
    
    for (start, end, name) in &[
        (0x013806, 0x01736e, "start.smp"),
        (0x01736e, 0x01ae16, "end.smp"),
        (0x01ae16, 0x01e392, "getready.smp"),
        (0x01e392, 0x02246c, "replay.smp"),
    ] {
        fs::write(format!("{}/{}", OUT_DIR, name), &data[*start..*end])?;
    }

    Ok(())
}
