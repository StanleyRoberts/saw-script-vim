use std::collections::{HashMap, HashSet};
use std::fmt;


/* The logical and operation as a function on bool */
pub fn bool_and (x:bool, y:bool) -> bool {
    x & y
}

/* The logical and operation as a function on bools in a pair */
pub fn bool_and_pair (xy:(bool,bool)) -> bool {
    xy.0 & xy.1
}

#[repr(C)]
pub struct BoolStruct { fst_bool:bool,snd_bool:bool }

/* The logical and operation as a function on bools in a struct */
pub fn bool_and_struct (xy:BoolStruct) -> bool {
    xy.fst_bool & xy.snd_bool
}

/* A struct containing 2 32-bit values, to test how structs that fit into 1
 * 64-bit value are represented */
pub struct TwoValues(u32,u32);

pub fn mk_two_values (x1:u32,x2:u32) -> TwoValues {
    TwoValues(x1,x2)
}

pub extern fn mk_two_values_extern (x1:u32,x2:u32) -> TwoValues {
    TwoValues(x1,x2)
}

pub fn two_values_proj1 (x:TwoValues) -> u32 {
    match x {
        TwoValues(x1,_) => x1
    }
}

pub extern fn two_values_proj1_extern (x:TwoValues) -> u32 {
    match x {
        TwoValues(x1,_) => x1
    }
}

/* A struct containing 3 32-bit values, to test how structs that fit but don't
 * fill up 2 64-bit values are represented */
pub struct ThreeValues(u32,u32,u32);

pub fn mk_three_values (x1:u32,x2:u32,x3:u32) -> ThreeValues {
    ThreeValues(x1,x2,x3)
}

pub extern fn mk_three_values_extern (x1:u32,x2:u32,x3:u32) -> ThreeValues {
    ThreeValues(x1,x2,x3)
}

pub fn three_values_proj1 (x:ThreeValues) -> u32 {
    match x {
        ThreeValues(x1,_,_) => x1
    }
}

pub extern fn three_values_proj1_extern (x:ThreeValues) -> u32 {
    match x {
        ThreeValues(x1,_,_) => x1
    }
}


/* A struct containing 4 32-bit values, to test how structs that fit into 2
 * 64-bit values are represented */
pub struct FourValues(u32,u32,u32,u32);

pub fn mk_four_values (x1:u32,x2:u32,x3:u32,x4:u32) -> FourValues {
    FourValues(x1,x2,x3,x4)
}

pub extern fn mk_four_values_extern (x1:u32,x2:u32,x3:u32,x4:u32) -> FourValues {
    FourValues(x1,x2,x3,x4)
}

pub fn four_values_proj1 (x:FourValues) -> u32 {
    match x {
        FourValues(x1,_,_,_) => x1
    }
}

pub extern fn four_values_proj1_extern (x:FourValues) -> u32 {
    match x {
        FourValues(x1,_,_,_) => x1
    }
}


/* A struct containing 5 32-bit values, to test how structs that do not quite
 * fit into 2 64-bit values are represented */
pub struct FiveValues(u32,u32,u32,u32,u32);

pub fn mk_five_values (x1:u32,x2:u32,x3:u32,x4:u32,x5:u32) -> FiveValues {
    FiveValues(x1,x2,x3,x4,x5)
}

pub extern fn mk_five_values_extern (x1:u32,x2:u32,x3:u32,x4:u32,x5:u32)
                                     -> FiveValues {
    FiveValues(x1,x2,x3,x4,x5)
}


/* Test if a Result is Ok or Err */
pub fn test_result <'a> (r:&'a Result<u64,u64>) -> bool {
    match r {
        Ok(_) => true,
        Err(_) => false
    }
}

/* Sum of two types; yes, this is like Result, but defined locally so we can
 * make an impl block on it */
#[derive(Clone, Debug, PartialEq)]
pub enum Sum<X,Y> {
    Left (X),
    Right (Y)
}


/***
 *** Some tests for how Rust compiles functions on enums
 ***/

impl Sum<u64,u64> {
    pub fn test_sum_impl (&self) -> bool {
        match self {
            Sum::Left(_) => true,
            Sum::Right(_) => false
        }
    }

    pub fn mk_u64_sum_left (x:u64) -> Self {
        Sum::Left (x)
    }

    pub extern fn mk_u64_sum_left_extern (x:u64) -> Self {
        Sum::Left (x)
    }

    pub fn mk_u64_sum_left_add3 (x:&u64, y:u64, z:u64) -> Self {
        Sum::Left (*x+y+z)
    }
}

pub fn mk_string_sum_left (x:&str) -> Sum<String,u64> {
    Sum::Left (x.to_string())
}

pub fn mk_sum_sum_left (x:Sum<u64,u64>) -> Sum<Sum<u64,u64>,u64> {
    Sum::Left (x)
}

pub extern fn mk_sum_sum_left_extern (x:Sum<u64,u64>) -> Sum<Sum<u64,u64>,u64> {
    Sum::Left (x)
}

