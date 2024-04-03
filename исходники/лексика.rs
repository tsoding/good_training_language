use стд::прелюдия::*;
use super::диагностика::*;
use super::Результат;
use std::fmt::Write;
use std::path::Path;

pub const ПРИСТАВКИ_ПРЕПИНАНИЙ: &[(&[char], ВидЛексемы)] = &[
    (&['('], ВидЛексемы::ОткрытаяСкобка),
    (&[')'], ВидЛексемы::ЗакрытаяСкобка),
    (&[';'], ВидЛексемы::ТочкаЗапятая),
    (&['.', '.'], ВидЛексемы::ТочкаТочка),
    (&['.'], ВидЛексемы::Точка),
    (&[':', '='], ВидЛексемы::Присваивание),
    (&[':'], ВидЛексемы::Двоеточие),
    (&[','], ВидЛексемы::Запятая),
    (&['+', '?', '='], ВидЛексемы::БольшеРавно),
    (&['+', '?'], ВидЛексемы::Больше),
    (&['+'], ВидЛексемы::Плюс),
    (&['-', '?', '='], ВидЛексемы::МеньшеРавно),
    (&['-', '?'], ВидЛексемы::Меньше),
    (&['-'], ВидЛексемы::Минус),
    (&['*'], ВидЛексемы::Звёздочка),
    (&['/'], ВидЛексемы::ПрямаяНаклонная),
    (&['='], ВидЛексемы::Равно),
    (&['!', '='], ВидЛексемы::НеРавно),
    (&['!'], ВидЛексемы::Не),
];

