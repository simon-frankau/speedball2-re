//
// Sound extractor
//
// Dump a human-readable version of the sound data.
//

use std::collections::HashMap;
use std::fs;
use std::str;

const BANK_START: usize = 0x010280;
const BANK_END: usize = 0x011a42;

const BANK_HEADER_LEN: usize = 0x22;

const INSTRUMENT_SIZE: usize = 0x3f;
const OPERATOR_SIZE: usize = 0x0b;

const SEQ_START: usize = 0x011b22;
const SEQ_END: usize = 0x0135b0;

const SEQ_TABLE_START: usize = 0x0135b0;
const SEQ_TABLE_END: usize = 0x013770;

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

fn name_instruments(data: &[u8]) -> Vec<String> {
   data[BANK_START+BANK_HEADER_LEN..BANK_END]
       .chunks(INSTRUMENT_SIZE)
       .map(|instr| str::from_utf8(&instr[0..0x0a]).unwrap().to_string())
       .collect()
}

////////////////////////////////////////////////////////////////////////
// Sequence dump
//

fn print_sequences(data: &[u8], instr_names: &[String]) {

    // Create a table of sounds starting at each address, so we can print out lables.
    let sequence_table = data[SEQ_TABLE_START..SEQ_TABLE_END]
        .chunks(4)
        .map(|quad| { match quad {
                [a, b, c, d] => ((*a as usize) << 24) | ((*b as usize) << 16) | ((*c as usize) << 8) | (*d as usize),
                _ => panic!("Nope"),
            }
        })
        .collect::<Vec<_>>();

    let mut sequence_map = HashMap::new();
    for (idx, addr) in sequence_table.iter().enumerate() {
        let v = sequence_map.entry(addr).or_insert_with(|| Vec::new());
        v.push(idx);
    }

    let mut instrument_bank = data[SEQ_START..SEQ_END].iter().enumerate();

    while let Some((offset, instr)) = instrument_bank.next() {
        let addr = SEQ_START + offset;
        if let Some(idxs) = sequence_map.get(&addr) {
            for idx in idxs.iter() {
                println!("Sound {:02x}:", idx);
            }
        }

        // NB: We print a blank line after commands that end a sequence,
        // to give a visual reference to the gaps between sequences.
        match instr {
            0x80 => println!("    Set 0x36 {}",
                instrument_bank.next().unwrap().1),
            0x84 => println!("    Noop (0x84)"),
            0x88 => println!("    Go to start\n"),
            0x8c => println!("    Set note length {}",
                instrument_bank.next().unwrap().1),
            0x90 => println!("    Rest"),
            0x94 => println!("    Set tempo {}",
                instrument_bank.next().unwrap().1),
            0x98 => println!("    Noop (0x98)"),
            0x9c => println!("    Noop (0x9c) {}",
                instrument_bank.next().unwrap().1),
            0xa0 => println!("    Noop (0xa0)"),
            0xa4 => println!("    Noop (0xa4)"),
            0xa8 => println!("    Or with 0x38 {}",
                instrument_bank.next().unwrap().1),
            0xac => println!("    Stop\n"),
            0xb0 => println!("    Call {:02x}",
                instrument_bank.next().unwrap().1),
            0xb4 => println!("    Return\n"),
            0xb8 => println!("    Add to 3a {}",
                instrument_bank.next().unwrap().1),
            0xbc => println!("    Set 3a {}",
                instrument_bank.next().unwrap().1),
            0xc0 => println!("    For {}",
                instrument_bank.next().unwrap().1),
            0xc4 => println!("    Next"),
            0xc8 => println!("    Noop (0xc8)"),
            0xcc => println!("    Noop (0xcc)"),
            0xd0 => {
                // NB: Misses the indirection via the mapping table, but that
                // is very nearly the identity function.
                let i = instrument_bank.next().unwrap().1;
                println!("    Set instrument {:02x} ({})", i, instr_names[*i as usize]);
            },
            0xd4 => println!("    Jump to {:02x}\n",
                instrument_bank.next().unwrap().1),

            n if *n < 0x80 => println!("    Note {}", n),
            n => println!("    Invalid op 0x{:02x}", n),
        }
    }
}

////////////////////////////////////////////////////////////////////////
// Entry point
//

fn main() {
    let data = fs::read("../../speedball2-usa.bin").unwrap();

    let instrument_names = name_instruments(&data);

    print_instruments(&data);
    println!("");
    println!("");
    print_sequences(&data, &instrument_names);
}
