use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use компилятор::ПП;
use компилятор::ВидИнструкции;
use Результат;

fn сгенерировать_инструкции(файл: &mut impl Write, пп: &ПП, точка_входа_программы: usize) -> Результат<()> {
    let mut внешние_символы: Vec<_> = пп.внешние_символы.iter().collect();
    внешние_символы.sort_by_key(|(_, индекс)| *индекс);
    let _ = writeln!(файл, "    call инструкция_{точка_входа_программы}");
    let _ = writeln!(файл, "    mov rax, 60");
    let _ = writeln!(файл, "    mov rdi, 0");
    let _ = writeln!(файл, "    syscall");
    for (индекс, инструкция) in пп.код.iter().enumerate() {
        let _ = writeln!(файл, "инструкция_{индекс}: ;;; {путь_к_файлу}:{строка}:{столбец}: {вид_инструкции:?}",
                         путь_к_файлу = инструкция.лок.путь_к_файлу.display(),
                         строка = инструкция.лок.строка,
                         столбец = инструкция.лок.столбец,
                         вид_инструкции = инструкция.вид);
        match инструкция.вид {
            ВидИнструкции::Ноп => {}
            ВидИнструкции::ПротолкнутьЦелое(значение) => {
                let _ = writeln!(файл, "    mov rax, {значение}");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::ПротолкнутьУказатель(указатель) => {
                let _ = writeln!(файл, "    mov rax, данные+{указатель}");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::Вытолкнуть(количество) => {
                // СДЕЛАТЬ: можеть быть стоит напрямую модифицировать регистр rsp одной операцией?
                for _ in 0..количество {
                    let _ = writeln!(файл, "    pop rax");
                }
            }
            ВидИнструкции::Обменять => {
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    push rax");
                let _ = writeln!(файл, "    push rbx");
            }
            ВидИнструкции::СохранитьКадр => {
                let _ = writeln!(файл, "    push rbp");
                let _ = writeln!(файл, "    mov rbp, rsp");
            }
            ВидИнструкции::ВосстановитьКадр => {
                let _ = writeln!(файл, "    pop rbp");
            }
            ВидИнструкции::ПрочитатьКадр(смещение) => {
                let смещение = -(смещение + 1)*8;
                if смещение < 0 {
                    let _ = writeln!(файл, "    mov rax, [rbp{смещение}]");
                } else {
                    let _ = writeln!(файл, "    mov rax, [rbp+{смещение}]");
                }
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::ЗаписатьКадр(смещение) => {
                let смещение = -(смещение + 1)*8;
                let _ = writeln!(файл, "    pop rax");
                if смещение < 0 {
                    let _ = writeln!(файл, "    mov [rbp{смещение}], rax");
                } else {
                    let _ = writeln!(файл, "    mov [rbp+{смещение}], rax");
                }
            }
            ВидИнструкции::Записать8 => {
                let _ = writeln!(файл, "    pop rsi");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    mov [rsi], al");
            }
            ВидИнструкции::Записать64 => {
                let _ = writeln!(файл, "    pop rsi");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    mov [rsi], rax");
            }
            ВидИнструкции::Прочитать64 => {
                let _ = writeln!(файл, "    pop rsi");
                let _ = writeln!(файл, "    mov rax, [rsi]");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::ЦелСложение => {
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    add rax, rbx");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::ЦелВычитание => {
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    sub rax, rbx");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::ЦелУмножение => {
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    xor rdx, rdx");
                let _ = writeln!(файл, "    mul rbx");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::ЦелДеление => {
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    xor rdx, rdx");
                let _ = writeln!(файл, "    div rbx");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::ЦелОстаток => {
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    xor rdx, rdx");
                let _ = writeln!(файл, "    div rbx");
                let _ = writeln!(файл, "    push rdx");
            }
            ВидИнструкции::ЦелМеньше => {
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    xor rcx, rcx");
                let _ = writeln!(файл, "    cmp rax, rbx");
                let _ = writeln!(файл, "    setb cl");
                let _ = writeln!(файл, "    push rcx");
                // СДЕЛАТЬ: можно ли использовать условное
                // перемещение для реализации инструкций сравнения?
            }
            ВидИнструкции::ЦелБольше => {
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    xor rcx, rcx");
                let _ = writeln!(файл, "    cmp rax, rbx");
                let _ = writeln!(файл, "    seta cl");
                let _ = writeln!(файл, "    push rcx");
            }
            ВидИнструкции::ЦелРавно => {
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    xor rcx, rcx");
                let _ = writeln!(файл, "    cmp rax, rbx");
                let _ = writeln!(файл, "    setz cl");
                let _ = writeln!(файл, "    push rcx");
            }
            ВидИнструкции::ЛогОтрицание => {
                let _ = writeln!(файл, "    xor rbx, rbx");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    test rax, rax");
                let _ = writeln!(файл, "    setz bl");
                let _ = writeln!(файл, "    push rbx");
            }
            ВидИнструкции::ПечатьСтроки => {
                let _ = writeln!(файл, "    mov rax, 1 ; SYS_write");
                let _ = writeln!(файл, "    mov rdi, 1 ; stdout");
                let _ = writeln!(файл, "    pop rsi");
                let _ = writeln!(файл, "    pop rdx");
                let _ = writeln!(файл, "    syscall");
            }
            ВидИнструкции::Ввод => {
                let _ = writeln!(файл, "    mov rax, 0 ; SYS_read");
                let _ = writeln!(файл, "    mov rdi, 0 ; stdin");
                let _ = writeln!(файл, "    pop rdx");
                let _ = writeln!(файл, "    pop rsi");
                let _ = writeln!(файл, "    syscall");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::Возврат => {
                let _ = writeln!(файл, "    ret");
            }
            ВидИнструкции::ВызватьВнутреннююПроцедуру(индекс_инструкции_пп_цели) => {
                let _ = writeln!(файл, "    call инструкция_{индекс_инструкции_пп_цели}");
            }
            ВидИнструкции::ВызватьВнешнююПроцедуру{индекс, арность, результат} => {
                let регистры = &["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
                if арность > регистры.len() {
                    сделать!(&инструкция.лок, "Слишком большая арность");
                    return Err(())
                }
                for регистр in &регистры[0..арность] {
                    let _ = writeln!(файл, "    pop {регистр}");
                }
                let _ = writeln!(файл, "    call {имя}", имя = внешние_символы[индекс].0);
                if результат {
                    let _ = writeln!(файл, "    push rax");
                }
            }
            ВидИнструкции::Прыжок(индекс_инструкции_пп_цели) => {
                let _ = writeln!(файл, "    jmp инструкция_{индекс_инструкции_пп_цели}");
            }
            ВидИнструкции::УсловныйПрыжок(индекс_инструкции_пп_цели) => {
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    test rax, rax");
                let _ = writeln!(файл, "    jnz инструкция_{индекс_инструкции_пп_цели}");
            }
            ВидИнструкции::СисВызов{..} => {
                let _ = writeln!(файл, "    ;;; СДЕЛАТЬ");
            }
        }
    }
    Ok(())
}

pub fn сгенерировать_исполняемый_файл(путь_к_исполняемому: &Path, пп: &ПП, точка_входа_программы: usize) -> Результат<()> {
    let статический = пп.внешние_символы.len() == 0;

    let путь_к_фазму = путь_к_исполняемому.with_extension("fasm");
    let файл = fs::File::create(&путь_к_фазму).map_err(|ошибка| {
        eprintln!("ОШИБКА: не удалось открыть файл «{путь_к_фазму}»: {ошибка}",
                  путь_к_фазму = путь_к_фазму.display());
    })?;
    let mut файл = io::BufWriter::new(файл);
    if статический {
        let _ = writeln!(&mut файл, "format ELF64 executable");
        let _ = writeln!(&mut файл, "entry _start");
        let _ = writeln!(&mut файл, "_start:");
    } else {
        let _ = writeln!(&mut файл, "format ELF64");
        let _ = writeln!(&mut файл, "section \".text\" executable");
        let _ = writeln!(&mut файл, "public _start");
        for (имя, _) in &пп.внешние_символы {
            let _ = writeln!(&mut файл, "extrn {имя}");
        }
        let _ = writeln!(&mut файл, "_start:");
    }
    сгенерировать_инструкции(&mut файл, пп, точка_входа_программы)?;

    if !статический {
        let _ = writeln!(&mut файл, "section \".data\" writable");
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

    let _ = writeln!(&mut файл, "section \".note.GNU-stack\"");

    drop(файл);
    println!("ИНФО: сгенерирован файл «{путь_к_фазму}»",
             путь_к_фазму = путь_к_фазму.display());

    // СДЕЛАТЬ: более умный способ находить бинарник fasm и ld. Возможно имеет смысл держать их прямо в репе.
    if статический {
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
    } else {
        let путь_к_объектнику = путь_к_исполняемому.with_extension("o");

        Command::new("fasm")
            .arg(&путь_к_фазму)
            .arg(&путь_к_объектнику)
            .stdout(Stdio::inherit())
            .spawn()
            .map_err(|ошибка| {
                eprintln!("ОШИБКА: не получилось запустить дочерний процесс fasm: {ошибка}");
            })?
            .wait()
            .map_err(|ошибка| {
                eprintln!("ОШИБКА: что-то пошло не так пока мы ждали завершения дочернего процесса fasm: {ошибка}");
            })?;

        println!("ИНФО: сгенерирован файл «{путь_к_объектнику}»",
                 путь_к_объектнику = путь_к_объектнику.display());

        let mut кмд = Command::new("ld");
        кмд
            .arg("-o").arg(&путь_к_исполняемому)
            .arg(путь_к_объектнику)
            // СДЕЛАТЬ: расхардкодить -dynamic-linker
            .arg("-dynamic-linker").arg("/lib64/ld-linux-x86-64.so.2")
            // СДЕЛАТЬ: расхардкодить пусть к линкуемым библиотекам
            .arg("-L./модули/");
        let mut библиотеки: Vec<_> = пп.библиотеки.iter().collect();
        библиотеки.sort_by_key(|(_, индекс)| *индекс);
        for (имя, _) in &библиотеки {
            кмд.arg(format!("-l{имя}"));
        }
        кмд.stdout(Stdio::inherit())
            .spawn()
            .map_err(|ошибка| {
                eprintln!("ОШИБКА: не получилось запустить дочерний процесс ld: {ошибка}");
            })?
            .wait()
            .map_err(|ошибка| {
                eprintln!("ОШИБКА: что-то пошло не так пока мы ждали завершения дочернего процесса ld: {ошибка}");
            })?;

        println!("ИНФО: сгенерирован файл «{путь_к_исполняемому}»",
                 путь_к_исполняемому = путь_к_исполняемому.display());
    }

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
