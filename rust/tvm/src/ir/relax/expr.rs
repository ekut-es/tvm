use tvm_macros::{Object, external};
use tvm_rt::{DataType, NDArray};

use crate::ir::function::BaseFuncNode;
use crate::ir::PrimExpr;
use crate::runtime::String as TString;
use crate::runtime::{Object, ObjectRef};
use crate::runtime::array::Array;
use crate::ir::relay::{ExprNode, Expr};
use crate::ir::span::Span;
use crate::ir::attrs::Attrs;
use crate::runtime::map::Map;


#[repr(C)]
#[derive(Object, Debug)]
#[ref_name = "Id"]
#[type_key = "relax.Id"]
pub struct IdNode {
    pub base: Object,
    pub name_int: TString,
}

#[repr(C)]
#[derive(Object, Debug)]
#[ref_name = "StructInfo"]
#[type_key = "relax.StructInfo"]
pub struct StructInfoNode {
    pub base: Object,
    pub span: Span, 
}

#[repr(C)]
#[derive(Object, Debug)]
#[ref_name = "Call"]
#[type_key = "relax.expr.Call"]
pub struct CallNode {
    pub base: ExprNode,
    pub op: Expr,
    pub args: Array<Expr>,
    pub attrs: Attrs,
    pub sinfo_args: Array<StructInfo>,
}

 
#[repr(C)]
#[derive(Object, Debug)]
#[ref_name = "If"]
#[type_key = "relax.expr.If"]
pub struct IfNode {
    pub base: ExprNode,
    pub cond: Expr,
    pub true_branch: Expr,
    pub false_branch: Expr,
}

#[repr(C)]
#[derive(Object, Debug)]
#[ref_name = "Tuple"]
#[type_key = "relax.expr.Tuple"]
pub struct TupleNode {
    pub base: ExprNode,
    pub fields: Array<Expr>,
}

#[repr(C)]
#[derive(Object, Debug)]
#[ref_name = "TupleGetItem"]
#[type_key = "relax.expr.TupleGetItem"]
pub struct TupleGetItemNode {
    pub base: ExprNode,
    pub tuple: Expr,
    pub index: i32, //TODO:(cgerum): this is an int in c++
}

#[repr(C)]
#[derive(Object, Debug)]
#[ref_name = "LeafExpr"]
#[type_key = "relax.expr.LeafExpr"]
pub struct LeafExprNode {
    pub base: ExprNode,
}

#[repr(C)]
#[derive(Object, Debug)]
#[ref_name = "ShapeExpr"]
#[type_key = "relax.expr.ShapeExprr"]
pub struct  ShapeExprNode {
    pub base: LeafExprNode,
    pub id: Id,
}

#[repr(C)]
#[derive(Object, Debug)]
#[ref_name = "Var"]
#[type_key = "relax.expr.Var"]
pub struct VarNode {
    pub base: LeafExprNode,
    pub vid: Id,
}

#[repr(C)]
#[derive(Object, Debug)]
#[ref_name = "DataflowVar"]
#[type_key = "relax.expr.DataflowVar"]
pub struct DataflowVarNode {
    pub base: VarNode,
}

#[repr(C)]
#[derive(Object, Debug)]
#[ref_name = "Constant"]
#[type_key = "relax.expr.Constant"]
pub struct ConstantNode {
    pub base: LeafExprNode,
    pub data: NDArray,
}

#[repr(C)]
#[derive(Object, Debug)]
#[ref_name = "PrimValue"]
#[type_key = "relax.expr.PrimValue"]
pub struct PrimValueNode {
    pub base: LeafExprNode,
    pub value: PrimExpr,
}

#[repr(C)]
#[derive(Object, Debug)]
#[ref_name = "StringImm"]
#[type_key = "relax.expr.StringImm"]
pub struct StringImmNode {
    pub base: LeafExprNode,
    pub value: TString,
}

