use std::borrow::Cow;

pub enum InsnAttributeKey<'a> {
    OrigFmt,
    OrigName,
    LA32,
    LBT,
    LVZ,
    Primary,
    Provisional,
    QEMU,
    Rev,
    Custom(Cow<'a, str>),
}

impl<'a> From<&'a str> for InsnAttributeKey<'a> {
    fn from(x: &'a str) -> Self {
        match x {
            "orig_fmt" => Self::OrigFmt,
            "orig_name" => Self::OrigName,
            "la32" => Self::LA32,
            "lbt" => Self::LBT,
            "lvz" => Self::LVZ,
            "primary" => Self::Primary,
            "provisional" => Self::Provisional,
            "qemu" => Self::QEMU,
            "rev" => Self::Rev,
            _ => Self::Custom(Cow::Borrowed(x)),
        }
    }
}

pub enum InsnAttributeDecl<'a> {
    BoolTrue(InsnAttributeKey<'a>),
    String(InsnAttributeKey<'a>, Cow<'a, str>),
}
