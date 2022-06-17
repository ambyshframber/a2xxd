use std::env::args;
use std::fs::read;

fn main() {
    let a: Vec<String> = args().collect();

    let filename = a.get(1).expect("filename is required!");
    let rom = read(&filename).expect("error opening file");
    let (rom_trim, header) = get_header(&rom);
    let start_offset = header.start_offset();

    println!("format: {}", header.format_name());

    let bytes_per_row = a.get(2).map(|n|
        n.parse::<usize>().unwrap_or(BYTES_PER_ROW)
    ).unwrap_or(BYTES_PER_ROW);

    disassemble(rom_trim, start_offset, bytes_per_row)
}

fn get_header(rom: &[u8]) -> (&[u8], AvcHeader) {
    if &rom[..4] == b"AVC\x00" {
        (&rom[4..], AvcHeader::Avc2V1)
    }
    else if &rom[..4] == b"AVD\x00" {
        (&rom[4..], AvcHeader::AvdV1)
    }
    else {
        (rom, AvcHeader::None)
    }
}
enum AvcHeader {
    None,
    Avc2V1, // AVC\x00
    AvdV1, // AVD\x00
}
impl AvcHeader {
    pub fn start_offset(&self) -> u16 {
        match self {
            Self::Avc2V1 => 0x0300,
            _ => 0,
        }
    }
    pub fn format_name(&self) -> &str {
        match self {
            Self::None => "headerless/other",
            Self::Avc2V1 => "AVC2 Version 1 ROM",
            Self::AvdV1 => "AVD Version 1 Archive"
        }.into()
    }
}

// trim headers first
fn disassemble(rom: &[u8], start_offset: u16, bytes_per_row: usize) {
    for (i, c) in rom.chunks(bytes_per_row).enumerate() {
        print_row((i * bytes_per_row) as u16 + start_offset, c, bytes_per_row)
    }
}

fn print_row(ofs: u16, row: &[u8], bpr: usize) {
    print!("{:04x}: ", ofs);

    let mut hex_row = String::new();
    let mut chars = String::new();
    let mut ops = String::new();
    for b in row {
        hex_row.push_str(&format!("{:02x} ", b));
        chars.push(sanitise_char(*b));
        ops.push_str(&format!("{:OPCODE_WIDTH$}", OPS[*b as usize]))
    }

    while hex_row.len() < bpr * 3 {
        hex_row.push(' ')
    }
    print!("{} ", hex_row);

    while chars.len() < bpr {
        chars.push(' ')
    }
    print!("{}  ", chars);

    println!("{}", ops)    
}

fn sanitise_char(c: u8) -> char {
    let is_valid = c < 0x7f && c >= 0x20;
    let c_out = is_valid as u8 * c;
    let dot_out = !is_valid as u8 * b'.';
    (c_out + dot_out) as char
}

const BYTES_PER_ROW: usize = 4;
const OPCODE_WIDTH: usize = 7;
const OPS: [&str; 256] = ["", "", "", "POP", "SWP", "ROT", "DUP", "OVR", "EQU", "GTH", "JMP", "JNZ", "JSR", "STH", "", "", "LDZ", "STZ", "LDR", "STR", "LDA", "STA", "PIC", "PUT", "ADC", "SBC", "MUL", "DVM", "AND", "IOR", "XOR", "SFT", "SEC", "", "", "POP2", "SWP2", "ROT2", "DUP2", "OVR2", "EQU2", "GTH2", "JMP2", "JNZ2", "JSR2", "STH2", "", "", "LDZ2", "STZ2", "LDR2", "STR2", "LDA2", "STA2", "PIC2", "PUT2", "ADC2", "SBC2", "MUL2", "DVM2", "AND2", "IOR2", "XOR2", "SFT2", "CLC", "", "", "POPr", "SWPr", "ROTr", "DUPr", "OVRr", "EQUr", "GTHr", "JMPr", "JNZr", "JSRr", "STHr", "", "", "LDZr", "STZr", "LDRr", "STRr", "LDAr", "STAr", "PICr", "PUTr", "ADCr", "SBCr", "MULr", "DVMr", "ANDr", "IORr", "XORr", "SFTr", "EXT", "", "", "POPr2", "SWPr2", "ROTr2", "DUPr2", "OVRr2", "EQUr2", "GTHr2", "JMPr2", "JNZr2", "JSRr2", "STHr2", "", "", "LDZr2", "STZr2", "LDRr2", "STRr2", "LDAr2", "STAr2", "PICr2", "PUTr2", "ADCr2", "SBCr2", "MULr2", "DVMr2", "ANDr2", "IORr2", "XORr2", "SFTr2", "LIT", "", "", "RTI", "", "", "", "", "EQUk", "GTHk", "JMPk", "JNZk", "JSRk", "STHk", "", "", "LDZk", "STZk", "LDRk", "STRk", "LDAk", "STAk", "PICk", "PUTk", "ADCk", "SBCk", "MULk", "DVMk", "ANDk", "IORk", "XORk", "SFTk", "LIT2", "", "", "", "", "", "", "", "EQUk2", "GTHk2", "JMPk2", "JNZk2", "JSRk2", "STHk2", "", "", "LDZk2", "STZk2", "LDRk2", "STRk2", "LDAk2", "STAk2", "PICk2", "PUTk2", "ADCk2", "SBCk2", "MULk2", "DVMk2", "ANDk2", "IORk2", "XORk2", "SFTk2", "LITr", "", "", "", "", "", "", "", "EQUkr", "GTHkr", "JMPkr", "JNZkr", "JSRkr", "STHkr", "", "", "LDZkr", "STZkr", "LDRkr", "STRkr", "LDAkr", "STAkr", "PICkr", "PUTkr", "ADCkr", "SBCkr", "MULkr", "DVMkr", "ANDkr", "IORkr", "XORkr", "SFTkr", "LITr2", "", "", "", "", "", "", "", "EQUkr2", "GTHkr2", "JMPkr2", "JNZkr2", "JSRkr2", "STHkr2", "", "", "LDZkr2", "STZkr2", "LDRkr2", "STRkr2", "LDAkr2", "STAkr2", "PICkr2", "PUTkr2", "ADCkr2", "SBCkr2", "MULkr2", "DVMkr2", "ANDkr2", "IORkr2", "XORkr2", "SFTkr2"];

#[allow(soft_unstable)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn san_test() {
        assert_eq!(sanitise_char(0x00), '.');
        assert_eq!(sanitise_char(0x68), 'h');
        assert_eq!(sanitise_char(0x20), ' ');
        assert_eq!(sanitise_char(0xff), '.');
        assert_eq!(sanitise_char(0x83), '.');
    }

    #[test]
    fn disass_test() {
        disassemble(&[0x00, 0xff, 0x06, 0x68, 0x83, 0x35], 0x0300, 4)
    }
}
