use super::диагностика::*;
use super::Результат;
use std::fmt::Write;

pub const ПРИСТАВКИ_ПРЕПИНАНИЙ: &[(&[char], ВидЛексемы)] = &[
    (&['('], ВидЛексемы::ОткрытаяСкобка),
    (&[')'], ВидЛексемы::ЗакрытаяСкобка),
    (&['['], ВидЛексемы::ОткрытаяКвадСкобка),
    (&[']'], ВидЛексемы::ЗакрытаяКвадСкобка),
    (&[';'], ВидЛексемы::ТочкаЗапятая),
    (&[':', '='], ВидЛексемы::Присваивание),
    (&[':'], ВидЛексемы::Двоеточие),
    (&[','], ВидЛексемы::Запятая),
    (&['+'], ВидЛексемы::Плюс),
    (&['-'], ВидЛексемы::Минус),
    (&['/'], ВидЛексемы::ПрямаяНаклонная),
    (&['%'], ВидЛексемы::Процент),
    (&['=', '='], ВидЛексемы::РавноРавно),
    (&['='], ВидЛексемы::Равно),
];

pub const КЛЮЧЕВЫЕ_СЛОВА: &[(&str, ВидЛексемы)] = &[
    ("пер", ВидЛексемы::КлючПер),
    ("про", ВидЛексемы::КлючПро),
    ("конст", ВидЛексемы::КлючКонст),
    ("если", ВидЛексемы::КлючЕсли),
    ("пока", ВидЛексемы::КлючПока),
    ("вернуть", ВидЛексемы::КлючВернуть),
    ("мн", ВидЛексемы::КлючМн),
    ("бл", ВидЛексемы::КлючБл),
    ("нч", ВидЛексемы::КлючНч),
    ("кц", ВидЛексемы::КлючКц),
    ("как", ВидЛексемы::КлючКак),
    ("вкл", ВидЛексемы::КлючВкл),
];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ВидЛексемы {
    Конец,
    Идент,

    КлючПер,
    КлючПро,
    КлючКонст,
    КлючЕсли,
    КлючПока,
    КлючВернуть,
    КлючМн,
    КлючБл,
    КлючНч,
    КлючКц,
    КлючКак,
    КлючВкл,

    ОткрытаяСкобка,
    ЗакрытаяСкобка,
    ОткрытаяКвадСкобка,
    ЗакрытаяКвадСкобка,
    ТочкаЗапятая,
    Двоеточие,
    Запятая,
    Плюс,
    Минус,
    ПрямаяНаклонная,
    Процент,
    Присваивание,
    Равно,
    РавноРавно,

    Число,
    Строка,
}

impl ВидЛексемы {
    pub fn сущ(&self) -> Сущ {
        match self {
            ВидЛексемы::Конец              => Сущ{текст: "конец ввода",                род: Род::Муж},
            ВидЛексемы::Идент              => Сущ{текст: "идентификатор",              род: Род::Муж},

            // Ключевые слова
            ВидЛексемы::КлючПер            => Сущ{текст: "«пер»",                      род: Род::Сред},
            ВидЛексемы::КлючПро            => Сущ{текст: "«про»",                      род: Род::Сред},
            ВидЛексемы::КлючКонст          => Сущ{текст: "«конст»",                    род: Род::Сред},
            ВидЛексемы::КлючЕсли           => Сущ{текст: "«если»",                     род: Род::Сред},
            ВидЛексемы::КлючПока           => Сущ{текст: "«пока»",                     род: Род::Сред},
            ВидЛексемы::КлючВернуть        => Сущ{текст: "«вернуть»",                  род: Род::Сред},
            ВидЛексемы::КлючМн             => Сущ{текст: "«мн»",                       род: Род::Сред},
            ВидЛексемы::КлючБл             => Сущ{текст: "«бл»",                       род: Род::Сред},
            ВидЛексемы::КлючНч             => Сущ{текст: "«нч»",                       род: Род::Сред},
            ВидЛексемы::КлючКц             => Сущ{текст: "«кц»",                       род: Род::Муж},
            ВидЛексемы::КлючКак            => Сущ{текст: "«как»",                      род: Род::Муж},
            ВидЛексемы::КлючВкл            => Сущ{текст: "«вкл»",                      род: Род::Муж},

            // Знаки препинания
            ВидЛексемы::ОткрытаяСкобка     => Сущ{текст: "открытая скобка",            род: Род::Жен},
            ВидЛексемы::ЗакрытаяСкобка     => Сущ{текст: "закрытая скобка",            род: Род::Жен},
            ВидЛексемы::ОткрытаяКвадСкобка => Сущ{текст: "открытая квадратная скобка", род: Род::Жен},
            ВидЛексемы::ЗакрытаяКвадСкобка => Сущ{текст: "закрытая квадратная скобка", род: Род::Жен},
            ВидЛексемы::ТочкаЗапятая       => Сущ{текст: "точка с запятой",            род: Род::Жен},
            ВидЛексемы::Двоеточие          => Сущ{текст: "двоеточие",                  род: Род::Сред},
            ВидЛексемы::Запятая            => Сущ{текст: "запятая",                    род: Род::Жен},
            ВидЛексемы::Плюс               => Сущ{текст: "плюс",                       род: Род::Муж},
            ВидЛексемы::Минус              => Сущ{текст: "минус",                      род: Род::Муж},
            ВидЛексемы::ПрямаяНаклонная    => Сущ{текст: "прямая наклонная черта",     род: Род::Жен},
            ВидЛексемы::Процент            => Сущ{текст: "процент",                    род: Род::Муж},
            ВидЛексемы::Равно              => Сущ{текст: "равно",                      род: Род::Сред},
            ВидЛексемы::РавноРавно         => Сущ{текст: "двойное равно",              род: Род::Сред},
            ВидЛексемы::Присваивание       => Сущ{текст: "присваивание",               род: Род::Сред},

            ВидЛексемы::Число              => Сущ{текст: "число",                      род: Род::Сред},
            ВидЛексемы::Строка             => Сущ{текст: "строка",                     род: Род::Жен},
        }
    }
}

