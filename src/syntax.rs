use crossterm::style::Color;


#[derive(Clone)]
pub struct Syntax {
    pub number: bool,
    pub string: &'static [&'static str],
    pub character: Option<&'static str>,
    pub number_delim: Option<char>,
    pub line_comment: Option<&'static str>,
    pub block_comment: Option<(&'static str, &'static str)>,
    pub keywords: &'static [&'static str],
    pub control_statements: &'static [&'static str],
    pub types: &'static [&'static str],
    pub boolean: &'static [&'static str],
    pub def_keywords: &'static [&'static str],
    pub colors: SyntaxColor

}

#[derive(Clone)]
pub struct SyntaxColor {
    pub number: Color,
    pub string: Color,
    pub character: Color,
    pub comment: Color,
    pub keywords: Color,
    pub control_statements: Color,
    pub types: Color,
    pub boolean: Color,
    pub def_keywords: Color
}

pub const C: Syntax = Syntax {
    number: true,
    number_delim: None,
    string: &[r#"""#],
    character: Some("'"),
    line_comment: Some("//"),
    block_comment: Some(("/*", "*/")),
    keywords: &[
        "auto", "const", "enum", "extern", "inline", "register", "restrict", "sizeof", "static",
        "struct", "typedef", "union", "volatile",
    ],
    control_statements: &[
        "break", "case", "continue", "default", "do", "else", "for", "goto", "if", "return",
        "switch", "while",
    ],
    types: &[
        "char", "double", "float", "int", "long", "short", "signed", "unsigned", "void",
    ],
    boolean: &[],
    def_keywords: &["enum", "struct", "union"],

    colors: SyntaxColor {
        number: Color::Green,
        string: Color::DarkRed,
        character: Color::DarkRed,
        comment: Color::DarkGreen,
        keywords: Color::Magenta,
        control_statements: Color::Magenta,
        types: Color::Blue,
        boolean: Color::Red,
        def_keywords: Color::DarkYellow
    }
};