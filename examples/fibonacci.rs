extern crate ketos;

use std::env;

use ketos::{Interpreter};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print!("Usage: fibonacci <n>");
        return;
    }

    let n = &args[1];

    // First, create an interpreter.
    let interp = Interpreter::new();

    // Run some code that defines a function.
    // This will insert the newly defined function into the interpreter scope.
    let code = format!(r#"
        (define (fibonacci n)
            (cond
                ((< n 2) n)
                (else (+ (fibonacci (- n 1)) (fibonacci (- n 2))))
            )
        )
        (println (format "(fibonacci {}) = ~d" (fibonacci {})))
        "#, n, n);
    interp.run_code(&code, None).unwrap();
}
