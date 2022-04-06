use std::io;

#[allow(unused_macros)]
macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim().parse::<$type>().expect("Parsable");
    };
}

#[allow(unused_macros)]
macro_rules! read_vec {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).unwrap();
        let $out = inner
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<$type>().unwrap())
            .collect::<Vec<$type>>();
    };
}

fn main() {
    let t = 0;
    read!(t as usize);

    for _ in 0..t {
        let n = 0;
        read!(n as usize);

        let mut exp: f64 = 0.0;

        for _ in 0..n {
            let input: Vec<f64> = vec![0.0; 4];
            read_vec!(input as f64);

            let (x1, y1, x2, y2) = (input[0], input[1], input[2], input[3]);

            if (x1 == -x2 && y1 == -y2) || (x1 == 0.0 && y1 == 0.0) || (x2 == 0.0 && y2 == 0.0) {
                continue;
            }

            exp +=
                ((x1 * x2 + y1 * y2) / ((x1 * x1 + y1 * y1) * (x2 * x2 + y2 * y2)).sqrt()).acos();
        }

        println!("{:.5}", exp / (2.0 * std::f64::consts::PI));
    }
}
