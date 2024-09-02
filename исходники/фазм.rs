use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use компилятор::ПП;
use компилятор::ВидИнструкции;
use типизация::*;
use Результат;

fn сгенерировать_инструкции(файл: &mut impl Write, пп: &ПП, точка_входа_программы: usize) -> Результат<()> {
    let mut внешние_символы: Vec<_> = пп.внешние_символы.iter().collect();
    внешние_символы.sort_by_key(|(_, индекс)| *индекс);
    // https://stackoverflow.com/questions/18024672/what-registers-are-preserved-through-a-linux-x86-64-function-call
    let _ = writeln!(файл, "    mov r12, начало_второго_стека");
    let _ = writeln!(файл, "    mov r13, начало_второго_стека");
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
        match &инструкция.вид {
            ВидИнструкции::Ноп => {}
            ВидИнструкции::Вытолкнуть => {
                let _ = writeln!(файл, "    pop rax");
            }
            ВидИнструкции::Продублировать => {
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    push rax");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::Натуральное(значение) => {
                let _ = writeln!(файл, "    mov rax, {значение}");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::Целое(значение) => {
                let _ = writeln!(файл, "    mov rax, {значение}");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::ГлобальныеДанные(смещение) => {
                if *смещение < 0 {
                    let _ = writeln!(файл, "    mov rax, данные{смещение}");
                } else {
                    let _ = writeln!(файл, "    mov rax, данные+{смещение}");
                }
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::ВыделитьНаСтеке(размер) => {
                let _ = writeln!(файл, "    sub r12, {размер}");
            }
            ВидИнструкции::ОсвободитьСоСтека(размер) => {
                let _ = writeln!(файл, "    add r12, {размер}");
            }
            ВидИнструкции::ВершинаСтека(смещение) => {
                let _ = writeln!(файл, "    mov rax, r12");
                let _ = writeln!(файл, "    add rax, {смещение}");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::СохранитьКадр => {
                let _ = writeln!(файл, "    mov rax, r13");
                let _ = writeln!(файл, "    mov r13, r12");
                let _ = writeln!(файл, "    sub r12, 8");
                let _ = writeln!(файл, "    mov [r12], rax");
            }
            ВидИнструкции::ВосстановитьКадр => {
                let _ = writeln!(файл, "    mov r13, [r12]");
                let _ = writeln!(файл, "    add r12, 8");
            }
            ВидИнструкции::Кадр(смещение) => {
                let _ = writeln!(файл, "    mov rax, r13");
                let _ = writeln!(файл, "    add rax, {смещение}");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::АргументНаСтек => {
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    sub r12, 8");
                let _ = writeln!(файл, "    mov [r12], rax");
            }
            ВидИнструкции::АргументСоСтека => {
                let _ = writeln!(файл, "    mov rax, [r12]");
                let _ = writeln!(файл, "    push rax");
                let _ = writeln!(файл, "    add r12, 8");
            }
            ВидИнструкции::Записать8 => {
                let _ = writeln!(файл, "    pop rsi");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    mov [rsi], al");
            }
            ВидИнструкции::Записать16 => {
                сделать!(&инструкция.лок, "Кодогенерация инструкции Записать16");
                return Err(())
            }
            ВидИнструкции::Записать32 => {
                let _ = writeln!(файл, "    pop rsi");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    mov [rsi], eax");
            }
            ВидИнструкции::Записать64 => {
                let _ = writeln!(файл, "    pop rsi");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    mov [rsi], rax");
            }
            ВидИнструкции::ПрочитатьБезЗнак8 => {
                let _ = writeln!(файл, "    pop rsi");
                let _ = writeln!(файл, "    xor rax, rax");
                let _ = writeln!(файл, "    mov al, [rsi]");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::ПрочитатьБезЗнак16 => {
                сделать!(&инструкция.лок, "Кодогенерация инструкции ПрочитатьБезЗнак16");
                return Err(())
            }
            ВидИнструкции::ПрочитатьБезЗнак32 => {
                let _ = writeln!(файл, "    pop rsi");
                let _ = writeln!(файл, "    xor rax, rax");
                let _ = writeln!(файл, "    mov eax, [rsi]");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::ПрочитатьЗнак8 => {
                сделать!(&инструкция.лок, "Кодогенерация инструкции ПрочитатьЗнак8");
                return Err(())
            }
            ВидИнструкции::ПрочитатьЗнак16 => {
                сделать!(&инструкция.лок, "Кодогенерация инструкции ПрочитатьЗнак16");
                return Err(())
            }
            ВидИнструкции::ПрочитатьЗнак32 => {
                сделать!(&инструкция.лок, "Кодогенерация инструкции ПрочитатьЗнак32");
                return Err(())
            }
            ВидИнструкции::Прочитать64 => {
                let _ = writeln!(файл, "    pop rsi");
                let _ = writeln!(файл, "    mov rax, [rsi]");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::СкопироватьПамять => {
                let _ = writeln!(файл, "    cld");
                let _ = writeln!(файл, "    pop rcx");
                let _ = writeln!(файл, "    pop rdi");
                let _ = writeln!(файл, "    pop rsi");
                let _ = writeln!(файл, "    rep movsb");
            }
            ВидИнструкции::ПамятьРавно => {
                let _ = writeln!(файл, "    xor rax, rax");
                let _ = writeln!(файл, "    cld");
                let _ = writeln!(файл, "    pop rcx");
                let _ = writeln!(файл, "    pop rdi");
                let _ = writeln!(файл, "    pop rsi");
                let _ = writeln!(файл, "    rep cmpsb");
                let _ = writeln!(файл, "    setz al");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::НатСложение => {
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    add rax, rbx");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::НатВычитание => {
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    sub rax, rbx");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::НатУмножение => {
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    xor rdx, rdx");
                let _ = writeln!(файл, "    mul rbx");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::НатДеление => {
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    xor rdx, rdx");
                let _ = writeln!(файл, "    div rbx");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::НатОстаток => {
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    xor rdx, rdx");
                let _ = writeln!(файл, "    div rbx");
                let _ = writeln!(файл, "    push rdx");
            }
            ВидИнструкции::НатМеньше => {
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    xor rcx, rcx");
                let _ = writeln!(файл, "    cmp rax, rbx");
                let _ = writeln!(файл, "    setb cl");
                let _ = writeln!(файл, "    push rcx");
                // СДЕЛАТЬ: можно ли использовать условное
                // перемещение для реализации инструкций сравнения?
            }
            ВидИнструкции::НатМеньшеРавно => {
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    xor rcx, rcx");
                let _ = writeln!(файл, "    cmp rax, rbx");
                let _ = writeln!(файл, "    setbe cl");
                let _ = writeln!(файл, "    push rcx");
            }
            ВидИнструкции::НатБольше => {
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    xor rcx, rcx");
                let _ = writeln!(файл, "    cmp rax, rbx");
                let _ = writeln!(файл, "    seta cl");
                let _ = writeln!(файл, "    push rcx");
            }
            ВидИнструкции::НатБольшеРавно => {
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    xor rcx, rcx");
                let _ = writeln!(файл, "    cmp rax, rbx");
                let _ = writeln!(файл, "    setae cl");
                let _ = writeln!(файл, "    push rcx");
            }
            ВидИнструкции::НатРавно => {
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    xor rcx, rcx");
                let _ = writeln!(файл, "    cmp rax, rbx");
                let _ = writeln!(файл, "    setz cl");
                let _ = writeln!(файл, "    push rcx");
            }
            ВидИнструкции::ЦелМеньше => {
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    xor rcx, rcx");
                let _ = writeln!(файл, "    cmp rax, rbx");
                let _ = writeln!(файл, "    setl cl");
                let _ = writeln!(файл, "    push rcx");
            }
            ВидИнструкции::ЦелМеньшеРавно => {
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    xor rcx, rcx");
                let _ = writeln!(файл, "    cmp rax, rbx");
                let _ = writeln!(файл, "    setle cl");
                let _ = writeln!(файл, "    push rcx");
            }
            ВидИнструкции::ЦелБольше => {
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    xor rcx, rcx");
                let _ = writeln!(файл, "    cmp rax, rbx");
                let _ = writeln!(файл, "    setg cl");
                let _ = writeln!(файл, "    push rcx");
            }
            ВидИнструкции::ЦелБольшеРавно => {
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    xor rcx, rcx");
                let _ = writeln!(файл, "    cmp rax, rbx");
                let _ = writeln!(файл, "    setge cl");
                let _ = writeln!(файл, "    push rcx");
            }
            ВидИнструкции::ЦелУмножение => {
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    xor rdx, rdx");
                let _ = writeln!(файл, "    imul rbx");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::ЦелДеление => {
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    cqo");
                let _ = writeln!(файл, "    idiv rbx");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::ЦелОстаток => {
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    cqo");
                let _ = writeln!(файл, "    idiv rbx");
                let _ = writeln!(файл, "    push rdx");
            }
            ВидИнструкции::ЦелОтриц => {
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    neg rax");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::КонвертНат64Вещ32 => {
                // СДЕЛАТЬ: конвертация натуральных чисел в вещественные (и обратно) использует знаковую конвертацию.
                //
                // В общем, ничего плохого в этом нет до тех пор пока натуральные числе
                // не слишком большие.
                //
                // Для больших натуральных чисел нам нужно использовать трюки, которые
                // используют компиляторы языка Си: делим значение на два, чтобы убрать
                // бит знака, конвертируем споловиненное значение, и умножаем вещественный
                // результат обратно на два. С учётом остатков конечно же.
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    pxor xmm0, xmm0");
                let _ = writeln!(файл, "    cvtsi2ss xmm0, rax");
                let _ = writeln!(файл, "    movd eax, xmm0");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::КонвертЦел64Вещ32 => {
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    pxor xmm0, xmm0");
                let _ = writeln!(файл, "    cvtsi2ss xmm0, rax");
                let _ = writeln!(файл, "    movd eax, xmm0");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::КонвертВещ32Нат64 => {
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    movd xmm0, eax");
                let _ = writeln!(файл, "    cvttss2si rax, xmm0");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::КонвертВещ32Цел64 => {
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    movd xmm0, eax");
                let _ = writeln!(файл, "    cvttss2si rax, xmm0");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::Вещ32Умножение => {
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    movd xmm0, eax");
                let _ = writeln!(файл, "    movd xmm1, ebx");
                let _ = writeln!(файл, "    mulss xmm0, xmm1");
                let _ = writeln!(файл, "    movd eax, xmm0");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::Вещ32Деление => {
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    movd xmm0, eax");
                let _ = writeln!(файл, "    movd xmm1, ebx");
                let _ = writeln!(файл, "    divss xmm0, xmm1");
                let _ = writeln!(файл, "    movd eax, xmm0");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::Вещ32Сложение => {
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    movd xmm0, eax");
                let _ = writeln!(файл, "    movd xmm1, ebx");
                let _ = writeln!(файл, "    addss xmm0, xmm1");
                let _ = writeln!(файл, "    movd eax, xmm0");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::Вещ32Вычитание => {
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    movd xmm0, eax");
                let _ = writeln!(файл, "    movd xmm1, ebx");
                let _ = writeln!(файл, "    subss xmm0, xmm1");
                let _ = writeln!(файл, "    movd eax, xmm0");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::Вещ32Меньше => {
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    movd xmm0, eax");
                let _ = writeln!(файл, "    movd xmm1, ebx");
                let _ = writeln!(файл, "    cmpltss xmm0, xmm1");
                let _ = writeln!(файл, "    movd eax, xmm0");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::Вещ32МеньшеРавно => {
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    movd xmm0, eax");
                let _ = writeln!(файл, "    movd xmm1, ebx");
                let _ = writeln!(файл, "    cmpless xmm0, xmm1");
                let _ = writeln!(файл, "    movd eax, xmm0");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::Вещ32Больше => {
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    movd xmm0, eax");
                let _ = writeln!(файл, "    movd xmm1, ebx");
                let _ = writeln!(файл, "    cmpnless xmm0, xmm1");
                let _ = writeln!(файл, "    movd eax, xmm0");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::Вещ32БольшеРавно => {
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    movd xmm0, eax");
                let _ = writeln!(файл, "    movd xmm1, ebx");
                let _ = writeln!(файл, "    cmpnltss xmm0, xmm1");
                let _ = writeln!(файл, "    movd eax, xmm0");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::Вещ32Отриц => {
                let _ = writeln!(файл, "    mov eax, 0x80000000");
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    movd xmm0, ebx");
                let _ = writeln!(файл, "    movd xmm1, eax");
                let _ = writeln!(файл, "    pxor xmm0, xmm1");
                let _ = writeln!(файл, "    movd eax, xmm0");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::ЛогОтрицание => {
                let _ = writeln!(файл, "    xor rbx, rbx");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    test rax, rax");
                let _ = writeln!(файл, "    setz bl");
                let _ = writeln!(файл, "    push rbx");
            }
            ВидИнструкции::БитИли => {
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    or rax, rbx");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::БитИ => {
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    and rax, rbx");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::БитЛибо => {
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    xor rax, rbx");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::БитСмещениеВлево => {
                let _ = writeln!(файл, "    pop rcx");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    shl rax, cl");
                let _ = writeln!(файл, "    push rax");
            }
            // СДЕЛАТЬ: правое битовое смещение положительных и отрицательных чисел может отличаться
            ВидИнструкции::БитСмещениеВправо => {
                let _ = writeln!(файл, "    pop rcx");
                let _ = writeln!(файл, "    pop rax");
                let _ = writeln!(файл, "    shr rax, cl");
                let _ = writeln!(файл, "    push rax");
            }
            ВидИнструкции::ПечатьСтроки => {
                let _ = writeln!(файл, "    pop rbx");
                let _ = writeln!(файл, "    mov rsi, [rbx+{}]", СРЕЗ_АДРЕС_СМЕЩЕНИЕ);
                let _ = writeln!(файл, "    mov rdx, [rbx+{}]", СРЕЗ_РАЗМЕР_СМЕЩЕНИЕ);
                let _ = writeln!(файл, "    mov rax, 1 ; SYS_write");
                let _ = writeln!(файл, "    mov rdi, 1 ; stdout");
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
            ВидИнструкции::ВнутреннийВызов(индекс_инструкции_пп_цели) => {
                let _ = writeln!(файл, "    call инструкция_{индекс_инструкции_пп_цели}");
            }
            ВидИнструкции::ВнешнийВызов{индекс, арность, результат} => {
                let регистры = &["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
                if *арность > регистры.len() {
                    сделать!(&инструкция.лок, "Слишком большая арность");
                    return Err(())
                }
                for регистр in &регистры[0..*арность] {
                    let _ = writeln!(файл, "    pop {регистр}");
                }
                let _ = writeln!(файл, "    call {имя}", имя = внешние_символы[*индекс].0);
                if let Some(результат) = результат {
                    match результат {
                        Тип::Нат64 | Тип::Цел64 | Тип::Лог => {
                            let _ = writeln!(файл, "    push rax");
                        },
                        Тип::Вещ32 => {
                            let _ = writeln!(файл, "    movd eax, xmm0");
                            let _ = writeln!(файл, "    push rax");
                        }
                        _ => {
                            сделать!(&инструкция.лок, "Кодогенерация возврата типа «{тип}» из внешних процедур",
                                     тип = результат.текст());
                            return Err(())
                        }
                    }
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
                сделать!(&инструкция.лок, "Кодогенерация для инструкции «{вид:?}»", вид = инструкция.вид);
                return Err(())
            }
        }
    }
    Ok(())
}

pub fn сгенерировать_исполняемый_файл(путь_к_исполняемому: &Path, пп: &ПП, оставить_фасм_после: &bool, точка_входа_программы: usize) -> Результат<()> {
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
    let размер_второго_стека = 1_000_000;
    let _ = writeln!(&mut файл, "    rb {}", пп.размер_неиниц_данных + размер_второго_стека);
    let _ = writeln!(&mut файл, "начало_второго_стека:");

    if !статический {
        let _ = writeln!(&mut файл, "section \".note.GNU-stack\"");
    }

    drop(файл);
    println!("ИНФО: сгенерирован fasm файл «{путь_к_фазму}»",
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
    if !оставить_фасм_после {
        удалить_файл(&путь_к_фазму)
            .map_err(|ошибка| {
                eprintln!("ПРЕДУПРЕЖДЕНИЕ: не смог удалить : {ошибка}");
            })?;
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
pub fn удалить_файл(путь: &Path) -> Result<(), std::io::Error>{
    if путь.exists() {
        fs::remove_file(путь)?;
        println!("ИНФО: Удалил файл «{}»", путь.display());
    } else {
        println!("ПРЕДУПРЕЖДЕНИЕ: файл «{}» не существует", путь.display());
    }
    Ok(())
}