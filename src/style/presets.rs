use super::theme::*;
use super::*;

pub fn default_colors_theme() -> Theme {
    Theme::default()
        .set_color(seed_colors::Base::Black, CssColor::Hex(0x000000))
        .set_color(seed_colors::Base::White, CssColor::Hex(0xFFFFFF))
        .set_color(seed_colors::Gray::No1, CssColor::Hex(0xF7FAFC))
        .set_color(seed_colors::Gray::No2, CssColor::Hex(0xEDF2F7))
        .set_color(seed_colors::Gray::No3, CssColor::Hex(0xE2E8F0))
        .set_color(seed_colors::Gray::No4, CssColor::Hex(0xCBD5E0))
        .set_color(seed_colors::Gray::No5, CssColor::Hex(0xA0AEC0))
        .set_color(seed_colors::Gray::No6, CssColor::Hex(0x718096))
        .set_color(seed_colors::Gray::No7, CssColor::Hex(0x4A5568))
        .set_color(seed_colors::Gray::No8, CssColor::Hex(0x2D3748))
        .set_color(seed_colors::Gray::No9, CssColor::Hex(0x1A202C))
        .set_color(seed_colors::Red::No1, CssColor::Hex(0xFFF5F5))
        .set_color(seed_colors::Red::No2, CssColor::Hex(0xFED7D7))
        .set_color(seed_colors::Red::No3, CssColor::Hex(0xFEB2B2))
        .set_color(seed_colors::Red::No4, CssColor::Hex(0xFC8181))
        .set_color(seed_colors::Red::No5, CssColor::Hex(0xF56565))
        .set_color(seed_colors::Red::No6, CssColor::Hex(0xE53E3E))
        .set_color(seed_colors::Red::No7, CssColor::Hex(0xC53030))
        .set_color(seed_colors::Red::No8, CssColor::Hex(0x9B2C2C))
        .set_color(seed_colors::Red::No9, CssColor::Hex(0x742A2A))
        .set_color(seed_colors::Orange::No1, CssColor::Hex(0xFFFAF0))
        .set_color(seed_colors::Orange::No2, CssColor::Hex(0xFEEBC8))
        .set_color(seed_colors::Orange::No3, CssColor::Hex(0xFBD38D))
        .set_color(seed_colors::Orange::No4, CssColor::Hex(0xF6AD55))
        .set_color(seed_colors::Orange::No5, CssColor::Hex(0xED8936))
        .set_color(seed_colors::Orange::No6, CssColor::Hex(0xDD6B20))
        .set_color(seed_colors::Orange::No7, CssColor::Hex(0xC05621))
        .set_color(seed_colors::Orange::No8, CssColor::Hex(0x9C4221))
        .set_color(seed_colors::Orange::No9, CssColor::Hex(0x7B341E))
        .set_color(seed_colors::Yellow::No1, CssColor::Hex(0xFFFFF0))
        .set_color(seed_colors::Yellow::No2, CssColor::Hex(0xFEFCBF))
        .set_color(seed_colors::Yellow::No3, CssColor::Hex(0xFAF089))
        .set_color(seed_colors::Yellow::No4, CssColor::Hex(0xF6E05E))
        .set_color(seed_colors::Yellow::No5, CssColor::Hex(0xECC94B))
        .set_color(seed_colors::Yellow::No6, CssColor::Hex(0xD69E2E))
        .set_color(seed_colors::Yellow::No7, CssColor::Hex(0xB7791F))
        .set_color(seed_colors::Yellow::No8, CssColor::Hex(0x975A16))
        .set_color(seed_colors::Yellow::No9, CssColor::Hex(0x744210))
        .set_color(seed_colors::Green::No1, CssColor::Hex(0xF0FFF4))
        .set_color(seed_colors::Green::No2, CssColor::Hex(0xC6F6D5))
        .set_color(seed_colors::Green::No3, CssColor::Hex(0x9AE6B4))
        .set_color(seed_colors::Green::No4, CssColor::Hex(0x68D391))
        .set_color(seed_colors::Green::No5, CssColor::Hex(0x48BB78))
        .set_color(seed_colors::Green::No6, CssColor::Hex(0x38A169))
        .set_color(seed_colors::Green::No7, CssColor::Hex(0x2F855A))
        .set_color(seed_colors::Green::No8, CssColor::Hex(0x276749))
        .set_color(seed_colors::Green::No9, CssColor::Hex(0x22543D))
        .set_color(seed_colors::Teal::No1, CssColor::Hex(0xE6FFFA))
        .set_color(seed_colors::Teal::No2, CssColor::Hex(0xB2F5EA))
        .set_color(seed_colors::Teal::No3, CssColor::Hex(0x81E6D9))
        .set_color(seed_colors::Teal::No4, CssColor::Hex(0x4FD1C5))
        .set_color(seed_colors::Teal::No5, CssColor::Hex(0x38B2AC))
        .set_color(seed_colors::Teal::No6, CssColor::Hex(0x319795))
        .set_color(seed_colors::Teal::No7, CssColor::Hex(0x2C7A7B))
        .set_color(seed_colors::Teal::No8, CssColor::Hex(0x285E61))
        .set_color(seed_colors::Teal::No9, CssColor::Hex(0x234E52))
        .set_color(seed_colors::Blue::No1, CssColor::Hex(0xEBF8FF))
        .set_color(seed_colors::Blue::No2, CssColor::Hex(0xBEE3F8))
        .set_color(seed_colors::Blue::No3, CssColor::Hex(0x90CDF4))
        .set_color(seed_colors::Blue::No4, CssColor::Hex(0x63B3ED))
        .set_color(seed_colors::Blue::No5, CssColor::Hex(0x4299E1))
        .set_color(seed_colors::Blue::No6, CssColor::Hex(0x3182CE))
        .set_color(seed_colors::Blue::No7, CssColor::Hex(0x2B6CB0))
        .set_color(seed_colors::Blue::No8, CssColor::Hex(0x2C5282))
        .set_color(seed_colors::Blue::No9, CssColor::Hex(0x2A4365))
        .set_color(seed_colors::Indigo::No1, CssColor::Hex(0xEBF4FF))
        .set_color(seed_colors::Indigo::No2, CssColor::Hex(0xC3DAFE))
        .set_color(seed_colors::Indigo::No3, CssColor::Hex(0xA3BFFA))
        .set_color(seed_colors::Indigo::No4, CssColor::Hex(0x7F9CF5))
        .set_color(seed_colors::Indigo::No5, CssColor::Hex(0x667EEA))
        .set_color(seed_colors::Indigo::No6, CssColor::Hex(0x5A67D8))
        .set_color(seed_colors::Indigo::No7, CssColor::Hex(0x4C51BF))
        .set_color(seed_colors::Indigo::No8, CssColor::Hex(0x434190))
        .set_color(seed_colors::Indigo::No9, CssColor::Hex(0x3C366B))
        .set_color(seed_colors::Purple::No1, CssColor::Hex(0xFAF5FF))
        .set_color(seed_colors::Purple::No2, CssColor::Hex(0xE9D8FD))
        .set_color(seed_colors::Purple::No3, CssColor::Hex(0xD6BCFA))
        .set_color(seed_colors::Purple::No4, CssColor::Hex(0xB794F4))
        .set_color(seed_colors::Purple::No5, CssColor::Hex(0x9F7AEA))
        .set_color(seed_colors::Purple::No6, CssColor::Hex(0x805AD5))
        .set_color(seed_colors::Purple::No7, CssColor::Hex(0x6B46C1))
        .set_color(seed_colors::Purple::No8, CssColor::Hex(0x553C9A))
        .set_color(seed_colors::Purple::No9, CssColor::Hex(0x44337A))
        .set_color(seed_colors::Pink::No1, CssColor::Hex(0xFFF5F7))
        .set_color(seed_colors::Pink::No2, CssColor::Hex(0xFED7E2))
        .set_color(seed_colors::Pink::No3, CssColor::Hex(0xFBB6CE))
        .set_color(seed_colors::Pink::No4, CssColor::Hex(0xF687B3))
        .set_color(seed_colors::Pink::No5, CssColor::Hex(0xED64A6))
        .set_color(seed_colors::Pink::No6, CssColor::Hex(0xD53F8C))
        .set_color(seed_colors::Pink::No7, CssColor::Hex(0xB83280))
        .set_color(seed_colors::Pink::No8, CssColor::Hex(0x97266D))
        .set_color(seed_colors::Pink::No9, CssColor::Hex(0x702459))
}

