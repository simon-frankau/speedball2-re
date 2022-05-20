//
// Sound extractor
//
// Dump a human-readable version of the sound data.
//

use std::fs;
use std::str;

const BANK_START: usize = 0x010280;
const BANK_END: usize = 0x011a42;

const BANK_HEADER_LEN: usize = 0x22;

const INSTRUMENT_SIZE: usize = 0x3f;
const OPERATOR_SIZE: usize = 0x0b;

fn print_operator(idx: usize, data: &[u8]) {
    println!("  {:1} En: {:1} Detune: {:1} Mul: {:2} TL: {:3} RS: {:1} AR: {:2} AM: {:1} D1R: {:2} D2R: {:2} D1L: {:3} RR: {:2}",
        idx,
        data[0x00],
        data[0x01],
        data[0x02],
        data[0x03],
        data[0x04],
        data[0x05],
        data[0x06],
        data[0x07],
        data[0x08],
        data[0x09],
        data[0x0a],
    );
}

fn print_instrument(idx: usize, data: &[u8]) {
    let name = str::from_utf8(&data[0..0x0a]).unwrap();
        println!("{:02x} Name: '{}'", idx, name);
        println!("Duration: {:3}, Transposition: {:03}, FM feedback: {:02}, FM algorithm: {:02}",
            data[0x0a],
            data[0x0b] as i32 - 0x80,
            data[0x0c],
            data[0x0d],
        );
        println!("Glissando size: {:02}, !gliss flag: {:03}, gliss duration: {:03}, vibrato multiplier: {:02}, vib rate: {:03}",
            data[0x0e] as i32 - 0x80,
            data[0x0f],
            data[0x10],
            data[0x11],
            data[0x12],
       );
       for (idx, operator) in data[0x13..].chunks(OPERATOR_SIZE).enumerate() {
           print_operator(idx, operator);
       }
        println!("");
}

fn main() {
    let data = fs::read("../../speedball2-usa.bin").unwrap();

    let instrument_bank = data[BANK_START+BANK_HEADER_LEN..BANK_END]
        .chunks(INSTRUMENT_SIZE);

    for (idx, instrument) in instrument_bank.enumerate() {
        print_instrument(idx, instrument);
    }
}
