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

const SEQ_START: usize = 0x011b22;
const SEQ_END: usize = 0x0135b0;

////////////////////////////////////////////////////////////////////////
// Instrument dump
//

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

fn print_instruments(data: &[u8]) {
   let instrument_bank = data[BANK_START+BANK_HEADER_LEN..BANK_END]
        .chunks(INSTRUMENT_SIZE);

    for (idx, instrument) in instrument_bank.enumerate() {
        print_instrument(idx, instrument);
    }
}

////////////////////////////////////////////////////////////////////////
// Sequence dump
//

fn print_sequences(data: &[u8]) {
    let mut instrument_bank = data[SEQ_START..SEQ_END].iter();

    while let Some(instr) = instrument_bank.next() {
        match instr {
            0x80 => println!("Set 0x36 {}",
                instrument_bank.next().unwrap()),
            0x84 => println!("Noop (0x84)"),
            0x88 => println!("Go to start"),
            0x8c => println!("Set note length {}",
                instrument_bank.next().unwrap()),
            0x90 => println!("Continue"),
            0x94 => println!("Set tempo {}",
                instrument_bank.next().unwrap()),
            0x98 => println!("Noop (0x98)"),
            0x9c => println!("Noop (0x9c) {}",
                instrument_bank.next().unwrap()),
            0xa0 => println!("Noop (0xa0)"),
            0xa4 => println!("Noop (0xa4)"),
            0xa8 => println!("Or with 0x38 {}",
                instrument_bank.next().unwrap()),
            0xac => println!("Stop"),
            0xb0 => println!("Call {:02x}",
                instrument_bank.next().unwrap()),
            0xb4 => println!("Return"),
            0xb8 => println!("Add to 3a {}",
                instrument_bank.next().unwrap()),
            0xbc => println!("Set 3a {}",
                instrument_bank.next().unwrap()),
            0xc0 => println!("For {}",
                instrument_bank.next().unwrap()),
            0xc4 => println!("Next"),
            0xc8 => println!("Noop (0xc8)"),
            0xcc => println!("Noop (0xcc)"),
            0xd0 => println!("Set instrument {:02x}",
                instrument_bank.next().unwrap()),
            0xd4 => println!("Jump to {:02x}",
                instrument_bank.next().unwrap()),

            n if *n < 0x80 => println!("Note {}", n),
            n => println!("Invalid op 0x{:02x}", n),
        }
    }
}

////////////////////////////////////////////////////////////////////////
// Entry point
//

fn main() {
    let data = fs::read("../../speedball2-usa.bin").unwrap();

    print_instruments(&data);
    println!("");
    println!("");
    print_sequences(&data);
}
