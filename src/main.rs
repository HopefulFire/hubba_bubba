use rust_decimal::prelude::*;

fn main()
{
    let n1 = Decimal::new(25_0_000_000_000, 10);
    let n2 = Decimal::new(10_0_000_000_000, 10);
    let n3 = n1 + n2;
    println!("{}", n3);
}
