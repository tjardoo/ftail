use std::fmt::{Display, Formatter, Result};

macro_rules! generate_styling_functions {
    (
        $(#[$enum_attrs:meta])*
        $vis:vis enum $enum_name:ident {
            $(
                #[code = $code:literal]
                $(#[$variant_attrs:meta])*
                $variant:ident
            ),*
            $(,)?
        }
    ) => {
        $(#[$enum_attrs])*
        $vis enum $enum_name {
            $(
                $(#[$variant_attrs])*
                $variant,
            )*
        }

        impl $enum_name {
            #[allow(dead_code)]
            pub fn code(&self) -> u8 {
                match self {
                    $(Self::$variant => $code,)*
                }
            }
        }

        impl $enum_name {
            $(
                #[allow(dead_code)]
                pub fn $variant() -> Self {
                    Self::$variant
                }
            )*
        }

        #[allow(dead_code)]
        pub trait TextStyling: Display {
            $(
                fn $variant(self) -> Style<Self>
                where
                    Self: Sized,
                {
                    self.style($enum_name::$variant)
                }
            )*
        }

        impl<T: Display> TextStyling for T {}
    }
}

pub struct Style<T> {
    text: T,
    code: u8,
}

generate_styling_functions! {
    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub enum StyleCode {
        #[code = 30]
        black,
        #[code = 31]
        red,
        #[code = 32]
        green,
        #[code = 33]
        yellow,
        #[code = 34]
        blue,
        #[code = 35]
        magenta,
        #[code = 36]
        cyan,
        #[code = 37]
        white,
        #[code = 90]
        bright_black,
        #[code = 91]
        bright_red,
        #[code = 92]
        bright_green,
        #[code = 93]
        bright_yellow,
        #[code = 94]
        bright_blue,
        #[code = 95]
        bright_magenta,
        #[code = 96]
        bright_cyan,
        #[code = 97]
        bright_white,
        #[code = 40]
        bg_black,
        #[code = 41]
        bg_red,
        #[code = 42]
        bg_green,
        #[code = 43]
        bg_yellow,
        #[code = 44]
        bg_blue,
        #[code = 45]
        bg_magenta,
        #[code = 46]
        bg_cyan,
        #[code = 47]
        bg_white,
        #[code = 100]
        bg_bright_black,
        #[code = 101]
        bg_bright_red,
        #[code = 102]
        bg_bright_green,
        #[code = 103]
        bg_bright_yellow,
        #[code = 104]
        bg_bright_blue,
        #[code = 105]
        bg_bright_magenta,
        #[code = 106]
        bg_bright_cyan,
        #[code = 107]
        bg_bright_white,
        #[code = 1]
        bold,
        #[code = 3]
        italic,
        #[code = 4]
        underline,
        #[code = 9]
        strikethrough,
    }
}

pub trait GeneratedTextStyling: Display + TextStyling {
    fn style(self, style_code: StyleCode) -> Style<Self>
    where
        Self: Sized,
    {
        Style {
            text: self,
            code: style_code.code(),
        }
    }
}

impl<T: Display> GeneratedTextStyling for T {}

impl<T: Display> Display for Style<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "\x1b[{}m{}\x1b[0m", self.code, self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_styling() {
        let text = "Hello, world!";

        let black = text.black();
        let red = text.red();
        let green = text.green();
        let yellow = text.yellow();
        let blue = text.blue();
        let magenta = text.magenta();
        let cyan = text.cyan();
        let white = text.white();
        let bright_black = text.bright_black();
        let bright_red = text.bright_red();
        let bright_green = text.bright_green();
        let bright_yellow = text.bright_yellow();
        let bright_blue = text.bright_blue();
        let bright_magenta = text.bright_magenta();
        let bright_cyan = text.bright_cyan();
        let bright_white = text.bright_white();
        let bg_black = text.bg_black();
        let bg_red = text.bg_red();
        let bg_green = text.bg_green();
        let bg_yellow = text.bg_yellow();
        let bg_blue = text.bg_blue();
        let bg_magenta = text.bg_magenta();
        let bg_cyan = text.bg_cyan();
        let bg_white = text.bg_white();
        let bg_bright_black = text.bg_bright_black();
        let bg_bright_red = text.bg_bright_red();
        let bg_bright_green = text.bg_bright_green();
        let bg_bright_yellow = text.bg_bright_yellow();
        let bg_bright_blue = text.bg_bright_blue();
        let bg_bright_magenta = text.bg_bright_magenta();
        let bg_bright_cyan = text.bg_bright_cyan();
        let bg_bright_white = text.bg_bright_white();
        let bold = text.bold();
        let italic = text.italic();
        let underline = text.underline();
        let strikethrough = text.strikethrough();

        assert_eq!(format!("{}", black), "\x1b[30mHello, world!\x1b[0m");
        assert_eq!(format!("{}", red), "\x1b[31mHello, world!\x1b[0m");
        assert_eq!(format!("{}", green), "\x1b[32mHello, world!\x1b[0m");
        assert_eq!(format!("{}", yellow), "\x1b[33mHello, world!\x1b[0m");
        assert_eq!(format!("{}", blue), "\x1b[34mHello, world!\x1b[0m");
        assert_eq!(format!("{}", magenta), "\x1b[35mHello, world!\x1b[0m");
        assert_eq!(format!("{}", cyan), "\x1b[36mHello, world!\x1b[0m");
        assert_eq!(format!("{}", white), "\x1b[37mHello, world!\x1b[0m");
        assert_eq!(format!("{}", bright_black), "\x1b[90mHello, world!\x1b[0m");
        assert_eq!(format!("{}", bright_red), "\x1b[91mHello, world!\x1b[0m");
        assert_eq!(format!("{}", bright_green), "\x1b[92mHello, world!\x1b[0m");
        assert_eq!(format!("{}", bright_yellow), "\x1b[93mHello, world!\x1b[0m");
        assert_eq!(format!("{}", bright_blue), "\x1b[94mHello, world!\x1b[0m");
        assert_eq!(
            format!("{}", bright_magenta),
            "\x1b[95mHello, world!\x1b[0m"
        );
        assert_eq!(format!("{}", bright_cyan), "\x1b[96mHello, world!\x1b[0m");
        assert_eq!(format!("{}", bright_white), "\x1b[97mHello, world!\x1b[0m");
        assert_eq!(format!("{}", bg_black), "\x1b[40mHello, world!\x1b[0m");
        assert_eq!(format!("{}", bg_red), "\x1b[41mHello, world!\x1b[0m");
        assert_eq!(format!("{}", bg_green), "\x1b[42mHello, world!\x1b[0m");
        assert_eq!(format!("{}", bg_yellow), "\x1b[43mHello, world!\x1b[0m");
        assert_eq!(format!("{}", bg_blue), "\x1b[44mHello, world!\x1b[0m");
        assert_eq!(format!("{}", bg_magenta), "\x1b[45mHello, world!\x1b[0m");
        assert_eq!(format!("{}", bg_cyan), "\x1b[46mHello, world!\x1b[0m");
        assert_eq!(format!("{}", bg_white), "\x1b[47mHello, world!\x1b[0m");
        assert_eq!(
            format!("{}", bg_bright_black),
            "\x1b[100mHello, world!\x1b[0m"
        );
        assert_eq!(
            format!("{}", bg_bright_red),
            "\x1b[101mHello, world!\x1b[0m"
        );
        assert_eq!(
            format!("{}", bg_bright_green),
            "\x1b[102mHello, world!\x1b[0m"
        );
        assert_eq!(
            format!("{}", bg_bright_yellow),
            "\x1b[103mHello, world!\x1b[0m"
        );
        assert_eq!(
            format!("{}", bg_bright_blue),
            "\x1b[104mHello, world!\x1b[0m"
        );
        assert_eq!(
            format!("{}", bg_bright_magenta),
            "\x1b[105mHello, world!\x1b[0m"
        );
        assert_eq!(
            format!("{}", bg_bright_cyan),
            "\x1b[106mHello, world!\x1b[0m"
        );
        assert_eq!(
            format!("{}", bg_bright_white),
            "\x1b[107mHello, world!\x1b[0m"
        );
        assert_eq!(format!("{}", bold), "\x1b[1mHello, world!\x1b[0m");
        assert_eq!(format!("{}", italic), "\x1b[3mHello, world!\x1b[0m");
        assert_eq!(format!("{}", underline), "\x1b[4mHello, world!\x1b[0m");
        assert_eq!(format!("{}", strikethrough), "\x1b[9mHello, world!\x1b[0m");
    }
}
