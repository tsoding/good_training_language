use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use компилятор::ПП;
use компилятор::ВидИнструкции;
use Результат;

pub fn сгенерировать_исполняемый_файл(путь_к_исполняемому: &Path, пп: &ПП, точка_входа_программы: usize) -> Результат<()> {
    let путь_к_фазму = путь_к_исполняемому.with_extension("fasm");
    let файл = fs::File::create(&путь_к_фазму).map_err(|ошибка| {
        eprintln!("ОШИБКА: не удалось открыть файл «{путь_к_фазму}»: {ошибка}",
                  путь_к_фазму = путь_к_фазму.display());
    })?;
    let mut файл = io::BufWriter::new(файл);
    let _ = writeln!(&mut файл, "format ELF64 executable");
    let _ = writeln!(&mut файл, "entry _start");
    let _ = writeln!(&mut файл, "_start:");
    let _ = writeln!(&mut файл, "    call инструкция_{точка_входа_программы}");
    let _ = writeln!(&mut файл, "    mov rax, 60");
    let _ = writeln!(&mut файл, "    mov rdi, 0");
    let _ = writeln!(&mut файл, "    syscall");
    for (индекс, инструкция) in пп.код.iter().enumerate() {
        let _ = writeln!(&mut файл, "инструкция_{индекс}: ;;; {путь_к_файлу}:{строка}:{столбец}: {вид_инструкции:?}",
                         путь_к_файлу = инструкция.лок.путь_к_файлу.display(),
                         строка = инструкция.лок.строка,
                         столбец = инструкция.лок.столбец,
                         вид_инструкции = инструкция.вид);
        match инструкция.вид {
            ВидИнструкции::Ноп => {}
            ВидИнструкции::ПротолкнутьЦелое(значение) => {
                assert!(значение <= i32::MAX as usize, "СДЕЛАТЬ: реализовать проталкивание больших чисел");
                let _ = writeln!(&mut файл, "    push {значение}");
            }
            ВидИнструкции::ПротолкнутьУказатель(указатель) => {
                assert!(указатель <= i32::MAX as usize);
                let _ = writeln!(&mut файл, "    push данные+{указатель}");
            }
            ВидИнструкции::Вытолкнуть(количество) => {
                // СДЕЛАТЬ: можеть быть стоит напрямую модифицировать регистр rsp одной операцией?
                for _ in 0..количество {
                    let _ = writeln!(&mut файл, "    pop rax");
                }
            }
            ВидИнструкции::СохранитьКадр => {
                let _ = writeln!(&mut файл, "    push rbp");
                let _ = writeln!(&mut файл, "    mov rbp, rsp");
            }
            ВидИнструкции::ВосстановитьКадр => {
                let _ = writeln!(&mut файл, "    pop rbp");
            }
            ВидИнструкции::ПрочитатьКадр(смещение) => {
                let смещение = -(смещение + 1)*8;
                if смещение < 0 {
                    let _ = writeln!(&mut файл, "    mov rax, [rbp{смещение}]");
                } else {
                    let _ = writeln!(&mut файл, "    mov rax, [rbp+{смещение}]");
                }
                let _ = writeln!(&mut файл, "    push rax");
            }
            ВидИнструкции::ЗаписатьКадр(смещение) => {
                let смещение = -(смещение + 1)*8;
                let _ = writeln!(&mut файл, "    pop rax");
                if смещение < 0 {
                    let _ = writeln!(&mut файл, "    mov [rbp{смещение}], rax");
                } else {
                    let _ = writeln!(&mut файл, "    mov [rbp+{смещение}], rax");
                }
            }
            ВидИнструкции::Записать8 => {
                let _ = writeln!(&mut файл, "    pop rsi");
                let _ = writeln!(&mut файл, "    pop rax");
                let _ = writeln!(&mut файл, "    mov [rsi], al");
            }
            ВидИнструкции::Записать64 => {
                let _ = writeln!(&mut файл, "    pop rsi");
                let _ = writeln!(&mut файл, "    pop rax");
                let _ = writeln!(&mut файл, "    mov [rsi], rax");
            }
            ВидИнструкции::Прочитать64 => {
                let _ = writeln!(&mut файл, "    pop rsi");
                let _ = writeln!(&mut файл, "    mov rax, [rsi]");
                let _ = writeln!(&mut файл, "    push rax");
            }
            ВидИнструкции::ЦелСложение => {
                let _ = writeln!(&mut файл, "    pop rbx");
                let _ = writeln!(&mut файл, "    pop rax");
                let _ = writeln!(&mut файл, "    add rax, rbx");
                let _ = writeln!(&mut файл, "    push rax");
            }
            ВидИнструкции::ЦелВычитание => {
                let _ = writeln!(&mut файл, "    pop rbx");
                let _ = writeln!(&mut файл, "    pop rax");
                let _ = writeln!(&mut файл, "    sub rax, rbx");
                let _ = writeln!(&mut файл, "    push rax");
            }
            ВидИнструкции::ЦелУмножение => {
                let _ = writeln!(&mut файл, "    pop rbx");
                let _ = writeln!(&mut файл, "    pop rax");
                let _ = writeln!(&mut файл, "    xor rdx, rdx");
                let _ = writeln!(&mut файл, "    mul rbx");
                let _ = writeln!(&mut файл, "    push rax");
            }
            ВидИнструкции::ЦелДеление => {
                let _ = writeln!(&mut файл, "    pop rbx");
                let _ = writeln!(&mut файл, "    pop rax");
                let _ = writeln!(&mut файл, "    xor rdx, rdx");
                let _ = writeln!(&mut файл, "    div rbx");
                let _ = writeln!(&mut файл, "    push rax");
            }
            ВидИнструкции::ЦелОстаток => {
                let _ = writeln!(&mut файл, "    pop rbx");
                let _ = writeln!(&mut файл, "    pop rax");
                let _ = writeln!(&mut файл, "    xor rdx, rdx");
                let _ = writeln!(&mut файл, "    div rbx");
                let _ = writeln!(&mut файл, "    push rdx");
            }
            ВидИнструкции::ЦелМеньше => {
                let _ = writeln!(&mut файл, "    pop rbx");
                let _ = writeln!(&mut файл, "    pop rax");
                let _ = writeln!(&mut файл, "    xor rcx, rcx");
                let _ = writeln!(&mut файл, "    cmp rax, rbx");
                let _ = writeln!(&mut файл, "    setb cl");
                let _ = writeln!(&mut файл, "    push rcx");
                // СДЕЛАТЬ: можно ли использовать условное
                // перемещение для реализации инструкций сравнения?
            }
            ВидИнструкции::ЦелБольше => {
                let _ = writeln!(&mut файл, "    pop rbx");
                let _ = writeln!(&mut файл, "    pop rax");
                let _ = writeln!(&mut файл, "    xor rcx, rcx");
                let _ = writeln!(&mut файл, "    cmp rax, rbx");
                let _ = writeln!(&mut файл, "    seta cl");
                let _ = writeln!(&mut файл, "    push rcx");
            }
            ВидИнструкции::ЦелРавно => {
                let _ = writeln!(&mut файл, "    pop rbx");
                let _ = writeln!(&mut файл, "    pop rax");
                let _ = writeln!(&mut файл, "    xor rcx, rcx");
                let _ = writeln!(&mut файл, "    cmp rax, rbx");
                let _ = writeln!(&mut файл, "    setz cl");
                let _ = writeln!(&mut файл, "    push rcx");
            }
            ВидИнструкции::ЛогОтрицание => {
                let _ = writeln!(&mut файл, "    xor rbx, rbx");
                let _ = writeln!(&mut файл, "    pop rax");
                let _ = writeln!(&mut файл, "    test rax, rax");
                let _ = writeln!(&mut файл, "    setz bl");
                let _ = writeln!(&mut файл, "    push rbx");
            }
            ВидИнструкции::ПечатьСтроки => {
                let _ = writeln!(&mut файл, "    mov rax, 1 ; SYS_write");
                let _ = writeln!(&mut файл, "    mov rdi, 1 ; stdout");
                let _ = writeln!(&mut файл, "    pop rsi");
                let _ = writeln!(&mut файл, "    pop rdx");
                let _ = writeln!(&mut файл, "    syscall");
            }
            ВидИнструкции::Ввод => {
                let _ = writeln!(&mut файл, "    mov rax, 0 ; SYS_read");
                let _ = writeln!(&mut файл, "    mov rdi, 0 ; stdin");
                let _ = writeln!(&mut файл, "    pop rdx");
                let _ = writeln!(&mut файл, "    pop rsi");
                let _ = writeln!(&mut файл, "    syscall");
                let _ = writeln!(&mut файл, "    push rax");
            }
            ВидИнструкции::Возврат => {
                let _ = writeln!(&mut файл, "    ret");
            }
            ВидИнструкции::ВызватьВнутреннююПроцедуру(индекс_инструкции_пп_цели) => {
                let _ = writeln!(&mut файл, "    call инструкция_{индекс_инструкции_пп_цели}");
            }
            ВидИнструкции::ВызватьВнешнююПроцедуру(_) => {
                сделать!(&инструкция.лок, "вызов внешних процедур в кодогенерации фазм");
                return Err(());
            }
            ВидИнструкции::Прыжок(индекс_инструкции_пп_цели) => {
                let _ = writeln!(&mut файл, "    jmp инструкция_{индекс_инструкции_пп_цели}");
            }
            ВидИнструкции::УсловныйПрыжок(индекс_инструкции_пп_цели) => {
                let _ = writeln!(&mut файл, "    pop rax");
                let _ = writeln!(&mut файл, "    test rax, rax");
                let _ = writeln!(&mut файл, "    jnz инструкция_{индекс_инструкции_пп_цели}");
            }
            ВидИнструкции::СисВызов{..} => {
                let _ = writeln!(&mut файл, "    ;;; СДЕЛАТЬ");
            }
        }
    }

    let _ = writeln!(&mut файл, "данные:");
    let длинна_строки = 10;
    let количество_строк = пп.иниц_данные.len()/10;
    let остаток_строки = пп.иниц_данные.len()%длинна_строки;
    for строка in 0..количество_строк {
        let _ = write!(&mut файл, "    db");
        for столбец in 0..длинна_строки {
            let индекс = строка*длинна_строки + столбец;
            let _ = write!(&mut файл, " {байт:#04X}", байт = пп.иниц_данные[индекс]);
            if столбец + 1 < длинна_строки {
                let _ = write!(&mut файл, ",");
            }
        }
        let _ = writeln!(&mut файл);
    }
    if остаток_строки > 0 {
        let _ = write!(&mut файл, "    db");
        for столбец in 0..остаток_строки {
            let индекс = количество_строк*длинна_строки + столбец;
            let _ = write!(&mut файл, " {байт:#04X}", байт = пп.иниц_данные[индекс]);
            if столбец + 1 < остаток_строки {
                let _ = write!(&mut файл, ",");
            }
        }
        let _ = writeln!(&mut файл);
    }
    let _ = writeln!(&mut файл, "    rb {}", пп.размер_неиниц_данных);

    drop(файл);
    println!("ИНФО: сгенерирован файл «{путь_к_фазму}»",
             путь_к_фазму = путь_к_фазму.display());

    // СДЕЛАТЬ: более умный способ находить бинарник fasm. Возможно имеет смысл держать его прямо в репе. Он довольно маленький.
    Command::new("fasm")
        .arg(&путь_к_фазму)
        .arg(&путь_к_исполняемому)
        .stdout(Stdio::inherit())
        .spawn()
        .map_err(|ошибка| {
            eprintln!("ОШИБКА: не получилось запустить дочерний процесс fasm: {ошибка}");
        })?
        .wait()
        .map_err(|ошибка| {
            eprintln!("ОШИБКА: что-то пошло не так пока мы ждали завершения дочернего процесса fasm: {ошибка}");
        })?;
    println!("ИНФО: сгенерирован файл «{путь_к_исполняемому}»",
             путь_к_исполняемому = путь_к_исполняемому.display());

    #[cfg(all(unix))] {
        use std::os::unix::fs::PermissionsExt;
        let файл = fs::File::open(&путь_к_исполняемому).map_err(|ошибка| {
            eprintln!("ОШИБКА: не получилось открыть файл «{путь_к_исполняемому}»: {ошибка}",
                      путь_к_исполняемому = путь_к_исполняемому.display());
        })?;
        let mut права = файл.metadata().map_err(|ошибка| {
            eprintln!("ОШИБКА: не получилось прочитать метаданные файла «{путь_к_исполняемому}»: {ошибка}",
                      путь_к_исполняемому = путь_к_исполняемому.display());
        })?.permissions();
        права.set_mode(0o755);
        файл.set_permissions(права).map_err(|ошибка| {
            eprintln!("ОШИБКА: не получилось установить права для файла «{путь_к_исполняемому}»: {ошибка}",
                      путь_к_исполняемому = путь_к_исполняемому.display());
        })?;
    }

    Ok(())
}

pub fn сгенерировать_объектный_файл(_путь_к_файлу: &Path) -> Результат<()> {
    todo!("СДЕЛАТЬ: генерация объектных файлов фазм")
}
