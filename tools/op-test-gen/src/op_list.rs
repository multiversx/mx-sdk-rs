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
        }
    }

    pub fn is_division(&self) -> bool {
        matches!(self, BaseOperator::Div | BaseOperator::Rem)
    }
}

#[derive(Debug, Clone)]
pub struct OpInfo {
    pub name: String,
    pub base_operator: BaseOperator,
    pub assign: bool,
    pub group: OpGroup,
}

impl OpInfo {
    pub fn new(name: &str, base_operator: BaseOperator, group: OpGroup) -> Self {
        Self {
            name: name.to_owned(),
            base_operator,
            assign: false,
            group,
        }
    }

    pub fn assign(self) -> Self {
        assert!(!self.assign, "Operator is already an assign operator");
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
pub enum OpGroup {
    Arithmetic,
    Bitwise,
    Shift,
}

pub struct OperatorList(pub Vec<OpInfo>);

impl OperatorList {
    pub fn create() -> Self {
        let binary_operators = vec![
            // Arithmetic binary operators
            OpInfo::new("add", BaseOperator::Add, OpGroup::Arithmetic),
            OpInfo::new("sub", BaseOperator::Sub, OpGroup::Arithmetic),
            OpInfo::new("mul", BaseOperator::Mul, OpGroup::Arithmetic),
            OpInfo::new("div", BaseOperator::Div, OpGroup::Arithmetic),
            OpInfo::new("rem", BaseOperator::Rem, OpGroup::Arithmetic),
            // Bitwise binary operators
            OpInfo::new("bit_and", BaseOperator::BitAnd, OpGroup::Bitwise),
            OpInfo::new("bit_or", BaseOperator::BitOr, OpGroup::Bitwise),
            OpInfo::new("bit_xor", BaseOperator::BitXor, OpGroup::Bitwise),
            // Bitwise shift binary operators
            OpInfo::new("shr", BaseOperator::Shr, OpGroup::Shift),
            OpInfo::new("shl", BaseOperator::Shl, OpGroup::Shift),
        ];

        let mut all_operators = Vec::new();
        all_operators.extend(binary_operators.iter().cloned());
        all_operators.extend(binary_operators.iter().cloned().map(|op| op.assign()));
        OperatorList(all_operators)
    }
}
