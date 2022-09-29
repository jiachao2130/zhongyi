#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]
pub struct Quantum {
    size: usize,
    unit: Unit
}

impl Quantum {
    fn new(size: usize, unit: Unit) -> Self {
        Quantum {
            size,
            unit: unit
        }
    }
}

/// 目前总共有三种单位
///     其一是模糊单位，如几个，几枚，几分
///     其二为重量单位，铢-两-斤-钧-石
///     其三是容积单位，龠-合-升-斗-斛
#[derive(Clone, Debug)]
pub enum Unit {
    FuzzyUnit(String),
    WeightUnit(WeightUnit),
    VolumeUnit(VolumeUnit)
}

impl ToString for Unit {
    fn to_string(&self) -> String {
        match self {
            Unit::FuzzyUnit(s) => { s.to_string() },
            Unit::WeightUnit(w) => { w.to_string() },
            Unit::VolumeUnit(v) => { v.to_string() },
        }
    }
}

/// 重量计量单位
#[derive(Clone, Copy, Debug)]
pub enum WeightUnit {
    Zhu,    // 6 豆
    Liang,  // 24 铢
    Jin,    // 16 两
    Jun,    // 30 斤
    Shi     // 4 钧
}

impl ToString for WeightUnit {
    fn to_string(&self) -> String {
        match self {
            WeightUnit::Zhu => { String::from("铢") },
            WeightUnit::Liang => { String::from("两") },
            WeightUnit::Jin => { String::from("斤") },
            WeightUnit::Jun => { String::from("钧") },
            WeightUnit::Shi => { String::from("石") }
        }
    }
}

/// 容积计量单位
#[derive(Clone, Copy, Debug)]
pub enum VolumeUnit {
    Yue,    // 千二百黍实其龠
    Ge,     // 2 龠
    Sheng,  // 10 合
    Dou,    // 10 升
    Hu,     // 10 斗
}

impl ToString for VolumeUnit {
    fn to_string(&self) -> String {
        match self {
            VolumeUnit::Yue => { String::from("龠") },
            VolumeUnit::Ge => { String::from("合") },
            VolumeUnit::Sheng => { String::from("升") },
            VolumeUnit::Dou => { String::from("斗") },
            VolumeUnit::Hu => { String::from("斛") },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_quantum() {
        let _ge = Unit::VolumeUnit(VolumeUnit::Ge);
        let _liang = Unit::WeightUnit(WeightUnit::Liang);
        let _fen = Unit::FuzzyUnit("分".to_string());

        let _yi_ge = Quantum::new(1, _ge.clone());
        let _er_liang = Quantum::new(2, _liang.clone());
        let _yi_fen = Quantum::new(1, _fen.clone());

        // test to_string
        assert_eq!(_ge.to_string(), "合");
        assert_eq!(_liang.to_string(), "两");
        assert_eq!(_fen.to_string(), "分");
    }
}
