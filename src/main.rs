use const_eval::ConstEvaluator;
use parser::Parser;

mod const_eval;
mod lexer;
mod parser;

fn main() {
    let source = r#"
        let a = 2 * 4;
        { let b = a + 3 * 2; }
        let c = a + b;
        { let b = c / a; }
    "#;

    let mut parser = Parser::new(source);
    let ast = parser.parse();

    let evaluator = ConstEvaluator::new();
    let const_values = evaluator.eval(&ast);

    println!("{const_values:#?}");
}