#[derive(Debug, Clone)]
pub struct Лексема {
    pub вид: ВидЛексемы,
    pub текст: String,
    pub лок: Лок,
}

pub struct Лексер<'a> {
    pub символы: &'a [char],
    pub путь_к_файлу: &'a str,
    pub позиция: usize,
    pub начало_строки: usize,
    pub строка: usize,
    pub буфер: Option<Результат<Лексема>>
}

impl<'a> Лексер<'a> {
    pub fn новый(путь_к_файлу: &'a str, символы: &'a [char]) -> Лексер<'a> {
        Лексер {
            символы,
            путь_к_файлу,
            позиция: 0,
            начало_строки: 0,
            строка: 0,
            буфер: None,
        }
    }

    pub fn текущий_символ(&self) -> Option<&char> {
        self.символы.get(self.позиция)
    }

    pub fn имеет_приставку(&self, приставка: &[char]) -> bool {
        self.символы[self.позиция..].starts_with(приставка)
    }

    pub fn отрезать_символы(&mut self, mut количество: usize) {
        while количество > 0 && self.текущий_символ().is_some() {
            self.отрезать_символ();
            количество -= 1;
        }
    }

    pub fn отрезать_символ(&mut self) {
        if let Some(&символ) = self.текущий_символ() {
            self.позиция += 1;
            if символ == '\n' {
                self.начало_строки = self.позиция;
                self.строка += 1;
            }
        }
    }

    pub fn подбрить_пробелы(&mut self) {
        while self.текущий_символ().map(|сим| сим.is_whitespace()).unwrap_or(false) {
            self.отрезать_символ();
        }
    }

    fn следующая_строка(&mut self) {
        while let Some(x) = self.текущий_символ().cloned() {
            self.отрезать_символ();
            if x == '\n' {
                break;
            }
        }
    }

    fn следующая_лексема(&mut self) -> Результат<Лексема> {
        'подбрить_пробелы_и_комментарии: loop {
            self.подбрить_пробелы();
            // СДЕЛАТЬ: многострочные комментарии в стиле Си /* ... */
            if self.имеет_приставку(&['/', '/']) {
                self.следующая_строка();
            } else {
                break 'подбрить_пробелы_и_комментарии;
            }
        }

        let лок = Лок {
            строка: self.строка + 1,
            столбец: self.позиция - self.начало_строки + 1,
            путь_к_файлу: self.путь_к_файлу.to_string(),
        };

        let сим = if let Some(сим) = self.текущий_символ().cloned() {
            сим
        } else {
            return Ok(Лексема {
                вид: ВидЛексемы::Конец,
                текст: String::new(),
                лок,
            });
        };

        if сим.is_alphabetic() || сим == '_' {
            let начало = self.позиция;
            while self.символы.get(self.позиция).map(|сим| сим.is_alphanumeric() || *сим == '_').unwrap_or(false) {
                self.отрезать_символ();
            }
            let текст = self.символы[начало..self.позиция].iter().collect();
            for &(ключ, вид) in КЛЮЧЕВЫЕ_СЛОВА.iter() {
                if ключ == текст {
                    return Ok(Лексема {вид, текст, лок})
                }
            }
            return Ok(Лексема {
                вид: ВидЛексемы::Идент,
                текст,
                лок,
            })
        }

        if сим.is_numeric() {
            let начало = self.позиция;
            while self.символы.get(self.позиция).map(|сим| сим.is_numeric()).unwrap_or(false) {
                self.отрезать_символ();
            }
            let текст = self.символы[начало..self.позиция].iter().collect();
            return Ok(Лексема {
                вид: ВидЛексемы::Число,
                текст,
                лок,
            })
        }

        if сим == '"' || сим == '«' {
            self.отрезать_символ();
            let mut текст = String::new();
            while let Some(сим) = self.текущий_символ().cloned() {
                match сим {
                    '"' | '»' => break,
                    '\\' => {
                        self.отрезать_символ();
                        match self.текущий_символ().cloned() {
                            Some('н') => {
                                self.отрезать_символ();
                                текст.push('\n');
                            }
                            Some('\\') => {
                                self.отрезать_символ();
                                текст.push('\\');
                            }
                            Some('"') => {
                                self.отрезать_символ();
                                текст.push('"');
                            }
                            // СДЕЛАТЬ: вариант экранирования для «ёлочек»
                            Some('»') => {
                                self.отрезать_символ();
                                текст.push('»');
                            }
                            Some(сим) => {
                                let лок = Лок {
                                    строка: self.строка + 1,
                                    столбец: self.позиция - self.начало_строки + 1,
                                    путь_к_файлу: self.путь_к_файлу.to_string(),
                                };
                                диагностика!(&лок, "ОШИБКА", "неизвестная поседовательность экранирования начинается с «{сим}»");
                                return Err(());
                            }
                            None => {
                                диагностика!(&лок, "ОШИБКА", "незавершённая строка");
                                return Err(());
                            }
                        }
                    }
                    _ => {
                        self.отрезать_символ();
                        текст.push(сим)
                    }
                }
            }
            match self.текущий_символ().cloned() {
                Some('"') | Some('»') => {
                    self.отрезать_символ();
                    return Ok(Лексема {
                        вид: ВидЛексемы::Строка,
                        текст,
                        лок,
                    });
                }
                Some(_) => unreachable!(),
                None => {
                    диагностика!(&лок, "ОШИБКА", "незавершённая строка");
                    return Err(());
                }
            }
        }

        for &(приставка, вид) in ПРИСТАВКИ_ПРЕПИНАНИЙ.iter() {
            if self.имеет_приставку(приставка) {
                self.отрезать_символы(приставка.len());
                let текст = приставка.iter().collect();
                return Ok(Лексема {вид, текст, лок})
            }
        }

        self.отрезать_символ();
        диагностика!(&лок, "ОШИБКА", "Неизвестная лексема начинающаяся с «{сим}»");
        Err(())
    }

    pub fn вытащить_лексему(&mut self) -> Результат<Лексема> {
        if let Some(лексема) = self.буфер.take() {
            лексема
        } else {
            self.следующая_лексема()
        }
    }

    pub fn подсмотреть_лексему(&mut self) -> Результат<Лексема> {
        if self.буфер.is_none() {
            self.буфер = Some(self.следующая_лексема())
        }
        self.буфер.clone().unwrap()
    }

    pub fn вытащить_лексему_вида(&mut self, ожидаемые_виды: &[ВидЛексемы]) -> Результат<Лексема> {
        assert!(ожидаемые_виды.len() > 0);

        let лексема = self.вытащить_лексему()?;
        for ожидаемый_вид in ожидаемые_виды {
            if лексема.вид == *ожидаемый_вид {
                return Ok(лексема)
            }
        }

        if ожидаемые_виды.len() == 1 {
            let ожидаемый_вид = ожидаемые_виды[0];
            диагностика!(&лексема.лок, "ОШИБКА",
                         "{ожидался} {ожидаемый_вид}, но {повстречался} {действительный_вид}",
                         ожидался           = ГЛАГОЛ_ОЖИДАЛСЯ.отобразить(&ожидаемый_вид.сущ().род),
                         ожидаемый_вид      = ожидаемый_вид.сущ().текст,
                         повстречался       = ГЛАГОЛ_ПОВСТРЕЧАЛСЯ.отобразить(&лексема.вид.сущ().род),
                         действительный_вид = лексема.вид.сущ().текст);
        } else {
            let mut список_видов = String::new();
            for (порядок, ожидаемый_вид) in ожидаемые_виды.iter().enumerate() {
                let ожидаемый_вид = ожидаемый_вид.сущ().текст;
                if порядок == 0 {
                    write!(&mut список_видов, "{ожидаемый_вид}").unwrap();
                } else {
                    write!(&mut список_видов, ", либо {ожидаемый_вид}").unwrap();
                }
            }
            диагностика!(&лексема.лок, "ОШИБКА",
                         "ожидались {список_видов}, но {повстречался} {действительный_вид}",
                         повстречался       = ГЛАГОЛ_ПОВСТРЕЧАЛСЯ.отобразить(&лексема.вид.сущ().род),
                         действительный_вид = лексема.вид.сущ().текст);
        }

        Err(())
    }
}
