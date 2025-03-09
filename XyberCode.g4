grammar XyberCode;

@header {
  // XyberCode Grammar
}

// Lexer Rules
BEGIN      : '; beg' ;
TASK       : '; task' ;
EXPECT     : '; what is expected' ;
INSTRUCT   : '; explicit detailed instructions' ;
COMMAND    : '; command' ;
END        : '; end' ;

COMMENT    : '#' ~[\r\n]* -> skip ; // Ignore comments
WS         : [ \t\r\n]+ -> skip ; // Ignore whitespace

IDENTIFIER : [a-zA-Z_][a-zA-Z0-9_]* ;
STRING     : '"' (~["\r\n])* '"' ;
NUMBER     : '-'?[0-9]+ ('.' [0-9]+)? ;
BOOLEAN    : 'true' | 'false' ;

OPERATOR   : '+' | '-' | '*' | '/' | '%' ;
ASSIGN     : '=' ;
COMPARE    : '==' | '!=' | '>' | '>=' | '<' | '<=' ;
LOGICAL    : '&&' | '||' | '!' ;
DELIMITER  : ',' | ';' ;

// Parser Rules
program     : statement+ EOF ;
statement   : command | task | expectation | instruction | execution | block ;
block       : BEGIN statement* END ;
command     : COMMAND STRING ;
task        : TASK STRING ;
expectation : EXPECT STRING ;
instruction : INSTRUCT STRING ;
execution   : IDENTIFIER (ASSIGN expression)? DELIMITER ;
expression  : term ((OPERATOR | COMPARE | LOGICAL) term)* ;
term        : NUMBER | STRING | BOOLEAN | IDENTIFIER | '(' expression ')' ;
