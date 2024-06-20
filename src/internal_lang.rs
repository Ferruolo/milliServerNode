/*
* GOAL: Internal implementation of the simply typed lambda calculus, with extensions
* Will be extended, optimized to work with database
*/

/*
* CURRENT STATUS:
* Basic Read, write commands
*/
pub type FakeDatum = u64;


pub type KeyType = usize;

pub enum ImperativeOps<T> {
    Get(KeyType),
    Set(KeyType, T),
    SHUTDOWN,
}
