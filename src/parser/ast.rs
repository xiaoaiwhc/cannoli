// Abstract Syntax Tree definitions
// TODO implement trait or something to add line/col num & other meta info

#[derive(Debug, PartialEq)]
pub enum Ast {
    Module { body: Vec<Statement> }
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Return { value: Option<Expression> },
    Delete { targets: Vec<Expression> },
    Assign { targets: Vec<Expression>, value: Expression },
    AugAssign { target: Expression, op: Operator, value: Expression },
    AnnAssign { target: Expression, annotation: Expression,
        value: Option<Expression> },
    For { target: Expression, iter: Expression, body: Vec<Statement>,
        orelse: Vec<Statement> },
    While { test: Expression, body: Vec<Statement>, orelse: Vec<Statement> },
    If { test: Expression, body: Vec<Statement>, orelse: Vec<Statement> },
    With { items: Vec<WithItem>, body: Vec<Statement> },
    Raise { exc: Option<Expression>, cause: Option<Expression> },
    Try { body: Vec<Statement>, handlers: Vec<ExceptHandler>,
        orelse: Vec<Statement>, finalbody: Vec<Statement> },
    Assert { test: Expression, msg: Option<Expression> },
    Import { names: Vec<Alias> },
    ImportFrom { module: Option<String>, names: Vec<Alias>, level: usize },
    Global { names: Vec<String> },
    Nonlocal { names: Vec<String> },
    Expr { value: Expression },
    Pass,
    Break,
    Continue,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    BoolOp { op: BoolOperator, values: Vec<Expression> },
    BinOp { left: Box<Expression>, op: Operator, right: Box<Expression> },
    UnaryOp { op: UnaryOperator, operand: Box<Expression> },
    If { test: Box<Expression>, body: Box<Expression>,
        orelse: Box<Expression> },
    Dict { keys: Vec<Expression>, values: Vec<Expression> },
    Set { elts: Vec<Expression> },
    ListComp { elt: Box<Expression>, generators: Vec<Comprehension> },
    SetComp { elt: Box<Expression>, generators: Vec<Comprehension> },
    DictComp { key: Box<Expression>, value: Box<Expression>,
        generators: Vec<Comprehension> },
    Generator { elt: Box<Expression>, generators: Vec<Comprehension> },
    None, // For DictComp to have even length key/value lists
    Yield { value: Option<Box<Expression>> },
    YieldFrom { value: Box<Expression> },
    Compare { left: Box<Expression>, ops: Vec<CmpOperator>,
        comparators: Vec<Expression> },
    Call { func: Box<Expression>, args: Vec<Expression>,
        keywords: Vec<Keyword> },
    Num { n: Number },
    Str { s: String },
    NameConstant { value: Singleton },
    Ellipsis,
    Attribute { value: Box<Expression>, attr: String, ctx: ExprContext },
    Subscript { value: Box<Expression>, slice: Box<Slice>, ctx: ExprContext },
    Starred { value: Box<Expression>, ctx: ExprContext },
    Name { id: String, ctx: ExprContext },
    List { elts: Vec<Expression>, ctx: ExprContext },
    Tuple { elts: Vec<Expression>, ctx: ExprContext },
}

#[derive(Debug, PartialEq)]
pub enum ExprContext {
    Load,
    Store,
    Del,
    AugLoad,
    AugStore,
    Param
}

#[derive(Debug, PartialEq)]
pub enum Slice {
    Slice { lower: Option<Expression>, upper: Option<Expression>,
        step: Option<Expression> },
    ExtSlice { dims: Vec<Slice> },
    Index { value: Expression }
}

#[derive(Debug, PartialEq)]
pub enum BoolOperator {
    And,
    Or
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mult,
    MatMult,
    Div,
    Mod,
    Pow,
    LShift,
    RShift,
    BitOr,
    BitXor,
    BitAnd,
    FloorDiv
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Invert,
    Not,
    UAdd,
    USub
}

#[derive(Debug, PartialEq)]
pub enum CmpOperator {
    EQ,
    NE,
    LT,
    LE,
    GT,
    GE,
    Is,
    IsNot,
    In,
    NotIn
}

#[derive(Debug, PartialEq)]
pub enum Comprehension {
    // No { ..., int: is_async } in Comprehension definition
    Comprehension { target: Expression, iter: Expression,
        ifs: Vec<Expression> }
}

#[derive(Debug, PartialEq)]
pub enum ExceptHandler {
    ExceptHandler { etype: Option<Expression>, name: Option<String>,
        body: Vec<Statement> }
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Keyword { arg: Option<String>, value: Expression }
}

#[derive(Debug, PartialEq)]
pub enum Alias {
    Alias { name: String, asname: Option<String> }
}

#[derive(Debug, PartialEq)]
pub enum WithItem {
    WithItem { context_expr: Expression, optional_vars: Option<Expression> }
}

#[derive(Debug, PartialEq)]
pub enum Singleton {
    None,
    True,
    False
}

#[derive(Debug, PartialEq)]
pub enum Number {
    DecInteger(String),
    BinInteger(String),
    OctInteger(String),
    HexInteger(String),
    Float(String),
    Imaginary(String)
}
