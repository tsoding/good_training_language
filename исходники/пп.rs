/// Промежуточное Представление

use super::Результат;
use std::collections::HashMap;
use синтаксис::*;
use диагностика::*;
use лексика::*;

/// Инструкция промежуточного представления
#[derive(Debug)]
pub enum Инструкция {
    Ноп,
    /// Протолкнуть целое значение на стек аргументов.
    ПротолкнутьЦелое(usize),
    /// Протолкнуть указатель на данные.
    ///
    /// Эта инстуркция нужна потому, что мы не знаем во время
    /// компиляции где начинаются данные. Мы это только знаем во время
    /// интерпретации, либо генерации машинного кода.
    ПротолкнутьУказатель(usize), // СДЕЛАТЬ: по возможности, использовать u64 вместо usize для значений пп
    Записать64,
    Прочитать64,
    ЦелСложение,
    ЦелМеньше,
    ЛогОтрицание,
    ПечатьСтроки,
    ПечатьЦелого,
    ПечатьЛогического,
    Возврат,
    Прыжок(usize),
    УсловныйПрыжок(usize),
}

#[derive(Clone)]
pub struct СкомпПеременная {
    pub синтаксис: Переменная,
    pub адрес: usize,
}

#[derive(Debug)]
pub struct СкомпПроцедура {
    pub синтаксис: Процедура,
    pub точка_входа: usize,
}

#[derive(Debug)]
pub struct СкомпКонстанта {
    pub синтаксис: Константа,
    pub значение: usize,
}

#[derive(Default)]
pub struct Программа {
    pub код: Vec<Инструкция>,
    pub данные: Vec<u8>,
    pub значения_констант: HashMap<String, СкомпКонстанта>,
    pub скомпилированные_процедуры: HashMap<String, СкомпПроцедура>,
    pub скомпилированные_переменные: HashMap<String, СкомпПеременная>,
}

impl Программа {
    fn скомпилировать_выражение(&mut self, выражение: &Выражение) -> Результат<Тип> {
        match выражение {
            Выражение::Число(_, число) => {
                self.код.push(Инструкция::ПротолкнутьЦелое(*число));
                Ok(Тип::Цел)
            },
            Выражение::Строка(строка) => {
                let указатель = self.данные.len();
                let длинна = строка.текст.len();
                self.данные.extend(строка.текст.as_bytes());
                self.код.push(Инструкция::ПротолкнутьЦелое(длинна));
                self.код.push(Инструкция::ПротолкнутьУказатель(указатель));
                Ok(Тип::Строка)
            }
            Выражение::Идент(лексема) => {
                if let Some(константа) = self.значения_констант.get(&лексема.текст) {
                    self.код.push(Инструкция::ПротолкнутьЦелое(константа.значение));
                    return Ok(Тип::Цел);
                }
                if let Some(переменная) = self.скомпилированные_переменные.get(&лексема.текст) {
                    self.код.push(Инструкция::ПротолкнутьУказатель(переменная.адрес));
                    match переменная.синтаксис.тип {
                        Тип::Цел => {
                            self.код.push(Инструкция::Прочитать64);
                            return Ok(Тип::Цел);
                        }
                        Тип::Лог => {
                            сделать!(&лексема.лок, "чтение логических переменных");
                            return Err(())
                        }
                        Тип::Строка => {
                            сделать!(&лексема.лок, "чтение строковых переменных");
                            return Err(())
                        }
                    }
                }
                диагностика!(&лексема.лок, "ОШИБКА",
                             "не существует ни констант, ни переменных с имением «{имя}»",
                             имя = &лексема.текст);
                Err(())
            }
            Выражение::Биноп {ключ: _, вид, левое, правое} => {
                let левый_тип = self.скомпилировать_выражение(&левое)?;
                let правый_тип = self.скомпилировать_выражение(&правое)?;
                match вид {
                    ВидБинопа::Меньше => {
                        проверить_типы(левое.лок(), &Тип::Цел, &левый_тип)?;
                        проверить_типы(правое.лок(), &Тип::Цел, &правый_тип)?;
                        self.код.push(Инструкция::ЦелМеньше);
                        Ok(Тип::Лог)
                    }
                    ВидБинопа::Сложение => {
                        проверить_типы(левое.лок(), &Тип::Цел, &левый_тип)?;
                        проверить_типы(правое.лок(), &Тип::Цел, &правый_тип)?;
                        self.код.push(Инструкция::ЦелСложение);
                        Ok(Тип::Цел)
                    }
                }
            }
        }
    }