pub mod seed_colors {
    use super::*;
    #[derive(Hash, PartialEq, Eq, Clone)]
    pub enum Base {
        White,
        Black,
    }
    impl ColorTheme for Base {}

    #[derive(Hash, PartialEq, Eq, Clone)]
    pub enum Red {
        No1,
        No2,
        No3,
        No4,
        No5,
        No6,
        No7,
        No8,
        No9,
    }
    impl ColorTheme for Red {}
    #[derive(Hash, PartialEq, Eq, Clone)]
    pub enum Blue {
        No1,
        No2,
        No3,
        No4,
        No5,
        No6,
        No7,
        No8,
        No9,
    }
    impl ColorTheme for Blue {}
    #[derive(Hash, PartialEq, Eq, Clone)]
    pub enum Green {
        No1,
        No2,
        No3,
        No4,
        No5,
        No6,
        No7,
        No8,
        No9,
    }
    impl ColorTheme for Green {}

    #[derive(Hash, PartialEq, Eq, Clone)]
    pub enum Orange {
        No1,
        No2,
        No3,
        No4,
        No5,
        No6,
        No7,
        No8,
        No9,
    }
    impl ColorTheme for Orange {}

    #[derive(Hash, PartialEq, Eq, Clone)]
    pub enum Pink {
        No1,
        No2,
        No3,
        No4,
        No5,
        No6,
        No7,
        No8,
        No9,
    }
    impl ColorTheme for Pink {}

    #[derive(Hash, PartialEq, Eq, Clone)]
    pub enum Teal {
        No1,
        No2,
        No3,
        No4,
        No5,
        No6,
        No7,
        No8,
        No9,
    }
    impl ColorTheme for Teal {}

    #[derive(Hash, PartialEq, Eq, Clone)]
    pub enum Indigo {
        No1,
        No2,
        No3,
        No4,
        No5,
        No6,
        No7,
        No8,
        No9,
    }
    impl ColorTheme for Indigo {}
    #[derive(Hash, PartialEq, Eq, Clone)]
    pub enum Purple {
        No1,
        No2,
        No3,
        No4,
        No5,
        No6,
        No7,
        No8,
        No9,
    }
    impl ColorTheme for Purple {}

    #[derive(Hash, PartialEq, Eq, Clone)]
    pub enum Gray {
        No1,
        No2,
        No3,
        No4,
        No5,
        No6,
        No7,
        No8,
        No9,
    }
    impl ColorTheme for Gray {}

    #[derive(Hash, PartialEq, Eq, Clone)]
    pub enum Yellow {
        No1,
        No2,
        No3,
        No4,
        No5,
        No6,
        No7,
        No8,
        No9,
    }
    impl ColorTheme for Yellow {}
}
