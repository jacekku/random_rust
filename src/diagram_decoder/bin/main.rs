use std::io::Error;

fn main() -> Result<(), Error> {
    let mut buf = String::new();
    while let Ok(_) = std::io::stdin().read_line(&mut buf) {
        let input: Vec<char> = buf.chars().collect();

        let windows = input.windows(3);
        let mut skippable = 0;
        for v in windows {
            if skippable > 0 {
                skippable -= 1;
                continue;
            }
            let decoded = match v {
                ['%', '3', 'B'] => (";".to_string(), 2),
                ['%', '3', 'C'] => ("<".to_string(), 2),
                ['%', '3', 'D'] => ("=".to_string(), 2),
                ['%', '3', 'E'] => (">".to_string(), 2),
                ['%', '2', '0'] => (" ".to_string(), 2),
                ['%', '2', '2'] => ("\"".to_string(), 2),
                ['%', '2', 'F'] => ("/".to_string(), 2),
                ['%', '2', 'C'] => (",".to_string(), 2),
                ['%', '5', 'B'] => ("[".to_string(), 2),
                ['%', '5', 'D'] => ("]".to_string(), 2),
                [c, _, _] => (format!("{c}").to_string(), 0),
                _ => ("".to_string(), 0),
            };
            if decoded.1 > 0 {
                skippable += decoded.1;
            }
            print!("{}", decoded.0);
        }
    }

    Ok(())
}
