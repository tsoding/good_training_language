use std::fs;
use std::env;
use std::io;
use std::process::ExitCode;

#[path="./диагностика.rs"]
#[macro_use]
mod диагностика;
#[path="./лексика.rs"]
mod лексика;
#[path="./синтаксис.rs"]
mod синтаксис;
#[path="./пп.rs"]
mod пп;

use лексика::Лексер;
use синтаксис::разобрать_модуль;
use пп::Программа;

fn главная() -> Option<()> {
    let mut аргы = env::args();
    let программа = аргы.next().expect("программа");
    let путь_к_файлу = if let Some(путь_к_файлу) = аргы.next() {
        путь_к_файлу
    } else {
        eprintln!("Пример: {программа} <файл.хуя>");
        eprintln!("ОШИБКА: требуется файл с программой");
        return None;
    };
    let содержание: Vec<char> = match fs::read_to_string(&путь_к_файлу) {
        Ok(содержание) => содержание.chars().collect(),
        Err(ошибка) => {
            match ошибка.kind() {
                io::ErrorKind::NotFound => eprintln!("ОШИБКА: файл «{путь_к_файлу}» не найден"),
                _ => eprintln!("ОШИБКА: не получилось прочитать файл «{путь_к_файлу}»: {ошибка}"),
            }
            return None;
        }
    };
    let mut лекс = Лексер::новый(&путь_к_файлу, &содержание);
    let модуль = разобрать_модуль(&mut лекс)?;
    let mut программа = Программа::default();
    программа.скомпилировать_модуль(&модуль)?;
    программа.интерпретировать("главная")?;
    Some(())
}

fn main() -> ExitCode {
    match главная() {
        Some(()) => ExitCode::SUCCESS,
        None => ExitCode::FAILURE,
    }
}