#[repr(C)]
#[derive(Object, Debug)]
#[ref_name = "DataTypeImm"]
#[type_key = "relax.expr.DataTypeImm"]
pub struct DataTypeImmNode {
    pub base: LeafExprNode,
    pub value: DataType,
}

#[repr(C)]
#[derive(Object, Debug)]
#[ref_name = "Binding"]
#[type_key = "relax.expr.Binding"]
pub struct BindingNode {
    pub base: Object,
    pub var: Var,
    pub span: Span,
}

#[repr(C)]
#[derive(Object, Debug)]
#[ref_name = "MatchCast"]
#[type_key = "relax.expr.MatchCast"]
pub struct MatchCastNode {
    pub base: BindingNode,
    pub value: Expr,
    pub struct_info: StructInfo,
}	

#[repr(C)]
#[derive(Object, Debug)]
#[ref_name = "VarBinding"]
#[type_key = "relax.expr.VarBinding"]
pub struct VarBindingNode {
    pub base: BindingNode,
    pub value: Expr,
}

#[repr(C)]
#[derive(Object, Debug)]
#[ref_name = "BindingBlock"]
#[type_key = "relax.expr.BindingBlock"]
pub struct BindingBlockNode {
    pub base: BindingNode,
    pub span: Span,
    pub bindings: Array<Binding>,
}

#[repr(C)]
#[derive(Object, Debug)]
#[ref_name = "DataflowBlock"]
#[type_key = "relax.expr.DataflowBlock"]
pub struct DataflowBlockNode {
    pub base: BindingBlockNode,
}

#[repr(C)]
#[derive(Object, Debug)]
#[ref_name = "Function"]
#[type_key = "relax.expr.Function"]
pub struct FunctionNode {
    pub base: BaseFuncNode,
    pub params: Array<Var>,
    pub body: Expr,   
    pub ret_struct_info: StructInfo,
    pub is_pure: bool, //FIXME:(cgerum): this is a bool in c++, is bool correct here?
}

#[repr(C)]
#[derive(Object, Debug)]
#[ref_name = "ExternFunc"]
#[type_key = "relax.expr.ExternFunc"]
pub struct ExternFuncNode {
    pub base: BaseFuncNode,
    pub global_symbol: TString,
}

external! {
    #[name("relax.Call")]
    fn call(op: Expr, args: Array<Expr>, attrs: Attrs, sinfo_args: Array<StructInfo>, span: Span) -> Expr;
    #[name("relax.If")]
    fn if_expr(cond: Expr, true_branch: Expr, false_branch: Expr, span: Span) -> Expr;
    #[name("relax.Tuple")]
    fn tuple(fields: Array<Expr>, span: Span) -> Expr;
    #[name("relax.TupleGetItem")]
    fn tuple_get_item(tuple: Expr, index: i32, span: Span) -> Expr;
    #[name("relax.ShapeExpr")]
    fn shape_expr(values: Array<PrimExpr>, span: Span) -> Expr;

    //FIXME (cgerum): struct_info is an Optional<StructInfo> in c++ 
    #[name("relax.Var")]
    fn var(name_hint: TString, struct_info : StructInfo, span: Span) -> Expr; 
    #[name("relax.VerFromId")]
    fn var_from_id(vid: Id, struct_info: StructInfo, span: Span) -> Expr; 
    #[name("relax.DataflowVar")]
    fn dataflow_var(name_hint: TString, struct_info: StructInfo,  span: Span) -> Expr;
    #[name("relax.DataflowVarFromId")]
    fn dataflow_var_from_id(vid: Id, struct_info: StructInfo, span: Span) -> Expr;
    #[name("relax.Constant")]
    fn constant(data: NDArray, struct_info: StructInfo, span: Span) -> Expr;
    
    #[name("relax.PrimValue")]
    fn prim_value(value: PrimExpr, span: Span) -> Expr;
    #[name("relax.StringImm")]
    fn string_imm(value: TString, span: Span) -> Expr;
    //#[name("relax.DataTypeImm")]
    //fn data_type_imm(value: DataType, span: Span) -> Expr;
   
    #[name("relax.MatchCast")]
    fn match_cast(value: Expr, struct_info: StructInfo, span: Span) -> Expr;
    #[name("relax.VarBinding")]
    fn var_binding(var: Var, value: Expr, span: Span) -> Expr;
    #[name("relax.BindingBlock")]
    fn binding_block(bindings: Array<Binding>, span: Span) -> Expr;
    #[name("relax.DataflowBlock")]
    fn dataflow_block(bindings: Array<Binding>, span: Span) -> Expr;
    #[name("relax.SeqExpr")]
    fn seq_expr(blocks: Array<BindingBlock>, body: Expr, span: Span) -> Expr;
    #[name("relax.Function")]
    fn function(params: Array<Var>, body: Expr, ret_struct_info: StructInfo, is_pure: bool, span: Span) -> Expr;
    #[name("relax.FunctionCreateEmpty")]
    fn function_create_empty(params: Array<Var>, ret_struct_info: StructInfo, is_pure: bool, span: Span) -> Expr;
    #[name("relax.ExternFunc")]
    fn extern_func(global_symbol: TString, span: Span) -> Expr;
    #[name("relax.GetShapeOf")]
    fn get_shape_of(expr: Expr) -> Expr;
    #[name("relax.FuncWithAttr")]
    fn func_with_attr(func: Expr, key: TString, value: ObjectRef) -> Expr;
    #[name("relax.FuncWithAttrs")]
    fn func_with_attrs(func: Expr, attr_map: Map<TString, ObjectRef>) -> Expr;
    #[name("relax.FuncWithoutAttr")]
    fn func_without_attr(func: Expr, key: TString) -> Expr;
}   


