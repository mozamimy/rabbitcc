enum Operator {
    Mov,
    Add,
    Sub,
}

impl Operator {
    fn opcode(&self) -> &str {
        match self {
            Operator::Add => "add",
            Operator::Sub => "sub",
            Operator::Mov => "mov",
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid argument length.");
        std::process::exit(1);
    }

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    let mut num_buf = String::with_capacity(8);
    let mut operator = Operator::Mov;
    for (i, c) in args[1].chars().enumerate() {
        if i == 0 && !('0'..='9').contains(&c) {
            eprintln!("Unexpected character: {}", c);
            std::process::exit(1);
        }

        match c {
            '+' => {
                println!("  {} rax, {}", operator.opcode(), num_buf);
                num_buf.clear();
                operator = Operator::Add;
            }
            '-' => {
                println!("  {} rax, {}", operator.opcode(), num_buf);
                num_buf.clear();
                operator = Operator::Sub;
            }
            num @ '0'..='9' => {
                num_buf.push(num);
            }
            unexpected @ _ => {
                eprintln!("Unexpected character: {}", unexpected);
                std::process::exit(1);
            }
        }

        if i == args[1].len() - 1 {
            println!("  {} rax, {}", operator.opcode(), num_buf);
        }
    }
    println!("  ret");
}
