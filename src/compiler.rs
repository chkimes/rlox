use crate::chunk::*;
use crate::memory::*;
use crate::object::*;
use crate::scanner::*;
use crate::value::*;

use num_derive::FromPrimitive;
use num_derive::ToPrimitive;
use num_traits::ToPrimitive;

pub struct Compiler<'a> {
    parser: Parser<'a>,
    compiling_chunk: &'a mut Chunk,
    heap: Heap,
}

struct Parser<'a> {
    scanner: Scanner<'a>,
    current: Token,
    previous: Token,
    had_error: bool,
    panic_mode: bool,
}

struct ParseRule<'a> {
    prefix: Option<ParseFn<'a>>,
    infix: Option<ParseFn<'a>>,
    precedence: Precedence,
}

type ParseFn<'a> = fn(&mut Compiler<'a>) -> ();

#[derive(FromPrimitive, ToPrimitive)]
enum Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}

impl Precedence {
    fn next(&self) -> Precedence {
        match self {
            Precedence::None => Precedence::Assignment,
            Precedence::Assignment => Precedence::Or,
            Precedence::Or => Precedence::And,
            Precedence::And => Precedence::Equality,
            Precedence::Equality => Precedence::Comparison,
            Precedence::Comparison => Precedence::Term,
            Precedence::Term => Precedence::Factor,
            Precedence::Factor => Precedence::Unary,
            Precedence::Unary => Precedence::Call,
            Precedence::Call => Precedence::Primary,
            Precedence::Primary => Precedence::Primary, // unreachable
        }
    }
}

fn get_rule<'a>(token_type: TokenType) -> ParseRule<'a> {
    match token_type {
        TokenType::LeftParen    => ParseRule { prefix: Some(Compiler::grouping), infix: None,                   precedence: Precedence::None },
        TokenType::Minus        => ParseRule { prefix: Some(Compiler::unary),    infix: Some(Compiler::binary), precedence: Precedence::Term },
        TokenType::Plus         => ParseRule { prefix: None,                     infix: Some(Compiler::binary), precedence: Precedence::Term },
        TokenType::Slash        |
        TokenType::Star         => ParseRule { prefix: None,                     infix: Some(Compiler::binary), precedence: Precedence::Factor },
        TokenType::Number       => ParseRule { prefix: Some(Compiler::number),   infix: None,                   precedence: Precedence::None },
        TokenType::Bang         => ParseRule { prefix: Some(Compiler::unary),    infix: None,                   precedence: Precedence::None },
        TokenType::BangEqual    |
        TokenType::EqualEqual   => ParseRule { prefix: None,                     infix: Some(Compiler::binary), precedence: Precedence::Equality },
        TokenType::Greater      |
        TokenType::GreaterEqual |
        TokenType::Less         |
        TokenType::LessEqual    => ParseRule { prefix: None,                     infix: Some(Compiler::binary), precedence: Precedence::Comparison },
        TokenType::Nil          |
        TokenType::False        |
        TokenType::True         => ParseRule { prefix: Some(Compiler::literal),  infix: None,                   precedence: Precedence::None },
        TokenType::String       => ParseRule { prefix: Some(Compiler::string),   infix: None,                   precedence: Precedence::None },
        _                       => ParseRule { prefix: None,                     infix: None,                   precedence: Precedence::None }
    }
}

impl<'a> Compiler<'a> {
    pub fn new<'c>(source: &'c String, chunk: &'c mut Chunk) -> Compiler<'c> {
        let scanner = Scanner::new(source);
        let parser = Parser::new(scanner);

        Compiler {
            parser: parser,
            compiling_chunk: chunk,
            heap: Heap::new(),
        }
    }

    pub fn compile(&mut self) -> bool {
        self.parser.advance();
        self.expression();
        self.parser.consume(TokenType::EOF, "Expect end of expression");
        self.end();

        return !self.parser.had_error;
    }

