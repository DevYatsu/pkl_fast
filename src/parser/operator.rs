
#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
/// An enum representing an assignement operator, used in variable assignement
pub enum AssignOperator {
    /// The assignment operator `=`
    Equal,

    /// The assignment operator `+=`
    PlusEqual,

    /// The assignment operator `-=`
    MinusEqual,

    /// The assignment operator `*=`
    TimesEqual,

    /// The assignment operator `/=`
    DivideEqual,

    /// The assignment operator `%=`
    ModuloEqual,
}

impl From<&str> for AssignOperator {
    fn from(value: &str) -> Self {
        match value {
            "=" => AssignOperator::Equal,
            "+=" => AssignOperator::PlusEqual,
            "-=" => AssignOperator::MinusEqual,
            "*=" => AssignOperator::TimesEqual,
            "/=" => AssignOperator::DivideEqual,
            "%=" => AssignOperator::ModuloEqual
        }
    }
}