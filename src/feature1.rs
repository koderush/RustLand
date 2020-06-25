pub fn calculate_fab(n: i32)
{
    let mut a = 1;
    let mut b = 2;
    
    while b <= n
    {
        a = a + b;
        print!("{} ", a);
        a ^= b;
        b ^= a;
        a ^= b;
    }

    println!();
}
