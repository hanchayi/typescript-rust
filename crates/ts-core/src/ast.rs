//! Abstract Syntax Tree definitions for TypeScript

use crate::utils::span::Span;
use serde::{Serialize, Deserialize};

/// AST node types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AstNode {
    Statement(Statement),
    Expression(Expression),
    Declaration(Declaration),
}

/// Statement types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    Expression(Expression),
    Block(BlockStatement),
    If(IfStatement),
    While(WhileStatement),
    For(ForStatement),
    Return(ReturnStatement),
    Break(BreakStatement),
    Continue(ContinueStatement),
    Variable(VariableStatement),
}

/// Expression types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Call(CallExpression),
    Member(MemberExpression),
    Assignment(AssignmentExpression),
    Function(FunctionExpression),
    Arrow(Box<ArrowFunctionExpression>),
    Object(ObjectExpression),
    Array(ArrayExpression),
}

/// Declaration types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Declaration {
    Function(FunctionDeclaration),
    Variable(VariableDeclaration),
    Class(ClassDeclaration),
    Interface(InterfaceDeclaration),
    Type(TypeDeclaration),
    Enum(EnumDeclaration),
    Namespace(NamespaceDeclaration),
    Import(ImportDeclaration),
    Export(Box<ExportDeclaration>),
}

/// Identifier
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Identifier {
    pub name: String,
    pub span: Span,
}

/// Literal values
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
    Undefined,
}

/// Binary expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: BinaryOperator,
    pub right: Box<Expression>,
    pub span: Span,
}

/// Binary operators
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    StrictEqual,
    StrictNotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LogicalAnd,
    LogicalOr,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
    UnsignedRightShift,
}

/// Unary expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnaryExpression {
    pub operator: UnaryOperator,
    pub operand: Box<Expression>,
    pub span: Span,
}

/// Unary operators
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnaryOperator {
    Plus,
    Minus,
    LogicalNot,
    BitwiseNot,
    Typeof,
    Void,
    Delete,
    PreIncrement,
    PostIncrement,
    PreDecrement,
    PostDecrement,
}

/// Function call expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallExpression {
    pub callee: Box<Expression>,
    pub arguments: Vec<Expression>,
    pub span: Span,
}

/// Member access expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemberExpression {
    pub object: Box<Expression>,
    pub property: Box<Expression>,
    pub computed: bool, // true for obj[prop], false for obj.prop
    pub span: Span,
}

/// Assignment expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssignmentExpression {
    pub left: Box<Expression>,
    pub operator: AssignmentOperator,
    pub right: Box<Expression>,
    pub span: Span,
}

/// Assignment operators
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AssignmentOperator {
    Assign,
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,
    BitwiseAndAssign,
    BitwiseOrAssign,
    BitwiseXorAssign,
    LeftShiftAssign,
    RightShiftAssign,
    UnsignedRightShiftAssign,
}

/// Block statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
    pub span: Span,
}

/// If statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IfStatement {
    pub test: Expression,
    pub consequent: Box<Statement>,
    pub alternate: Option<Box<Statement>>,
    pub span: Span,
}

/// While statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WhileStatement {
    pub test: Expression,
    pub body: Box<Statement>,
    pub span: Span,
}

/// For statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForStatement {
    pub init: Option<Box<Statement>>,
    pub test: Option<Expression>,
    pub update: Option<Expression>,
    pub body: Box<Statement>,
    pub span: Span,
}

/// Return statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReturnStatement {
    pub argument: Option<Expression>,
    pub span: Span,
}

/// Break statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreakStatement {
    pub label: Option<Identifier>,
    pub span: Span,
}

/// Continue statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContinueStatement {
    pub label: Option<Identifier>,
    pub span: Span,
}

/// Variable statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariableStatement {
    pub declarations: Vec<VariableDeclaration>,
    pub span: Span,
}

/// Function expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionExpression {
    pub name: Option<Identifier>,
    pub parameters: Vec<Parameter>,
    pub body: BlockStatement,
    pub span: Span,
}

