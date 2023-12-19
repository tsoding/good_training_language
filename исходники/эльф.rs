use std::fs;
use std::path::Path;
use пп::{Инструкция, Программа};

pub fn сгенерировать(путь_к_файлу: &Path, программа: &Программа) -> Option<()> {
    let размер_заголовков: u64 = 64 + 56;
    let точка_входа: u64 = 0x400000;
    let начало_данных = точка_входа + размер_заголовков;
    let mut код = vec![];

    код.extend(&программа.данные);

    for инструкция in &программа.код {
        match инструкция {
            // «Короткое» проталкивание (i8)  "\x6A\x7F"
            // «Длиннре» проталкивание (i32) "\x68\x00\x00\x00\x00"
            Инструкция::ПротолкнутьЦелое(значение) => {
                assert!(*значение <= i32::MAX as usize);
                код.push(0x68);
                код.extend((*значение as i32).to_le_bytes());
                // СДЕЛАТЬ: реализовать поддержу «коротких» проталкиваний для целых чисел.
            }
            Инструкция::ПротолкнутьУказатель(указатель) => {
                let значение = указатель + начало_данных as usize;
                assert!(значение <= i32::MAX as usize);
                код.push(0x68);
                код.extend((значение as i32).to_le_bytes());
            },
            Инструкция::ПечатьСтроки => {
                код.extend([0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00]); // mov rax, 1
                код.extend([0x48, 0xC7, 0xC7, 0x01, 0x00, 0x00, 0x00]); // mov rdi, 1
                код.extend([0x5e]); // pop rsi
                код.extend([0x5A]); // pop rdx
                код.extend([0x0F, 0x05]); // syscall
            },
            Инструкция::Возврат => {
                // syscall(SYS_exit, 0)
                код.extend([0x48, 0xC7, 0xC0, 0x3C, 0x00, 0x00, 0x00]); // mov rax, 60
                код.extend([0x48, 0xC7, 0xC7, 0x00, 0x00, 0x00, 0x00]); // mov rdi, 0
                код.extend([0x0F, 0x05]); // syscall
            }
        }
    }

    let mut байты: Vec<u8> = Vec::new();
    байты.extend([0x7f, 0x45, 0x4c, 0x46,
                  0x02, 0x01, 0x01, 0x00,
                  0x00, 0x00, 0x00, 0x00,
                  0x00, 0x00, 0x00, 0x00]); // e_ident
    байты.extend(2u16.to_le_bytes()); // e_type
    байты.extend(62u16.to_le_bytes()); // e_machine
    байты.extend(1u32.to_le_bytes()); // e_version
    байты.extend((точка_входа + размер_заголовков + программа.данные.len() as u64).to_le_bytes()); // e_entry
    байты.extend(64u64.to_le_bytes()); // e_phoff
    байты.extend(0u64.to_le_bytes()); // e_shoff
    байты.extend(0u32.to_le_bytes()); // e_flags
    байты.extend(64u16.to_le_bytes()); // e_ehsize
    байты.extend(56u16.to_le_bytes()); // e_phentsize
    байты.extend(1u16.to_le_bytes()); // e_phnum
    байты.extend(64u16.to_le_bytes()); // e_shentsize
    байты.extend(0u16.to_le_bytes()); // e_shnum
    байты.extend(0u16.to_le_bytes()); // e_shstrndx

    байты.extend(1u32.to_le_bytes()); // p_type
    байты.extend(7u32.to_le_bytes()); // p_flags
    байты.extend(0u64.to_le_bytes()); // p_offset
    байты.extend(точка_входа.to_le_bytes()); // p_vaddr
    байты.extend(точка_входа.to_le_bytes()); // p_paddr
    байты.extend((размер_заголовков + код.len() as u64).to_le_bytes()); // p_filesz
    байты.extend((размер_заголовков + код.len() as u64).to_le_bytes()); // p_memsz
    байты.extend(4096u64.to_le_bytes()); // p_align

    байты.extend(&код);

    match fs::write(путь_к_файлу, &байты) {
        Ok(_) => {
            println!("ИНФО: сгенерирован файл «{путь_к_файлу}»",
                     путь_к_файлу = путь_к_файлу.display());
            Some(())
        }
        Err(ошибка) => {
            eprintln!("ОШИБКА: не удалось записать файл «{путь_к_файлу}»: {ошибка}",
                      путь_к_файлу = путь_к_файлу.display());
            None
        }
    }
}
