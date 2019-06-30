use std::cell::RefCell;
use std::rc::Rc;

type TokenRef = Rc<RefCell<Token>>;

#[derive(Debug, Copy, Clone)]
enum Operator {
    Add,
    Sub,
}

impl Operator {
    fn opcode(&self) -> &str {
        match self {
            Operator::Add => "add",
            Operator::Sub => "sub",
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum TokenKind {
    TkReserved,
    TkNum,
    TkEof,
}

impl Default for TokenKind {
    fn default() -> Self {
        TokenKind::TkEof
    }
}

#[derive(Default, Debug)]
struct Token {
    kind: TokenKind,
    next: Option<TokenRef>,
    val: Option<i64>,
    operator: Option<Operator>,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "kind: {:?}, val: {:?}, operator: {:?}",
            self.kind, self.val, self.operator
        )
    }
}

fn main() -> Result<(), failure::Error> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid argument length.");
        std::process::exit(1);
    }

    let token_head = tokenize(&args[1])?;

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    let mut current_token = token_head;

    if current_token.borrow().kind == TokenKind::TkNum {
        println!("  mov rax, {}", current_token.borrow().val.unwrap());
        let current_token_tmp = current_token.clone();
        current_token = match current_token_tmp.borrow().next.clone() {
            Some(t) => t,
            None => {
                println!("  ret");
                return Ok(());
            }
        };

    } else {
        eprintln!("Invalid token: {:?}", current_token.borrow().kind);
        std::process::exit(1);
    }

    while current_token.borrow().kind != TokenKind::TkEof {
        let current_token_tmp = current_token.clone();
        let next = current_token_tmp.borrow().next.clone().unwrap();

        if current_token.borrow().kind == TokenKind::TkReserved
            && next.borrow().kind == TokenKind::TkNum
        {
            println!(
                "  {} rax, {}",
                current_token.borrow().operator.unwrap().opcode(),
                next.borrow().val.unwrap()
            );
            current_token = next;
        } else if current_token.borrow().kind == TokenKind::TkNum {
            current_token = next;
            continue;
        } else {
            eprintln!(
                "Invalid token sequence: {:?}, {:?}",
                current_token.borrow(),
                next
            );
            std::process::exit(1);
        }
    }

    println!("  ret");

    Ok(())
}

fn tokenize(input: &str) -> Result<TokenRef, failure::Error> {
    let mut current = Rc::new(RefCell::new(Token::default()));
    let head = current.clone();

    let mut num_buf = String::with_capacity(8);
    for c in input.chars() {
        match c {
            '+' => {
                current = create_new_num_token(current.clone(), &num_buf)?;
                current = create_new_reserved_token(current.clone(), Operator::Add);
                num_buf.clear();
            }
            '-' => {
                current = create_new_num_token(current.clone(), &num_buf)?;
                current = create_new_reserved_token(current.clone(), Operator::Sub);
                num_buf.clear();
            }
            num @ '0'..='9' => {
                num_buf.push(num);
            }
            space if space.is_whitespace() => continue,
            _ => return Err(failure::format_err!("Encountered invalid character: {}", c)),
        }
    }

    if num_buf == "" {
        return Err(failure::format_err!("There is no last number"));
    }

    let eof_token = Rc::new(RefCell::new(Token {
        kind: TokenKind::TkEof,
        next: None,
        val: None,
        operator: None,
    }));
    let last_num_token = Rc::new(RefCell::new(Token {
        kind: TokenKind::TkNum,
        next: Some(eof_token),
        val: Some(num_buf.parse()?),
        operator: None,
    }));
    current.borrow_mut().next = Some(last_num_token);

    let next_of_head = head.borrow();
    Ok(next_of_head.next.clone().unwrap())
}

fn create_new_num_token(current: TokenRef, val: &str) -> Result<TokenRef, failure::Error> {
    let token_num = Rc::new(RefCell::new(Token {
        kind: TokenKind::TkNum,
        next: None,
        val: Some(val.parse()?),
        operator: None,
    }));
    current.borrow_mut().next = Some(token_num.clone());

    Ok(token_num)
}

fn create_new_reserved_token(current: TokenRef, operator: Operator) -> TokenRef {
    let token = Rc::new(RefCell::new(Token {
        kind: TokenKind::TkReserved,
        next: None,
        val: None,
        operator: Some(operator),
    }));
    current.borrow_mut().next = Some(token.clone());

    token
}
