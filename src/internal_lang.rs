
/*
* GOAL: Internal implementation of the simply typed lambda calculus, with extensions
* Will be extended, optimized to work with database
*/
use crate::internal_lang::Type::{*};


/*
* CURRENT STATUS:
* Basic Read, write commands
*/

pub(crate) type KeyType = usize;

pub(crate)  enum ImperativeOps<T> {
    Get(KeyType),
    Set(KeyType, T),
    SHUTDOWN
}





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

