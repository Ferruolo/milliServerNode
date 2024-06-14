
/*
* Internal implementation of the simply typed lambda calculus, with extensions
* Will be extended, optimized to work with database
*/

use crate::internal_lang::Type::{*};

enum Type {
    Unit,
    Integer(i32), // Possibly too big on the i32?
    Float(f32),   // Does rust have BFloat16
    Boolean(bool),
    Arrow(Type, Type),
    Prod(Type, Type),
    Sum(Type, Type),
    Var(char),
    Rec(Type)
}

enum Expressions {
    // Values
    Unit,             // () trivial
    Int(i32),         // Integer Expression
    Float(f32),       // Floating Point
    Bool(bool),
    // Math Expressions
    Neg(Expressions), // returns -(expression)
    Plus(Expressions, Expressions), // returns e_1 + e_2
    Minus(Expressions, Expressions), // returns e_1 - e_2
    Times(Expressions, Expressions), // returns e_1 * e_2
    Divide(Expressions, Expressions), // returns e_1 / e_2
    Mod(Expressions, Expressions),    // returns e_1 % e_2
    // Boolean Expressions
    Lt(Expressions, Expressions), // This should be self explanatory
    Eq(Expressions, Expressions),
    Gt(Expressions, Expressions),
    LTE(Expressions, Expressions),
    GTE(Expressions, Expressions),
    And(Expressions, Expressions),
    Or(Expressions, Expressions),
    If(Expressions, Expressions),
    // Actual Expressions
    Var(char),
    Let(char, Expressions, Expressions),
    Ap(Expressions, Expressions),
    //Product Type
    Pair(Expressions, Expressions),
    LetPair(char, char, Expressions, Expressions),
    // Sum Type
    PrjL(Expressions),
    PrjR(Expressions),
    InjL(Type, Expressions),
    InjR(Type, Expressions),
    Case(Expressions, Expressions, Expressions),
    // Recursion
    Roll(Expressions),
    Unroll(Expressions)
}

union OpType {
    value: Type, mutations: Expressions
}

fn subst(e_1: Expressions, a: Expressions, e_2: Expressions) { // Not sure how to do this one rn

}


fn compile(e: Expressions) -> Type {
    match e {
        Expressions::Unit => {
            return Unit
        }
        Expressions::Int(i) => {return Integer(i)}
        Expressions::Float(f) => {
            return Float(f)
        }
        Expressions::Bool(b) => {
            return Boolean(b)
        }
        Expressions::Neg(e) => {

        }
        Expressions::Plus(_, _) => {}
        Expressions::Minus(_, _) => {}
        Expressions::Times(_, _) => {}
        Expressions::Divide(_, _) => {}
        Expressions::Mod(_, _) => {}
        Expressions::Lt(_, _) => {}
        Expressions::Eq(_, _) => {}
        Expressions::Gt(_, _) => {}
        Expressions::LTE(_, _) => {}
        Expressions::GTE(_, _) => {}
        Expressions::And(_, _) => {}
        Expressions::Or(_, _) => {}
        Expressions::If(_, _) => {}
        Expressions::Var(_) => {}
        Expressions::Let(_, _, _) => {}
        Expressions::Ap(_, _) => {}
        Expressions::Pair(_, _) => {}
        Expressions::LetPair(_, _, _, _) => {}
        Expressions::PrjL(_) => {}
        Expressions::PrjR(_) => {}
        Expressions::InjL(_, _) => {}
        Expressions::InjR(_, _) => {}
        Expressions::Case(_, _, _) => {}
        Expressions::Roll(_) => {}
        Expressions::Unroll(_) => {}
    }
}