/// Arrow function expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrowFunctionExpression {
    pub parameters: Vec<Parameter>,
    pub body: Box<ArrowFunctionBody>,
    pub span: Span,
}

/// Arrow function body
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ArrowFunctionBody {
    Expression(Box<Expression>),
    Block(BlockStatement),
}

/// Object expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectExpression {
    pub properties: Vec<ObjectProperty>,
    pub span: Span,
}

/// Object property
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectProperty {
    pub key: Expression,
    pub value: Expression,
    pub computed: bool,
    pub span: Span,
}

/// Array expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrayExpression {
    pub elements: Vec<Option<Expression>>,
    pub span: Span,
}

/// Function declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionDeclaration {
    pub name: Identifier,
    pub parameters: Vec<Parameter>,
    pub body: BlockStatement,
    pub span: Span,
}

/// Variable declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariableDeclaration {
    pub name: Identifier,
    pub init: Option<Expression>,
    pub span: Span,
}

/// Function parameter
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    pub name: Identifier,
    pub type_annotation: Option<TypeAnnotation>,
    pub optional: bool,
    pub span: Span,
}

/// Type annotation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeAnnotation {
    pub type_name: String,
    pub span: Span,
}

/// Class declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassDeclaration {
    pub name: Identifier,
    pub super_class: Option<Expression>,
    pub body: Vec<ClassMember>,
    pub span: Span,
}

/// Class member
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClassMember {
    Method(MethodDefinition),
    Property(PropertyDefinition),
    Constructor(ConstructorDefinition),
}

/// Method definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MethodDefinition {
    pub name: Identifier,
    pub parameters: Vec<Parameter>,
    pub body: BlockStatement,
    pub is_static: bool,
    pub span: Span,
}

/// Property definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PropertyDefinition {
    pub name: Identifier,
    pub value: Option<Expression>,
    pub is_static: bool,
    pub span: Span,
}

/// Constructor definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstructorDefinition {
    pub parameters: Vec<Parameter>,
    pub body: BlockStatement,
    pub span: Span,
}

/// Interface declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterfaceDeclaration {
    pub name: Identifier,
    pub members: Vec<InterfaceMember>,
    pub span: Span,
}

/// Interface member
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InterfaceMember {
    Property(InterfaceProperty),
    Method(InterfaceMethod),
}

/// Interface property
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterfaceProperty {
    pub name: Identifier,
    pub type_annotation: TypeAnnotation,
    pub optional: bool,
    pub span: Span,
}

/// Interface method
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterfaceMethod {
    pub name: Identifier,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<TypeAnnotation>,
    pub span: Span,
}

/// Type declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeDeclaration {
    pub name: Identifier,
    pub type_annotation: TypeAnnotation,
    pub span: Span,
}

/// Enum declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumDeclaration {
    pub name: Identifier,
    pub members: Vec<EnumMember>,
    pub span: Span,
}

/// Enum member
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumMember {
    pub name: Identifier,
    pub value: Option<Expression>,
    pub span: Span,
}

/// Namespace declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NamespaceDeclaration {
    pub name: Identifier,
    pub body: Vec<Statement>,
    pub span: Span,
}

/// Import declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportDeclaration {
    pub specifiers: Vec<ImportSpecifier>,
    pub source: String,
    pub span: Span,
}

/// Import specifier
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ImportSpecifier {
    Default(Identifier),
    Named(NamedImportSpecifier),
    Namespace(Identifier),
}

/// Named import specifier
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NamedImportSpecifier {
    pub imported: Identifier,
    pub local: Option<Identifier>,
    pub span: Span,
}

/// Export declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExportDeclaration {
    pub declaration: Option<Box<Declaration>>,
    pub specifiers: Vec<ExportSpecifier>,
    pub source: Option<String>,
    pub span: Span,
}

/// Export specifier
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExportSpecifier {
    Named(NamedExportSpecifier),
    Default(Identifier),
    All,
}

/// Named export specifier
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NamedExportSpecifier {
    pub local: Identifier,
    pub exported: Option<Identifier>,
    pub span: Span,
}