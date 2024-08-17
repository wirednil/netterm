use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[regex(r"_+")]
    FieldAlphanumeric,

    #[regex(r"([A-Za-záéíóúÁÉÍÓÚñÑ .]+:)")]
    Label,

    #[regex(r"-?((_?_?_(,___)+)|(_+))((\._*)|(\.\#))")]
    FieldNumeric,

    #[regex(r"_{2}/_{2}/_{2}")]
    FieldShortDate,

    #[regex(r"_{2}/_{2}/_{4}")]
    FieldLongDate,

    #[regex(r"_{2}:_{2}")]
    FieldShortTime,

    #[regex(r"_{2}:_{2}:_{2}")]
    FieldLongTIme,
}