pub fn mk_sum_sum_left_asym (x:Sum<u32,u64>) -> Sum<Sum<u32,u64>,u64> {
    Sum::Left (x)
}

pub extern fn mk_sum_sum_left_asym_extern (x:Sum<u32,u64>) -> Sum<Sum<u32,u64>,u64> {
    Sum::Left (x)
}


/* A struct containing a string */
pub struct StrStruct(String);

impl StrStruct {
    /* Constructor for StrStruct */
    pub fn new(name: &str) -> StrStruct {
        StrStruct(name.to_string())
    }
    pub extern fn new_extern(name: &str) -> StrStruct {
        StrStruct(name.to_string())
    }

    /* Accessor for StrStruct */
    pub fn name(&self) -> String {
        match self {
            StrStruct(s) => s.to_string(),
        }
    }
    pub extern fn name_extern(&self) -> String {
        match self {
            StrStruct(s) => s.to_string(),
        }
    }

    /* Version of name that maps to &str */
    pub fn name_str (&self) -> &str {
        match self {
            StrStruct(s) => s.as_str(),
        }
    }

    pub extern fn name_str_extern (&self) -> &str {
        match self {
            StrStruct(s) => s.as_str(),
        }
    }
}

/* A struct with a mix of different field types */
#[derive(Clone, Debug, PartialEq)]
pub struct MixedStruct {
    pub s: String,
    pub i1: u64,
    pub i2: u64,
}

impl MixedStruct {
    pub fn get_s(&self) -> String {
        self.s.clone()
    }

    pub fn get_i1(&self) -> u64 {
        self.i1
    }

    pub fn get_i2(&self) -> u64 {
        self.i2
    }
}

/* A 'true' enum */
#[derive(Clone, Debug, PartialEq)]
pub enum TrueEnum {
    Foo,
    Bar,
    Baz,
}

pub fn cycle_true_enum (te: &TrueEnum) -> TrueEnum {
    match te {
        TrueEnum::Foo => TrueEnum::Bar,
        TrueEnum::Bar => TrueEnum::Baz,
        TrueEnum::Baz => TrueEnum::Foo,
    }
}

impl fmt::Display for TrueEnum {
    fn fmt<'a, 'b>(&'a self, f: &'b mut fmt::Formatter) -> fmt::Result {
        match self {
            TrueEnum::Foo => "Foo".fmt(f),
            TrueEnum::Bar => "Bar".fmt(f),
            TrueEnum::Baz => "Baz".fmt(f),
        }
    }
}


/* A linked list */
#[derive(Clone, Debug, PartialEq)]
#[repr(C,u64)]
pub enum List<X> {
    Nil,
    Cons (X,Box<List<X>>)
}

/* Test if a list is empty */
pub fn list_is_empty (l: &List<u64>) -> bool {
    match l {
        List::Nil => true,
        List::Cons (_,_) => false
    }
}

/* Get the head of a linked list or return an error */
pub fn list_head (l: &List<u64>) -> Box<Sum<u64,()>> {
    match l {
        List::Nil => Box::new(Sum::Right (())),
        List::Cons (x,_) => Box::new(Sum::Left (*x))
    }
}

/* Get the head of a linked list or return an error, in an impl block */
impl List<u64> {
    pub fn list_head_impl (&self) -> Result<u64,()> {
        match self {
            List::Nil => Err (()),
            List::Cons (x,_) => Ok (*x)
        }
    }
}

/* A linked list specialized to 64-bit elements */
#[derive(Clone, Debug, PartialEq)]
#[repr(C,u64)]
pub enum List64 {
    Nil64,
    Cons64 (u64,Box<List64>)
}

/* Test if a List64 is empty */
pub fn list64_is_empty (l: &List64) -> bool {
    match l {
        List64::Nil64 => true,
        List64::Cons64 (_,_) => false
    }
}

/* Insert a mapping into m from the greatest of x and y to the other */
pub fn hash_map_insert_gt_to_le (m: &mut HashMap<u64,u64>, x:u64, y:u64) -> () {
    if x > y {
        m.insert (x, y);
    } else {
        m.insert (y, x);
    }
}

/* A binary tree */
pub enum BinTree<X> {
    BinLeaf (X),
    BinNode (Box<BinTree<X>>,Box<BinTree<X>>)
}

pub fn bintree_is_leaf (t: &BinTree<u64>) -> bool {
    match *t {
        BinTree::BinLeaf (_) => true,
        BinTree::BinNode (_,_) => false
    }
}


/* A tree whose internal nodes contain vectors of children */
pub enum Tree<X> {
    Leaf (X),
    Node (Vec<Tree<X>>)
}

pub fn tree_is_leaf (t: &Tree<u64>) -> bool {
    match *t {
        Tree::Leaf (_) => true,
        Tree::Node (_) => false
    }
}

/* Sum all leaves in a tree */
/*
pub fn tree_sum (t: &Tree<u64>) -> u64 {
    let mut acc = 0;
    match *t {
        Tree::Leaf (x) => x,
        Tree::Node (children) =>
            for u in children {
                acc += tree_sum (u);
            }
            acc;
    }
}
*/

