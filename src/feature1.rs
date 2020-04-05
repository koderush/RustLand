pub fn calculate_fab(n: i32)
{
    let mut a = 1;
    let mut b = 2;
    
    while b <= n
    {
        let c = a + b;
        print!("{} ", c);
        a = b;
        b = c;
    }
}