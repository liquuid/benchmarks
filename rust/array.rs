fn main() {
    const N:usize = 100000000;
    //let mut a: [f32; N] = [0.0; N]; 
    //#let mut a: Box<[f64]> = Box::new([0.0;N]);
    let mut a: Vec<f64> = std::iter::repeat(0.0).take(N).collect();
    for i in 0..N {
        //println!("{}", i);
        a[i] = (i % 3) as f64;
    }
    for i in 0..5 {
        println!("{}", a[i]);
    }
}