    fn grouping(&mut self) {
        self.expression();
        self.parser.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn unary(&mut self) {
        let operator_type = self.parser.previous.token_type;

        // Compile the operand.
        self.parse_precedence(Precedence::Unary);

        // Emit the operator instruction.
        match operator_type {
            TokenType::Minus => self.emit_op(Op::Negate),
            TokenType::Bang => self.emit_op(Op::Not),
            _ => return, // Unreachable.
        }
    }

    fn binary(&mut self) {
        let operator_type = self.parser.previous.token_type;
        let rule = get_rule(operator_type);
        self.parse_precedence(rule.precedence.next());

        match operator_type {
            TokenType::BangEqual    => self.emit_ops(Op::Equal, Op::Not),
            TokenType::EqualEqual   => self.emit_op(Op::Equal),
            TokenType::Greater      => self.emit_op(Op::Greater),
            TokenType::GreaterEqual => self.emit_ops(Op::Less, Op::Not),
            TokenType::Less         => self.emit_op(Op::Less),
            TokenType::LessEqual    => self.emit_ops(Op::Greater, Op::Not),
            TokenType::Plus         => self.emit_op(Op::Add),
            TokenType::Minus        => self.emit_op(Op::Subtract),
            TokenType::Star         => self.emit_op(Op::Multiply),
            TokenType::Slash        => self.emit_op(Op::Divide),
            _ => return, // Unreachable.
        }
    }

    fn literal(&mut self) {
        match self.parser.previous.token_type {
            TokenType::Nil   => self.emit_op(Op::Nil),
            TokenType::False => self.emit_op(Op::False),
            TokenType::True  => self.emit_op(Op::True),
            _ => return, // Unreachable.
        }
    }

    fn string(&mut self) {
        let token = self.parser.previous;
        let mut vec = vec![0; token.length - 2];
        vec.clone_from_slice(&self.parser.scanner.source[token.start + 1..token.start + token.length - 1]);
        let str = String::from_utf8(vec).unwrap();

        let interned = self.intern_string(str);
        self.emit_constant(interned);
    }

    // this is kind of a goofy workaround
    // We have to keep a reference to the string from the compiler so
    // that the pointer remains valid. Ideally we would do this from
    // the VM heap, but we don't exactly have access to the VM heap at this
    // point because the compiler doesn't know about it. The probably
    // correct thing to do here is to generate a Constant type to return
    // from the compiler and then handle the heap in the VM but the lazy
    // option is to jam it onto the compiler since it will live for the
    // lifetime of the VM run method.
    fn intern_string(&mut self, str: String) -> Value {
        Value::Object(Obj::LString(self.heap.manage(str)))
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.parser.advance();
        let prefix_rule = get_rule(self.parser.previous.token_type).prefix;
        match prefix_rule {
            None => self.parser.error("Expect expression."),
            Some(prefix_rule) => prefix_rule(self),
        }

        let prec_byte = ToPrimitive::to_u8(&precedence).unwrap();

        while prec_byte <= ToPrimitive::to_u8(&get_rule(self.parser.current.token_type).precedence).unwrap() {
            self.parser.advance();
            let infix_rule = get_rule(self.parser.previous.token_type).infix;
            match infix_rule {
                Some(infix_rule) => infix_rule(self),
                _ => {}
            }
        }
    }

    fn end(&mut self) {
        self.emit_return();
        if cfg!(feature = "DEBUG_PRINT_CODE") {
            if !self.parser.had_error {
                self.compiling_chunk.disassemble("code");
            }
        }
    }

    fn number(&mut self) {
        let value: f64 = self
            .parser
            .scanner
            .get_lexeme(&self.parser.previous)
            .parse()
            .unwrap();
        self.emit_constant(Value::Number(value));
    }

    fn emit_return(&mut self) {
        self.emit_op(Op::Return);
    }

    fn emit_op(&mut self, op: Op) {
        self.compiling_chunk.write_op(op, self.parser.previous.line);
    }

    fn emit_ops(&mut self, op1: Op, op2: Op) {
        self.compiling_chunk.write_op(op1, self.parser.previous.line);
        self.compiling_chunk.write_op(op2, self.parser.previous.line);
    }

    fn emit_byte(&mut self, byte: u8) {
        self.compiling_chunk.write(byte, self.parser.previous.line);
    }

    fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.compiling_chunk.write(byte1, self.parser.previous.line);
        self.compiling_chunk.write(byte2, self.parser.previous.line);
    }

    fn emit_constant(&mut self, value: Value) {
        self.emit_op(Op::Constant);
        let constant = self.make_constant(value);
        self.emit_byte(constant);
    }

    fn make_constant(&mut self, value: Value) -> u8 {
        let constant = self.compiling_chunk.add_constant(value);
        if constant > u8::MAX as usize {
            self.parser.error("Too many constants in one chunk");
            return 0;
        }

        return constant as u8;
    }
}

impl Parser<'_> {
    fn new<'s>(scanner: Scanner) -> Parser {
        Parser {
            scanner: scanner,
            current: Token::null(),
            previous: Token::null(),
            had_error: false,
            panic_mode: false,
        }
    }

    fn advance(&mut self) {
        self.previous = self.current;

        loop {
            self.current = self.scanner.scan();
            if self.current.token_type != TokenType::Error {
                break;
            }

            self.error_at_current("Unknown token");
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.current.token_type == token_type {
            self.advance();
            return;
        }

        self.error_at_current(message);
    }

    fn error_at_current(&mut self, message: &str) {
        let current = self.current;
        self.error_at(&current, message);
    }

    fn error(&mut self, message: &str) {
        let previous = self.previous;
        self.error_at(&previous, message);
    }

    fn error_at(&mut self, token: &Token, message: &str) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;

        eprint!("[line {}] Error", token.line);

        if token.token_type == TokenType::EOF {
            eprint!(" at end");
        } else if token.token_type == TokenType::Error {
            // Nothing.
        } else {
            eprint!(" at '{}'", self.scanner.get_lexeme(token));
        }

        eprintln!(": {}", message);
        self.had_error = true;
    }
}