/* A 20-element enum that just wraps around type X */
#[repr(u64)]
pub enum Enum20<X> {
  Enum20_0(X),
  Enum20_1(X),
  Enum20_2(X),
  Enum20_3(X),
  Enum20_4(X),
  Enum20_5(X),
  Enum20_6(X),
  Enum20_7(X),
  Enum20_8(X),
  Enum20_9(X),
  Enum20_10(X),
  Enum20_11(X),
  Enum20_12(X),
  Enum20_13(X),
  Enum20_14(X),
  Enum20_15(X),
  Enum20_16(X),
  Enum20_17(X),
  Enum20_18(X),
  Enum20_19(X),
}

pub fn enum20_list_proj<'a> (x:&'a Enum20<List<u64>>) -> &'a List<u64> {
  match x {
      Enum20::Enum20_0(l) => l,
      Enum20::Enum20_1(l) => l,
      Enum20::Enum20_2(l) => l,
      Enum20::Enum20_3(l) => l,
      Enum20::Enum20_4(l) => l,
      Enum20::Enum20_5(l) => l,
      Enum20::Enum20_6(l) => l,
      Enum20::Enum20_7(l) => l,
      Enum20::Enum20_8(l) => l,
      Enum20::Enum20_9(l) => l,
      Enum20::Enum20_10(l) => l,
      Enum20::Enum20_11(l) => l,
      Enum20::Enum20_12(l) => l,
      Enum20::Enum20_13(l) => l,
      Enum20::Enum20_14(l) => l,
      Enum20::Enum20_15(l) => l,
      Enum20::Enum20_16(l) => l,
      Enum20::Enum20_17(l) => l,
      Enum20::Enum20_18(l) => l,
      Enum20::Enum20_19(l) => l,
  }
}

/* A non-empty list type with 20 separate constructors */
#[repr(u64)]
pub enum List10<X> {
  List10Head(X),
  List10_0(X,Box<List10<X>>),
  List10_1(X,Box<List10<X>>),
  List10_2(X,Box<List10<X>>),
  List10_3(X,Box<List10<X>>),
  List10_4(X,Box<List10<X>>),
  List10_5(X,Box<List10<X>>),
  List10_6(X,Box<List10<X>>),
  List10_7(X,Box<List10<X>>),
  List10_8(X,Box<List10<X>>),
  List10_9(X,Box<List10<X>>),
}

pub fn list10_head<'a> (x:&'a List10<List<u64>>) -> &'a List<u64> {
  match x {
      List10::List10Head(l) => l,
      List10::List10_0(l,_) => l,
      List10::List10_1(l,_) => l,
      List10::List10_2(l,_) => l,
      List10::List10_3(l,_) => l,
      List10::List10_4(l,_) => l,
      List10::List10_5(l,_) => l,
      List10::List10_6(l,_) => l,
      List10::List10_7(l,_) => l,
      List10::List10_8(l,_) => l,
      List10::List10_9(l,_) => l,
  }
}


/* A non-empty list type with 20 separate constructors */
#[repr(u64)]
pub enum List20<X> {
  List20Head(X),
  List20_0(X,Box<List20<X>>),
  List20_1(X,Box<List20<X>>),
  List20_2(X,Box<List20<X>>),
  List20_3(X,Box<List20<X>>),
  List20_4(X,Box<List20<X>>),
  List20_5(X,Box<List20<X>>),
  List20_6(X,Box<List20<X>>),
  List20_7(X,Box<List20<X>>),
  List20_8(X,Box<List20<X>>),
  List20_9(X,Box<List20<X>>),
  List20_10(X,Box<List20<X>>),
  List20_11(X,Box<List20<X>>),
  List20_12(X,Box<List20<X>>),
  List20_13(X,Box<List20<X>>),
  List20_14(X,Box<List20<X>>),
  List20_15(X,Box<List20<X>>),
  List20_16(X,Box<List20<X>>),
  List20_17(X,Box<List20<X>>),
  List20_18(X,Box<List20<X>>),
  List20_19(X,Box<List20<X>>),
}

pub fn list20_head<'a> (x:&'a List20<List<u64>>) -> &'a List<u64> {
  match x {
      List20::List20Head(l) => l,
      List20::List20_0(l,_) => l,
      List20::List20_1(l,_) => l,
      List20::List20_2(l,_) => l,
      List20::List20_3(l,_) => l,
      List20::List20_4(l,_) => l,
      List20::List20_5(l,_) => l,
      List20::List20_6(l,_) => l,
      List20::List20_7(l,_) => l,
      List20::List20_8(l,_) => l,
      List20::List20_9(l,_) => l,
      List20::List20_10(l,_) => l,
      List20::List20_11(l,_) => l,
      List20::List20_12(l,_) => l,
      List20::List20_13(l,_) => l,
      List20::List20_14(l,_) => l,
      List20::List20_15(l,_) => l,
      List20::List20_16(l,_) => l,
      List20::List20_17(l,_) => l,
      List20::List20_18(l,_) => l,
      List20::List20_19(l,_) => l,
  }
}
