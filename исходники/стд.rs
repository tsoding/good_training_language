pub mod прелюдия {
    pub use super::{Опция, Некий, Нету, РасширениеОпции};
    pub use super::{Хорош, Ошиб};
    pub use super::{Строка, ВСтроку};
}

pub use std::option::Option as Опция;
pub use std::option::Option::Some as Некий;
pub use std::option::Option::None as Нету;

pub trait РасширениеОпции {
    fn это_некий(&self) -> bool;
    fn это_нету(&self) -> bool;
}

impl<Тэ> РасширениеОпции for Опция<Тэ> {
    fn это_некий(&self) -> bool {
        self.is_some()
    }

    fn это_нету(&self) -> bool {
        self.is_none()
    }
}

pub use std::result::Result as Результат;
pub use std::result::Result::Ok as Хорош;
pub use std::result::Result::Err as Ошиб;

pub use std::string::String as Строка;

pub trait ВСтроку {
    fn в_строку(&self) -> Строка;
}

impl<Тэ: ToString> ВСтроку for Тэ {
    fn в_строку(&self) -> Строка {
        self.to_string()
    }
}

macro_rules! вектор {
    ($( $t:tt )*) => { vec![$( $t )*] }
}

pub mod коллекции {
    pub use std::vec::Vec as Вектор;
    pub use std::collections::HashMap as ХэшТаблица;
}
