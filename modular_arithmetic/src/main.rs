use std::fmt::{Display, Formatter};
use std::io::{stdout, Write};

fn take_int() -> i128 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

struct Equation {
    //of form r = nx + mq for some m
    r: i128,
    x: i128,
    q: i128,
    n: i128,
    m: i128,
}

impl Display for Equation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        if self.m >= 0 {
            write!(
                f,
                "{} = {} x {} + {} x {}",
                self.r, self.n, self.x, self.m, self.q
            )
        } else {
            write!(
                f,
                "{} = {} x {} - {} x {}",
                self.r, self.n, self.x, -self.m, self.q
            )
        }
    }
}

impl Equation {
    fn substitute(e2: &Self, e1: &Self) -> Option<Self> {
        //where e2 is the equation "further up"
        if e2.r != e1.q {
            return None;
        }

        let r = e1.r;
        let x = e2.x;
        let n = e1.m * e2.n;
        let m = e1.n + e1.m * e2.m;
        let q = e2.q;

        Some(Self { r, x, q, n, m })
    }
}

fn forward(a: i128, b: i128) -> Vec<i128> {
    match b {
        0 => vec![a],
        _ => vec![vec![a], forward(b, a % b)].concat(),
    }
}

fn backward(v: &mut Vec<i128>) -> Option<Equation> {
    let hcf = v.pop()?;
    let mut q = v.pop()?;
    let mut x = v.pop()?;
    let mut e1 = Equation {
        r: hcf,
        n: 1,
        x,
        q,
        m: -(x / q),
    };

    while let Some(x2) = v.pop() {
        let rn = q;

        let e2 = Equation {
            r: rn,
            n: 1,
            x: x2,
            q: x,
            m: -(x2 / x),
        };

        q = x;
        x = x2;

        e1 = Equation::substitute(&e2, &e1)?;
    }
    Some(e1)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("I am going to solve a conguence of the form Ax ≡ B (mod C)");

    print!("Enter A: ");
    stdout().flush()?;
    let q = take_int();

    print!("Enter B: ");
    stdout().flush()?;
    let mut r = take_int();

    print!("Enter C: ");
    stdout().flush()?;
    let base = take_int();

    if base == 0 {
        eprintln!("This congruence cannot be solved as the base is zero.");
        return Err(Box::<dyn std::error::Error + Send + Sync>::from(
            "Base is zero.".to_string(),
        ));
    } else if base == 1 {
        println!("The congrence holds for all x ∈ ℤ");
        return Ok(());
    }

    if r >= base {
        eprintln!(
            "The remainder {} is greater than the base {}. This equation can never have solutions as p % q is less than q for every p."
            , r, base
        );
        r %= base;
        println!(
            "I will instead attempt to solve the congruence {}x ≡ {} (mod {})",
            q, r, base
        );
    }

    println!();

    let mut f = forward(q, base);
    let bezout = match backward(&mut f) {
        Some(k) => k,
        None => {
            eprintln!("Failed to get equation into Bezout identity form.");
            return Err(Box::<dyn std::error::Error + Send + Sync>::from(
                "Failed backward pass.".to_string(),
            ));
        }
    };

    println!("In Bezout's identity form: {}\n", bezout);

    if r % bezout.r == 0 {
        println!(
            "The solution is x ≡ {} (mod {})",
            (bezout.n * (r / bezout.r)) % base,
            base
        );
    } else {
        println!(
            "There are no solutions because {} is not a multiple of the HCF, which is {}",
            r, bezout.r
        );
    }

    Ok(())
}
