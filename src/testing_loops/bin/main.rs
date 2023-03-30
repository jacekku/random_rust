use std::io::Error;

struct MyNum {
    val: i32,
}

impl MyNum {
    fn new(val: i32) -> Self {
        MyNum { val }
    }

    fn get_val(&self) -> i32 {
        println!("value is read: {}", self.val);
        self.val
    }
}

fn main() -> Result<(), Error> {
    let mut arr = Vec::new();
    for i in 0..10 {
        arr.push(MyNum::new(i));
    }
    // for v in arr {
    //     v.get_val();
    // }

    let output: &Vec<&MyNum> = &arr
        .iter()
        .filter(|v| v.get_val() > 0)
        .filter(|v| v.get_val() > 2)
        .filter(|v| v.get_val() > 5)
        .filter(|v| v.get_val() > 8)
        .collect();

    Ok(())
}
