fn first<i32>(v: &Vec<i32>) -> Option<&i32> {
    v.first()
}
fn lcm(a: i32, b: i32) -> i32 {
    return a * b / gcd(a, b);
}
fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 {
        return b;
    }
    return gcd(b % a, a);
}
fn find_gcd(vec: &mut Vec<i32>) -> i32 {
    let rez = first(&vec).unwrap();
    let last = vec.last().unwrap();
    for n in vec.iter() {
        let rez = &gcd(*n, *rez);
        if *rez == 1 {
            return 1;
        }
        if n == last {
            return *rez;
        }
    }
    return *rez;
}

fn find_lcm(vec: &mut Vec<i32>) -> i32 {
    let rez = first(&vec).unwrap();
    let last = vec.last().unwrap();
    for n in vec.iter() {
        let rez = &lcm(*n, *rez);
        if *rez == 1 {
            return 1;
        }
        if n == last {
            return *rez;
        }
    }
    return *rez;
}
fn main() {
    let mut vec = vec![8, 12];
    println!("Didziausias bendras daliklis: {}", find_gcd(&mut vec));
    println!("Maziausias bendras kartotinis: {}", find_lcm(&mut vec));
}
