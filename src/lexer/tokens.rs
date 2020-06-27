#![allow(dead_code)]
#[warn(non_camel_case_types)]
#[derive(Debug)]


pub enum Tokens { 
   TOKEN_NEWLINE, // /n
   TOKEN_RCB {value: String},    // }
   TOKEN_LCB {value: String},    // {
   TOKEN_RRB {value: String},    // )
   TOKEN_RLB {value: String},    // (
   TOKEN_RSB {value: String},    // ]
   TOKEN_LSB {value: String},
   TOKEN_EQ {value: String},    // [
   TOKEN_ID {value: String},      // id for special keyword
   TOKEN_TXT {value: String},
   TOKEN_SXN,     // Sections
   TOKEN_ATR,     // Attribute // Modifier
   TOKEN_CMP,      // Component
   TOKEN_RDR,      // Render
   TOKEN_NULL
} 