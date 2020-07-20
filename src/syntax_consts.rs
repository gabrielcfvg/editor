use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static!{

    pub static ref C_SYNTAX: HashMap<char, u8> = [('0', 1), ('1', 1), ('2', 1)].iter().cloned().collect();

}