// Tokens

const T_INT:String = "INT"
const T_LBR:String = "LBR"
const T_RBR:String = "RBR"

struct Token    {
    type:String,
    value:String
}

struct Lexer    {
    text:String
    post:String = -1
    
}