// Tests for the bound functions
#[cfg(test)]
mod tests {
    use crate::ir::span::SourceName;
    use tvm_rt::{Device, DeviceType, IsObjectRef};

    use super::*;
    
    #[test]
    fn test_var() {
        let source_name = SourceName::new("test_var".into());
        let span = Span::new(source_name.clone(), 0, 0, 0, 0);
        let struct_info = StructInfo::null();
        let var = var("x".into(), struct_info, span.clone()).unwrap().downcast::<Var>().unwrap();
        assert!(var.vid.name_int == TString::from("x"));
    }

    #[test]
    fn test_const() {
        let source_name = SourceName::new("test_const".into());
        let span = Span::new(source_name.clone(), 0, 0, 0, 0);
        let struct_info = StructInfo::null();
        let dev = Device::new(DeviceType::CPU, 0);
        let dtype = DataType::float(32, 1);
        let shape: Vec<i64> = vec![1, 2, 3];
        let data = NDArray::empty(&shape, dev, dtype);
        let const_expr = constant(data, struct_info, span.clone()).unwrap().downcast::<Constant>().unwrap();
        assert!(const_expr.data.shape()[1] == 2);
    }


    #[test]
    fn test_call() {
        let source_name = SourceName::new("test_call".into());
        let span = Span::new(source_name.clone(), 0, 0, 0, 0);
        let struct_info = StructInfo::null();
        let op = var("op".into(), struct_info, span.clone()).unwrap();
        let args = Array::from_vec(vec![]).unwrap();
        let attrs = Attrs::null();
        let sinfo_args = Array::from_vec(vec![]).unwrap();
        let call = call(op.clone(), args, attrs, sinfo_args, span.clone()).unwrap().downcast::<Call>().unwrap();
        assert!(call.op == op);
    }


    #[test]
    fn test_get_shape_of() {
        let source_name = SourceName::new("test_get_shape_of".into());
        let span = Span::new(source_name.clone(), 0, 0, 0, 0);
        
        let con = constant(NDArray::empty(&vec![1, 2, 3], Device::new(DeviceType::CPU, 0), DataType::float(32, 1)), StructInfo::null(), span.clone()).unwrap();
        let get_shape = get_shape_of(con).unwrap();
        println!("{:?}",get_shape);        


        
    }

}