pub const КЛЮЧЕВЫЕ_СЛОВА: &[(&str, ВидЛексемы)] = &[
    ("пер", ВидЛексемы::КлючПер),
    ("про", ВидЛексемы::КлючПро),
    ("конст", ВидЛексемы::КлючКонст),
    ("если", ВидЛексемы::КлючЕсли),
    ("то", ВидЛексемы::КлючТо),
    ("иначе", ВидЛексемы::КлючИначе),
    ("пока", ВидЛексемы::КлючПока),
    ("для", ВидЛексемы::КлючДля),
    ("вернуть", ВидЛексемы::КлючВернуть),
    ("или", ВидЛексемы::КлючИли),
    ("и", ВидЛексемы::КлючИ),
    ("либо", ВидЛексемы::КлючЛибо),
    ("нч", ВидЛексемы::КлючНч),
    ("кц", ВидЛексемы::КлючКц),
    ("как", ВидЛексемы::КлючКак),
    ("вкл", ВидЛексемы::КлючВкл),
    ("внешняя", ВидЛексемы::КлючВнешняя),
    ("библ", ВидЛексемы::КлючБибл),
    ("структ", ВидЛексемы::КлючСтрукт),
    ("истина", ВидЛексемы::КлючИстина),
    ("ложь", ВидЛексемы::КлючЛожь),
    ("лбс", ВидЛексемы::КлючЛбс),
    ("пбс", ВидЛексемы::КлючПбс),
    ("ост", ВидЛексемы::КлючОст),
    ("вилка", ВидЛексемы::КлючВилка),
    ("когда", ВидЛексемы::КлючКогда),
    ("любое", ВидЛексемы::КлючЛюбое),
    // СДЕЛАТЬ: оператор «мод».
];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ВидЛексемы {
    Конец,
    Идент,

    КлючПер,
    КлючПро,
    КлючКонст,
    КлючЕсли,
    КлючТо,
    КлючИначе,
    КлючПока,
    КлючДля,
    КлючВернуть,
    КлючНч,
    КлючИли,
    КлючИ,
    КлючЛибо,
    КлючКц,
    КлючКак,
    КлючВкл,
    КлючВнешняя,
    КлючБибл,
    КлючСтрукт,
    КлючИстина,
    КлючЛожь,
    КлючЛбс,
    КлючПбс,
    КлючОст,
    КлючВилка,
    КлючКогда,
    КлючЛюбое,

    ОткрытаяСкобка,
    ЗакрытаяСкобка,
    ТочкаЗапятая,
    Точка,
    ТочкаТочка,
    Двоеточие,
    Запятая,
    Плюс,
    Минус,
    Звёздочка,
    ПрямаяНаклонная,
    Присваивание,
    Равно,
    Меньше,
    МеньшеРавно,
    Больше,
    БольшеРавно,
    Не,
    НеРавно,

    ЦелЧисло,
    ЦелШестЧисло,
    ВещЧисло,
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
            ВидЛексемы::КлючТо             => Сущ{текст: "«то»",                       род: Род::Сред},
            ВидЛексемы::КлючИначе          => Сущ{текст: "«иначе»",                    род: Род::Сред},
            ВидЛексемы::КлючПока           => Сущ{текст: "«пока»",                     род: Род::Сред},
            ВидЛексемы::КлючДля            => Сущ{текст: "«для»",                      род: Род::Сред},
            ВидЛексемы::КлючВернуть        => Сущ{текст: "«вернуть»",                  род: Род::Сред},
            ВидЛексемы::КлючЛибо           => Сущ{текст: "«либо»",                     род: Род::Сред},
            ВидЛексемы::КлючИли            => Сущ{текст: "«или»",                      род: Род::Муж},
            ВидЛексемы::КлючИ              => Сущ{текст: "«и»",                        род: Род::Муж},
            ВидЛексемы::КлючНч             => Сущ{текст: "«нч»",                       род: Род::Сред},
            ВидЛексемы::КлючКц             => Сущ{текст: "«кц»",                       род: Род::Муж},
            ВидЛексемы::КлючКак            => Сущ{текст: "«как»",                      род: Род::Муж},
            ВидЛексемы::КлючВкл            => Сущ{текст: "«вкл»",                      род: Род::Муж},
            ВидЛексемы::КлючВнешняя        => Сущ{текст: "«внешняя»",                  род: Род::Жен},
            ВидЛексемы::КлючБибл           => Сущ{текст: "«библ»",                     род: Род::Муж},
            ВидЛексемы::КлючСтрукт         => Сущ{текст: "«структ»",                   род: Род::Муж},
            ВидЛексемы::КлючИстина         => Сущ{текст: "«истина»",                   род: Род::Жен},
            ВидЛексемы::КлючЛожь           => Сущ{текст: "«ложь»",                     род: Род::Жен},
            ВидЛексемы::КлючЛбс            => Сущ{текст: "«лбс»",                      род: Род::Муж},
            ВидЛексемы::КлючПбс            => Сущ{текст: "«пбс»",                      род: Род::Муж},
            ВидЛексемы::КлючОст            => Сущ{текст: "«ост»",                      род: Род::Муж},
            ВидЛексемы::КлючВилка          => Сущ{текст: "«вилка»",                    род: Род::Жен},
            ВидЛексемы::КлючКогда          => Сущ{текст: "«когда»",                    род: Род::Сред},
            ВидЛексемы::КлючЛюбое          => Сущ{текст: "«любое»",                    род: Род::Сред},

            // Знаки препинания
            ВидЛексемы::ОткрытаяСкобка     => Сущ{текст: "открытая скобка",            род: Род::Жен},
            ВидЛексемы::ЗакрытаяСкобка     => Сущ{текст: "закрытая скобка",            род: Род::Жен},
            ВидЛексемы::ТочкаЗапятая       => Сущ{текст: "точка с запятой",            род: Род::Жен},
            ВидЛексемы::Точка              => Сущ{текст: "точка",                      род: Род::Жен},
            ВидЛексемы::ТочкаТочка         => Сущ{текст: "точка точка",                род: Род::Жен},
            ВидЛексемы::Двоеточие          => Сущ{текст: "двоеточие",                  род: Род::Сред},
            ВидЛексемы::Запятая            => Сущ{текст: "запятая",                    род: Род::Жен},
            ВидЛексемы::Плюс               => Сущ{текст: "плюс",                       род: Род::Муж},
            ВидЛексемы::Минус              => Сущ{текст: "минус",                      род: Род::Муж},
            ВидЛексемы::Звёздочка          => Сущ{текст: "звёздочка",                  род: Род::Жен},
            ВидЛексемы::ПрямаяНаклонная    => Сущ{текст: "прямая наклонная черта",     род: Род::Жен},
            ВидЛексемы::Присваивание       => Сущ{текст: "присваивание",               род: Род::Сред},
            ВидЛексемы::Равно              => Сущ{текст: "равно",                      род: Род::Сред},
            ВидЛексемы::Меньше             => Сущ{текст: "меньше",                     род: Род::Сред},
            ВидЛексемы::Больше             => Сущ{текст: "больше",                     род: Род::Сред},
            ВидЛексемы::МеньшеРавно        => Сущ{текст: "меньше либо равно",          род: Род::Сред},
            ВидЛексемы::БольшеРавно        => Сущ{текст: "больше либо равно",          род: Род::Сред},
            ВидЛексемы::Не                 => Сущ{текст: "отрицание",                  род: Род::Сред},
            ВидЛексемы::НеРавно            => Сущ{текст: "не равно",                   род: Род::Сред},

            ВидЛексемы::ЦелШестЧисло       => Сущ{текст: "целое шестнадцатеричное число", род: Род::Сред},
            ВидЛексемы::ЦелЧисло           => Сущ{текст: "целое число",                род: Род::Сред},
            ВидЛексемы::ВещЧисло           => Сущ{текст: "вещественное число",         род: Род::Сред},
            ВидЛексемы::Строка             => Сущ{текст: "строка",                     род: Род::Жен},
        }
    }
}

