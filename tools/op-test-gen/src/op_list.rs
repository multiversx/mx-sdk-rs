#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseOperator {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    BitAnd,
    BitOr,
    BitXor,
    Shr,
    Shl,
    Eq,
    Gt,
    Ge,
    Lt,
    Le,
}

impl BaseOperator {
    pub fn symbol(&self) -> &'static str {
        match self {
            BaseOperator::Add => "+",
            BaseOperator::Sub => "-",
            BaseOperator::Mul => "*",
            BaseOperator::Div => "/",
            BaseOperator::Rem => "%",
            BaseOperator::BitAnd => "&",
            BaseOperator::BitOr => "|",
            BaseOperator::BitXor => "^",
            BaseOperator::Shr => ">>",
            BaseOperator::Shl => "<<",
            BaseOperator::Eq => "==",
            BaseOperator::Gt => ">",
            BaseOperator::Ge => ">=",
            BaseOperator::Lt => "<",
            BaseOperator::Le => "<=",
        }
    }

    pub fn is_division(&self) -> bool {
        matches!(self, BaseOperator::Div | BaseOperator::Rem)
    }
}

#[derive(Debug, Clone)]
pub struct OperatorInfo {
    pub name: String,
    pub base_operator: BaseOperator,
    pub assign: bool,
    pub group: OperatorGroup,
}

impl OperatorInfo {
    pub fn new(name: &str, base_operator: BaseOperator, group: OperatorGroup) -> Self {
        Self {
            name: name.to_owned(),
            base_operator,
            assign: false,
            group,
        }
    }

    pub fn assign(self) -> Self {
        assert!(!self.assign, "Operator is already an assign operator");
        assert_ne!(
            self.group,
            OperatorGroup::Cmp,
            "comparison groups have no assign variant"
        );
        Self {
            name: format!("{}_assign", self.name),
            base_operator: self.base_operator,
            assign: true,
            group: self.group,
        }
    }

    pub fn symbol(&self) -> String {
        if self.assign {
            format!("{}=", self.base_operator.symbol())
        } else {
            self.base_operator.symbol().to_string()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorGroup {
    Arithmetic,
    Bitwise,
    Shift,
    Cmp,
}

pub struct OperatorList(pub Vec<OperatorInfo>);

impl OperatorList {
    pub fn create() -> Self {
        OperatorList(vec![
            // Direct variants
            OperatorInfo::new("add", BaseOperator::Add, OperatorGroup::Arithmetic),
            OperatorInfo::new("sub", BaseOperator::Sub, OperatorGroup::Arithmetic),
            OperatorInfo::new("mul", BaseOperator::Mul, OperatorGroup::Arithmetic),
            OperatorInfo::new("div", BaseOperator::Div, OperatorGroup::Arithmetic),
            OperatorInfo::new("rem", BaseOperator::Rem, OperatorGroup::Arithmetic),
            OperatorInfo::new("bit_and", BaseOperator::BitAnd, OperatorGroup::Bitwise),
            OperatorInfo::new("bit_or", BaseOperator::BitOr, OperatorGroup::Bitwise),
            OperatorInfo::new("bit_xor", BaseOperator::BitXor, OperatorGroup::Bitwise),
            OperatorInfo::new("shr", BaseOperator::Shr, OperatorGroup::Shift),
            OperatorInfo::new("shl", BaseOperator::Shl, OperatorGroup::Shift),
            // Assign variants
            OperatorInfo::new("add", BaseOperator::Add, OperatorGroup::Arithmetic).assign(),
            OperatorInfo::new("sub", BaseOperator::Sub, OperatorGroup::Arithmetic).assign(),
            OperatorInfo::new("mul", BaseOperator::Mul, OperatorGroup::Arithmetic).assign(),
            OperatorInfo::new("div", BaseOperator::Div, OperatorGroup::Arithmetic).assign(),
            OperatorInfo::new("rem", BaseOperator::Rem, OperatorGroup::Arithmetic).assign(),
            OperatorInfo::new("bit_and", BaseOperator::BitAnd, OperatorGroup::Bitwise).assign(),
            OperatorInfo::new("bit_or", BaseOperator::BitOr, OperatorGroup::Bitwise).assign(),
            OperatorInfo::new("bit_xor", BaseOperator::BitXor, OperatorGroup::Bitwise).assign(),
            OperatorInfo::new("shr", BaseOperator::Shr, OperatorGroup::Shift).assign(),
            OperatorInfo::new("shl", BaseOperator::Shl, OperatorGroup::Shift).assign(),
            // Equality/comparison
            OperatorInfo::new("eq", BaseOperator::Eq, OperatorGroup::Cmp),
            OperatorInfo::new("gt", BaseOperator::Gt, OperatorGroup::Cmp),
            OperatorInfo::new("ge", BaseOperator::Ge, OperatorGroup::Cmp),
            OperatorInfo::new("lt", BaseOperator::Lt, OperatorGroup::Cmp),
            OperatorInfo::new("le", BaseOperator::Le, OperatorGroup::Cmp),
        ])
    }
}
