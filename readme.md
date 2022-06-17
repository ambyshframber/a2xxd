# a2xxd

hexdumper/disassembler for avc2 roms and related binary files

## usage

`a2xxd FILE [BYTES-PER-ROW]`

if file is `-`, stdin is used. `BYTES-PER-ROW` defaults to 4.

a2xxd will determine the type of the file from the magic number. currently only AVC2 Version 1 ROMs and AVD Version 1 Archives are supported, but this will change as more file types are introduced. the offset will be automatically adjusted for ROMs.

## example output

```
$ a2xxd hello_world.avcr
format: AVC2 Version 1 ROM
0300: a0 03 1b 94  ....  LIT2   POP    DVM    LDAk   
0304: 06 80 07 0b  ....  DUP    LIT    OVR    JNZ    
0308: 80 01 a0 ff  ....  LIT           LIT2   SFTkr2 
030c: 0f 15 a0 ff  ....         STA    LIT2   SFTkr2 
0310: 09 15 a0 00  ....  GTH    STA    LIT2          
0314: 01 40 38 a0  .@8.         CLC    ADC2   LIT2   
0318: 03 03 2a 68  ..*h  POP    POP    JMP2   EQUr2  
031c: 65 6c 6c 6f  ello  ROTr2  JSRr2  JSRr2         
0320: 20 77 6f 72   wor  SEC    PUTr2         LDRr2  
0324: 6c 64 21 0a  ld!.  JSRr2  SWPr2         JMP    
0328: 00           .            
```
