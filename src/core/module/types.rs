use std::path::PathBuf;
use std::sync::Arc;
use crate::core::{ModuleId, Symbol};

#[derive(Debug, Clone)]
pub struct Export {
    pub name: Symbol,
    pub visibility: Visibility,
    pub kind: ExportKind,
    pub docs: Option<String>,
    pub deprecated: bool,
}

#[derive(Debug, Clone)]
pub enum ExportKind {
    Function(FunctionExport),
    Type(TypeExport),
    Constant(ConstantExport),
    Module(ModuleExport),
}

#[derive(Debug, Clone)]
pub struct FunctionExport {
    pub signature: String,
    pub is_public: bool,
    pub is_async: bool,
    pub is_unsafe: bool,
}

#[derive(Debug, Clone)]
pub struct TypeExport {
    pub definition: String,
    pub is_public: bool,
    pub type_params: Vec<String>,
    pub constructors: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ConstantExport {
    pub type_: String,
    pub value: String,
    pub is_configurable: bool,
}

#[derive(Debug, Clone)]
pub struct ModuleExport {
    pub path: PathBuf,
    pub version: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Import {
    pub module: ModuleId,
    pub name: Symbol,
    pub alias: Option<Symbol>,
    pub visibility: Visibility,
    pub is_reexport: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Private,
    Protected,
    Internal,
}

#[derive(Debug)]
pub struct SymbolData {
    pub name: Symbol,
    pub kind: SymbolKind,
    pub location: SourceLocation,
    pub docs: Option<String>,
}

#[derive(Debug)]
pub enum SymbolKind {
    Function(Arc<FunctionData>),
    Type(Arc<TypeData>),
    Constant(Arc<ConstantData>),
    Module(Arc<ModuleData>),
}

#[derive(Debug)]
pub struct FunctionData {
    pub signature: String,
    pub body: Option<String>,
    pub attributes: Vec<String>,
}

#[derive(Debug)]
pub struct TypeData {
    pub definition: String,
    pub constructors: Vec<String>,
    pub methods: Vec<String>,
}

#[derive(Debug)]
pub struct ConstantData {
    pub type_: String,
    pub value: String,
}

#[derive(Debug)]
pub struct ModuleData {
    pub path: PathBuf,
    pub exports: HashSet<Symbol>,
}

#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
    pub length: usize,
} 
use std::sync::Arc;
use crate::core::{ModuleId, Symbol};

#[derive(Debug, Clone)]
pub struct Export {
    pub name: Symbol,
    pub visibility: Visibility,
    pub kind: ExportKind,
    pub docs: Option<String>,
    pub deprecated: bool,
}

#[derive(Debug, Clone)]
pub enum ExportKind {
    Function(FunctionExport),
    Type(TypeExport),
    Constant(ConstantExport),
    Module(ModuleExport),
}

#[derive(Debug, Clone)]
pub struct FunctionExport {
    pub signature: String,
    pub is_public: bool,
    pub is_async: bool,
    pub is_unsafe: bool,
}

#[derive(Debug, Clone)]
pub struct TypeExport {
    pub definition: String,
    pub is_public: bool,
    pub type_params: Vec<String>,
    pub constructors: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ConstantExport {
    pub type_: String,
    pub value: String,
    pub is_configurable: bool,
}

#[derive(Debug, Clone)]
pub struct ModuleExport {
    pub path: PathBuf,
    pub version: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Import {
    pub module: ModuleId,
    pub name: Symbol,
    pub alias: Option<Symbol>,
    pub visibility: Visibility,
    pub is_reexport: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Private,
    Protected,
    Internal,
}

#[derive(Debug)]
pub struct SymbolData {
    pub name: Symbol,
    pub kind: SymbolKind,
    pub location: SourceLocation,
    pub docs: Option<String>,
}

#[derive(Debug)]
pub enum SymbolKind {
    Function(Arc<FunctionData>),
    Type(Arc<TypeData>),
    Constant(Arc<ConstantData>),
    Module(Arc<ModuleData>),
}

#[derive(Debug)]
pub struct FunctionData {
    pub signature: String,
    pub body: Option<String>,
    pub attributes: Vec<String>,
}

#[derive(Debug)]
pub struct TypeData {
    pub definition: String,
    pub constructors: Vec<String>,
    pub methods: Vec<String>,
}

#[derive(Debug)]
pub struct ConstantData {
    pub type_: String,
    pub value: String,
}

#[derive(Debug)]
pub struct ModuleData {
    pub path: PathBuf,
    pub exports: HashSet<Symbol>,
}

#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
    pub length: usize,
}