    fn скомпилировать_утвержление(&mut self, утверждение: &Утверждение) -> Результат<()> {
        match утверждение {
            Утверждение::Присваивание{имя, значение, ..} => {
                // ЗАМЕЧАНИЕ: Причина по которой мы клонируем (cloned)
                // скомпилированную переменную заключается в том, что
                // последующие вызовы «self.скомпилировать_выражение()»
                // потенциально могут модифицировать хeш-таблицу
                // «self.скомпилированные_переменные» тем самым
                // инвалидируя указатель на «переменная». Поэтому мы
                // храним её компию на стеке.
                if let Some(переменная) = self.скомпилированные_переменные.get(имя.текст.as_str()).cloned() {
                    let тип = self.скомпилировать_выражение(&значение)?;
                    проверить_типы(&значение.лок(), &переменная.синтаксис.тип, &тип)?;
                    self.код.push(Инструкция::ПротолкнутьУказатель(переменная.адрес));
                    self.код.push(Инструкция::Записать64);
                    Ok(())
                } else {
                    диагностика!(&имя.лок, "ОШИБКА", "Неизвестная переменная «{имя}»", имя = имя.текст);
                    return Err(())
                }
            },
            Утверждение::Вызов{имя, аргументы} => {
                match имя.текст.as_str() {
                    // СДЕЛАТЬ: не позволять переопределять процедуру «печать» в пользовательском коде.
                    "печать" => {
                        for арг in аргументы {
                            let тип = self.скомпилировать_выражение(&арг)?;
                            match тип {
                                Тип::Строка => self.код.push(Инструкция::ПечатьСтроки),
                                Тип::Цел => self.код.push(Инструкция::ПечатьЦелого),
                                Тип::Лог => self.код.push(Инструкция::ПечатьЛогического),
                            }
                        }
                        Ok(())
                    },
                    _ => {
                        if let Some(процедура) = self.скомпилированные_процедуры.get(&имя.текст) {
                            let количество_аргументов = аргументы.len();
                            let количество_параметров = процедура.синтаксис.параметры.len();
                            if количество_аргументов != количество_параметров {
                                диагностика!(&имя.лок, "ОШИБКА",
                                             "Неверное количество аргументов вызова процедуры. Процедура принимает {количество_параметров} {параметров}, но в данном вызове предоставлено лишь {количество_аргументов} {аргументов}.",
                                             параметров = ЧИСУЩ_ПАРАМЕТР.текст(количество_параметров),
                                             аргументов = ЧИСУЩ_АРГУМЕНТ.текст(количество_аргументов));
                                return Err(());
                            }
                            сделать!(&имя.лок, "Компиляция вызова процедуры");
                            Err(())
                        } else {
                            диагностика!(&имя.лок, "ОШИБКА", "Неизвестная процедура «{имя}»", имя = имя.текст);
                            Err(())
                        }
                    }
                }
            }
            Утверждение::Пока{ключ: _, условие, тело} => {
                let точка_условия = self.код.len();
                let тип = self.скомпилировать_выражение(&условие)?;
                проверить_типы(&условие.лок(), &Тип::Лог, &тип)?;
                self.код.push(Инструкция::ЛогОтрицание);
                let точка_условного_прыжка = self.код.len();
                self.код.push(Инструкция::Ноп);
                for утверждение in тело.iter() {
                    self.скомпилировать_утвержление(утверждение)?;
                }
                self.код.push(Инструкция::Прыжок(точка_условия));
                let точка_выхода = self.код.len();
                self.код[точка_условного_прыжка] = Инструкция::УсловныйПрыжок(точка_выхода);
                Ok(())
            }
        }
    }

    fn скомпилировать_процедуру(&mut self, процедура: Процедура) -> Результат<СкомпПроцедура> {
        let точка_входа = self.код.len();
        for утверждение in &процедура.тело {
            self.скомпилировать_утвержление(утверждение)?;
        }
        self.код.push(Инструкция::Возврат);
        Ok(СкомпПроцедура{синтаксис: процедура, точка_входа})
    }

