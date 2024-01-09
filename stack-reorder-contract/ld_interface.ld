PHDRS {
    dummy PT_LOAD FLAGS(5) ;
    text PT_LOAD FLAGS(5) ;
    data PT_LOAD FLAGS(6) ;

    relro PT_GNU_RELRO FLAGS(4) ;
    stack PT_GNU_STACK FLAGS(6) ;
}

ENTRY(__ckb_bootloader_start)

SECTIONS {
    . = 0x0;
    .ckb.vm.dummy : ALIGN(0x1000) {
        BYTE(0)
    } : dummy

    . = 0x80000;

    .dynsym : {
        *(.dynsym)
    }
    .gnu.hash : {
        *(.gnu.hash)
    }
    .hash : {
        *(.hash)
    }
    .dynstr : {
        *(.dynstr)
    }
    .rela : {
        *(.rela)
        *(.rela.*)
    }

    .text : ALIGN(0x1000) {
        *(.text)
        *(.text.*)
    } : text

    .rodata : {
        *(.rodata)
        *(.rodata.*)
    } : text

    .data : ALIGN(0x1000) {
        *(.data)
        *(.data.*)
    } : data

    .dynamic : {
        *(.dynamic)
    } : data : dynamic

    .data.rel.ro : ALIGN(0x1000) {
        *(.data.rel.ro)
        *(.data.rel.ro.*)
    } : relro
    
    .bss : {
        *(.bss)
        *(.bss.*)
    }

    .stack : {
        *(.stack)
    } : stack

    _end = .; PROVIDE (end = .);
}
