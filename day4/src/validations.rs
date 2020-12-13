pub fn validate_pid (value: &String) -> bool {
    value.len() == 9 && value.parse::<i32>().is_ok()
}

pub fn validate_byr (value: &String) -> bool {
    let year = value.parse::<i32>().unwrap();
    year >=1920 && year <= 2002
}

pub fn validate_iyr (value: &String) -> bool {
    let year = value.parse::<i32>().unwrap();
    year >=2010 && year <= 2020
}

pub fn validate_eyr (value: &String) -> bool {
    let year = value.parse::<i32>().unwrap();
    year >=2020 && year <= 2030
}

pub fn validate_hcl (value: &String) -> bool {
    let chars: Vec<char> = value.chars().collect();
    chars[0] == '#' && chars.len() == 7 
        && chars[1..].iter()
            .all(|c| {
                let a = *c as u32;
                a >= 48 && a <= 57 || a >= 97 && a <= 102 })
}

pub fn validate_hgt (value: &String) -> bool {
    if value.len() < 3 {
        return false
    }
    let t = value[value.len()-2..].to_string();
    let hgt = value[..value.len()-2].parse::<i32>();
    if hgt.is_err() {
        return false;
    }
    let hgt_v = hgt.unwrap();

    (t == *"cm" && hgt_v >= 150 && hgt_v <= 193)
        || (t == *"in" && hgt_v >= 59 && hgt_v <= 76)
}

pub fn validate_ecl (value: &String) -> bool {
    let valid_ecl: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    valid_ecl.iter().any(|e| e == value)
}