#[derive(Debug, Clone)]
pub struct Лексема {
    pub вид: ВидЛексемы,
    pub текст: Строка,
    pub лок: Лок,
}

pub struct Лексер<'ы> {
    pub символы: &'ы [char],
    pub путь_к_файлу: &'ы Path,
    pub позиция: usize,
    pub начало_строки: usize,
    pub строка: usize,
    pub буфер: Опция<Результат<Лексема>>
}

pub fn шестнадцатеричная_цифра(знак: &char) -> Опция<i64> {
    if знак.is_digit(10) {
        Некий(*знак as i64 - '0' as i64)
    } else {
        match знак {
            'А' | 'а' => Некий(10),
            'Б' | 'б' => Некий(11),
            'Ц' | 'ц' => Некий(12),
            'Д' | 'д' => Некий(13),
            'Е' | 'е' => Некий(14),
            'Ф' | 'ф' => Некий(15),
            _ => Нету,
        }
    }
}

impl<'a> Лексер<'a> {
    pub fn новый(путь_к_файлу: &'a Path, символы: &'a [char]) -> Лексер<'a> {
        Лексер {
            символы,
            путь_к_файлу,
            позиция: 0,
            начало_строки: 0,
            строка: 0,
            буфер: Нету,
        }
    }

    pub fn текущий_символ(&self) -> Опция<&char> {
        self.символы.get(self.позиция)
    }

    pub fn имеет_приставку(&self, приставка: &[char]) -> bool {
        self.символы
            .get(self.позиция..)
            .map(|строка| строка.starts_with(приставка))
            .unwrap_or(false)
    }

    pub fn отрезать_символы(&mut self, mut количество: usize) {
        while количество > 0 && self.текущий_символ().это_некий() {
            self.отрезать_символ();
            количество -= 1;
        }
    }

    pub fn отрезать_символ(&mut self) {
        if let Некий(&символ) = self.текущий_символ() {
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
        while let Некий(x) = self.текущий_символ().cloned() {
            self.отрезать_символ();
            if x == '\n' {
                break;
            }
        }
    }

    fn лок(&self) -> Лок {
        Лок {
            строка: self.строка + 1,
            столбец: self.позиция - self.начало_строки + 1,
            путь_к_файлу: self.путь_к_файлу.to_path_buf(),
        }
    }

    fn подбрить_многострочные_комментарии(&mut self) {
        let mut вложенность: usize = 1;
        while вложенность > 0 && self.текущий_символ().это_некий() {
            if self.имеет_приставку(&['/', '*']) {
                вложенность += 1;
                self.отрезать_символы(2);
            } else if self.имеет_приставку(&['*', '/']) {
                вложенность -= 1;
                self.отрезать_символы(2);
            } else {
                self.отрезать_символ();
            }
        }
    }

    fn следующая_лексема(&mut self) -> Результат<Лексема> {
        'подбрить_пробелы_и_комментарии: loop {
            self.подбрить_пробелы();
            if self.имеет_приставку(&['/', '/']) {
                self.следующая_строка();
            } else if self.имеет_приставку(&['/', '*']) {
                self.отрезать_символы(2);
                self.подбрить_многострочные_комментарии();
            } else {
                break 'подбрить_пробелы_и_комментарии;
            }
        }

        let лок = self.лок();

        let сим = if let Некий(сим) = self.текущий_символ().cloned() {
            сим
        } else {
            return Хорош(Лексема {
                вид: ВидЛексемы::Конец,
                текст: Строка::new(),
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
                    return Хорош(Лексема {вид, текст, лок})
                }
            }
            return Хорош(Лексема {
                вид: ВидЛексемы::Идент,
                текст,
                лок,
            })
        }

        if self.имеет_приставку(&['1', '6', '%']) {
            self.отрезать_символы(3);
            let начало = self.позиция;
            while self.символы.get(self.позиция).and_then(шестнадцатеричная_цифра).это_некий() {
                self.отрезать_символ();
            }
            let текст = self.символы[начало..self.позиция].iter().collect();
            return Хорош(Лексема {
                вид: ВидЛексемы::ЦелШестЧисло,
                текст,
                лок,
            });
        }

        if сим.is_numeric() {
            let начало = self.позиция;
            while self.символы.get(self.позиция).map(|сим| сим.is_numeric()).unwrap_or(false) {
                self.отрезать_символ();
            }
            // Это условие крайне важно, чтобы разобрать выражение
            // "1..10" как "1", "..", "10".
            if Некий('.') == self.символы.get(self.позиция).cloned() && Некий('.') != self.символы.get(self.позиция + 1).cloned() {
                self.отрезать_символ();
                while self.символы.get(self.позиция).map(|сим| сим.is_numeric()).unwrap_or(false) {
                    self.отрезать_символ();
                }
                let текст = self.символы[начало..self.позиция].iter().collect();
                return Хорош(Лексема {
                    вид: ВидЛексемы::ВещЧисло,
                    текст,
                    лок,
                })
            } else {
                let текст = self.символы[начало..self.позиция].iter().collect();
                return Хорош(Лексема {
                    вид: ВидЛексемы::ЦелЧисло,
                    текст,
                    лок,
                })
            }
        }

        if сим == '«' {
            self.отрезать_символ();
            let mut вложенность: usize = 1;
            let mut текст = Строка::new();
            loop {
                if let Некий(сим) = self.текущий_символ().cloned() {
                    match сим {
                        '«' => {
                            self.отрезать_символ();
                            вложенность += 1;
                            текст.push(сим);
                        }
                        '»' => {
                            self.отрезать_символ();
                            вложенность -= 1;
                            if вложенность == 0 {
                                return Хорош(Лексема {
                                    вид: ВидЛексемы::Строка,
                                    текст,
                                    лок,
                                });
                            }
                            текст.push(сим);
                        }
                        '\\' => {
                            self.отрезать_символ();
                            match self.текущий_символ().cloned() {
                                Некий('н') => {
                                    self.отрезать_символ();
                                    текст.push('\n');
                                }
                                Некий('т') => {
                                    self.отрезать_символ();
                                    текст.push('\t');
                                }
                                Некий('\\') => {
                                    self.отрезать_символ();
                                    текст.push('\\');
                                }
                                Некий('"') => {
                                    let лок = Лок {
                                        строка: self.строка + 1,
                                        столбец: self.позиция - self.начало_строки + 1,
                                        путь_к_файлу: self.путь_к_файлу.to_path_buf(),
                                    };
                                    диагностика!(&лок, "ОШИБКА", "Экранировать \"лапки\" внутри «ёлочек» не нужно!");
                                    return Ошиб(());
                                }
                                Некий('»') => {
                                    self.отрезать_символ();
                                    текст.push('»');
                                }
                                Некий('«') => {
                                    self.отрезать_символ();
                                    текст.push('«');
                                }
                                Некий(сим) => {
                                    let лок = Лок {
                                        строка: self.строка + 1,
                                        столбец: self.позиция - self.начало_строки + 1,
                                        путь_к_файлу: self.путь_к_файлу.to_path_buf(),
                                    };
                                    диагностика!(&лок, "ОШИБКА", "неизвестная поседовательность экранирования начинается с «{сим}»");
                                    return Ошиб(());
                                }
                                Нету => {
                                    диагностика!(&лок, "ОШИБКА", "незавершённая строка");
                                    return Ошиб(());
                                }
                            }
                        }
                        _ => {
                            self.отрезать_символ();
                            текст.push(сим)
                        }
                    }
                } else {
                    диагностика!(&лок, "ОШИБКА", "незавершённая строка");
                    return Ошиб(());
                }
            }
        }

        if сим == '"' {
            self.отрезать_символ();
            let mut текст = Строка::new();
            loop {
                if let Некий(сим) = self.текущий_символ().cloned() {
                    match сим {
                        '"' => {
                            self.отрезать_символ();
                            return Хорош(Лексема {
                                вид: ВидЛексемы::Строка,
                                текст,
                                лок,
                            });
                        }
                        '\\' => {
                            self.отрезать_символ();
                            match self.текущий_символ().cloned() {
                                Некий('н') => {
                                    self.отрезать_символ();
                                    текст.push('\n');
                                }
                                Некий('т') => {
                                    self.отрезать_символ();
                                    текст.push('\t');
                                }
                                Некий('\\') => {
                                    self.отрезать_символ();
                                    текст.push('\\');
                                }
                                Некий('"') => {
                                    self.отрезать_символ();
                                    текст.push('\"');
                                }
                                Некий('«') | Некий('»') => {
                                    let лок = Лок {
                                        строка: self.строка + 1,
                                        столбец: self.позиция - self.начало_строки + 1,
                                        путь_к_файлу: self.путь_к_файлу.to_path_buf(),
                                    };
                                    диагностика!(&лок, "ОШИБКА", "Экранировать «ёлочки» внутри \"лапок\" не нужно!");
                                    return Ошиб(());
                                }
                                Некий(сим) => {
                                    let лок = Лок {
                                        строка: self.строка + 1,
                                        столбец: self.позиция - self.начало_строки + 1,
                                        путь_к_файлу: self.путь_к_файлу.to_path_buf(),
                                    };
                                    диагностика!(&лок, "ОШИБКА", "неизвестная поседовательность экранирования начинается с «{сим}»");
                                    return Ошиб(());
                                }
                                Нету => {
                                    диагностика!(&лок, "ОШИБКА", "незавершённая строка");
                                    return Ошиб(());
                                }
                            }
                        }
                        _ => {
                            self.отрезать_символ();
                            текст.push(сим)
                        }
                    }
                } else {
                    диагностика!(&лок, "ОШИБКА", "незавершённая строка");
                    return Ошиб(());
                }
            }
        }

        for &(приставка, вид) in ПРИСТАВКИ_ПРЕПИНАНИЙ.iter() {
            if self.имеет_приставку(приставка) {
                self.отрезать_символы(приставка.len());
                let текст = приставка.iter().collect();
                return Хорош(Лексема {вид, текст, лок})
            }
        }

        self.отрезать_символ();
        диагностика!(&лок, "ОШИБКА", "Неизвестная лексема начинающаяся с «{сим}»");
        Ошиб(())
    }

    pub fn вытащить_лексему(&mut self) -> Результат<Лексема> {
        if let Некий(лексема) = self.буфер.take() {
            лексема
        } else {
            self.следующая_лексема()
        }
    }

    pub fn подсмотреть_лексему(&mut self) -> Результат<Лексема> {
        if self.буфер.это_нету() {
            self.буфер = Некий(self.следующая_лексема())
        }
        self.буфер.clone().unwrap()
    }

    pub fn вытащить_лексему_вида(&mut self, ожидаемые_виды: &[ВидЛексемы]) -> Результат<Лексема> {
        assert!(ожидаемые_виды.len() > 0);

        let лексема = self.вытащить_лексему()?;
        for ожидаемый_вид in ожидаемые_виды {
            if лексема.вид == *ожидаемый_вид {
                return Хорош(лексема)
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
            let mut список_видов = Строка::new();
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

        Ошиб(())
    }
}