    fn интерпретировать_выражение_константы(&mut self, выражение: &Выражение) -> Результат<usize> {
        match выражение {
            &Выражение::Число(_, число) => Ok(число),
            Выражение::Строка(строка) => {
                сделать!(&строка.лок, "строковые константы");
                Err(())
            }
            Выражение::Идент(имя) => {
                if let Some(константа) = self.значения_констант.get(имя.текст.as_str()) {
                    Ok(константа.значение)
                } else {
                    диагностика!(&имя.лок, "ОШИБКА", "Неизвестная константа «{имя}»", имя = имя.текст);
                    Err(())
                }
            }
            Выражение::Биноп{ключ, вид, левое, правое, ..} => {
                let левое_значение = self.интерпретировать_выражение_константы(левое)?;
                let правое_значение = self.интерпретировать_выражение_константы(правое)?;
                match вид {
                    ВидБинопа::Меньше => {
                        сделать!(&ключ.лок, "булевые константы");
                        Err(())
                    },
                    ВидБинопа::Сложение => {
                        Ok(левое_значение + правое_значение)
                    }
                }
            }
        }
    }

    fn верифицировать_переопределение_имени(&self, имя: &Лексема) -> Результат<()> {
        if let Some(существующая_переменная) = self.скомпилированные_переменные.get(&имя.текст) {
            диагностика!(&имя.лок, "ОШИБКА",
                         "уже существует переменная с именем «{имя}»",
                         имя = имя.текст);
            диагностика!(&существующая_переменная.синтаксис.имя.лок, "ИНФО",
                         "она определена здесь здесь. Выберите другое имя.");
            return Err(())
        }

        if let Some(существующая_процедура) = self.скомпилированные_процедуры.get(&имя.текст) {
            диагностика!(&имя.лок, "ОШИБКА",
                         "уже существует процедура с именем «{имя}»",
                         имя = имя.текст);
            диагностика!(&существующая_процедура.синтаксис.имя.лок, "ИНФО",
                         "она определена здесь здесь. Выберите другое имя.");
            return Err(())
        }

        if let Some(существующая_константа) = self.значения_констант.get(&имя.текст) {
            диагностика!(&имя.лок, "ОШИБКА",
                         "уже существует константа с именем «{имя}»",
                         имя = имя.текст);
            диагностика!(&существующая_константа.синтаксис.имя.лок, "ИНФО",
                         "она определена здесь здесь. Выберите другое имя.");
            return Err(())
        }

        Ok(())
    }

    pub fn скомпилировать_модуль(&mut self, лекс: &mut Лексер) -> Результат<()> {
        loop {
            let ключ = лекс.вытащить_лексему_вида(&[
                ВидЛексемы::КлючПер,
                ВидЛексемы::КлючПро,
                ВидЛексемы::КлючКонст,
                ВидЛексемы::Конец,
            ])?;
            match ключ.вид {
                ВидЛексемы::КлючПер => {
                    let синтаксис = Переменная::разобрать(лекс)?;
                    self.верифицировать_переопределение_имени(&синтаксис.имя)?;
                    let адрес = self.данные.len();
                    let новый_размер = self.данные.len() + синтаксис.тип.размер();
                    self.данные.resize(новый_размер, 0u8);
                    if let Some(_) = self.скомпилированные_переменные.insert(синтаксис.имя.текст.clone(), СкомпПеременная {адрес, синтаксис}) {
                        unreachable!("Проверка переопределения переменных должна происходить на этапе разбора")
                    }
                }
                ВидЛексемы::КлючПро => {
                    let процедура = Процедура::разобрать(лекс)?;
                    self.верифицировать_переопределение_имени(&процедура.имя)?;
                    let скомп_процедура = self.скомпилировать_процедуру(процедура)?;
                    if let Some(_) = self.скомпилированные_процедуры.insert(скомп_процедура.синтаксис.имя.текст.clone(), скомп_процедура) {
                        unreachable!("Проверка переопределения процедур должна происходить на этапе разбора")
                    }
                }
                ВидЛексемы::КлючКонст => {
                    let константа = Константа::разобрать(лекс)?;
                    self.верифицировать_переопределение_имени(&константа.имя)?;
                    let значение = self.интерпретировать_выражение_константы(&константа.выражение)?;
                    if let Some(_) = self.значения_констант.insert(константа.имя.текст.clone(), СкомпКонстанта { синтаксис: константа, значение }) {
                        unreachable!("Проверка переопределения констант должна происходить на этапе разбора")
                    }
                }
                ВидЛексемы::Конец => return Ok(()),
                _ => unreachable!(),
            }
        }
    }

    pub fn вывалить(&self) {
        let ширина_столбца_индекса = self.код.len().to_string().len();
        for (индекс, инструкция) in self.код.iter().enumerate() {
            println!("{индекс:0>ширина_столбца_индекса$}: {инструкция:?}")
        }
        // СДЕЛАТЬ: так же вывалить данные
    }
}
