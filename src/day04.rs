use std::collections::BTreeMap;

type Passport = BTreeMap<String, String>;

fn parse_passports(passports: &str) -> Vec<Passport> {
    passports.split("\n\n").map(parse_passport).collect()
}

fn parse_passport(passport: &str) -> Passport {
    passport
        .replace('\n', " ")
        .split(' ')
        .map(|p| {
            let mut split = p.split(':');
            let key = split.next().unwrap();
            let value = split.next().unwrap();
            (key.to_string(), value.to_string())
        })
        .collect()
}

fn is_valid_simple(passport: &Passport) -> bool {
    const REQUIRED: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    REQUIRED.iter().all(|r| passport.contains_key(*r))
}

#[aoc_generator(day4)]
fn generator(input: &str) -> Vec<Passport> {
    parse_passports(input)
}

#[aoc(day4, part1)]
pub fn first(input: &[Passport]) -> usize {
    input.iter().filter(|p| is_valid_simple(p)).count()
}

fn is_valid_byr(byr: &str) -> bool {
    byr.len() == 4 && byr.chars().all(|c| c.is_ascii_digit()) && {
        let num = byr.parse().unwrap();
        num >= 1920 && 2002 >= num
    }
}

fn is_valid_iyr(iyr: &str) -> bool {
    iyr.len() == 4 && iyr.chars().all(|c| c.is_ascii_digit()) && {
        let num = iyr.parse().unwrap();
        num >= 2010 && 2020 >= num
    }
}

fn is_valid_eyr(eyr: &str) -> bool {
    eyr.len() == 4 && eyr.chars().all(|c| c.is_ascii_digit()) && {
        let num = eyr.parse().unwrap();
        num >= 2020 && 2030 >= num
    }
}

fn is_valid_hgt(hgt: &str) -> bool {
    if hgt.ends_with("cm") && hgt.len() == 5 && hgt[..3].chars().all(|c| c.is_ascii_digit()) {
        let hgt = hgt[..3].parse().unwrap();
        hgt >= 150 && 193 >= hgt
    } else if hgt.ends_with("in") && hgt.len() == 4 && hgt[..2].chars().all(|c| c.is_ascii_digit())
    {
        let hgt = hgt[..2].parse().unwrap();
        hgt >= 59 && 76 >= hgt
    } else {
        false
    }
}

fn is_valid_hcl(hcl: &str) -> bool {
    hcl.starts_with('#') && hcl[1..].chars().all(|c| c.is_ascii_hexdigit())
}

fn is_valid_ecl(ecl: &str) -> bool {
    matches!(ecl, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
}

fn is_valid_pid(pid: &str) -> bool {
    pid.len() == 9 && pid.chars().all(|c| c.is_ascii_digit())
}

fn is_valid_complex(passport: &Passport) -> bool {
    if let Some(byr) = passport.get("byr") {
        if !is_valid_byr(byr) {
            return false;
        }
    } else {
        return false;
    }

    if let Some(iyr) = passport.get("iyr") {
        if !is_valid_iyr(iyr) {
            return false;
        }
    } else {
        return false;
    }

    if let Some(eyr) = passport.get("eyr") {
        if !is_valid_eyr(eyr) {
            return false;
        }
    } else {
        return false;
    }

    if let Some(hgt) = passport.get("hgt") {
        if !is_valid_hgt(hgt) {
            return false;
        }
    } else {
        return false;
    }

    if let Some(hcl) = passport.get("hcl") {
        if !is_valid_hcl(hcl) {
            return false;
        }
    } else {
        return false;
    }

    if let Some(ecl) = passport.get("ecl") {
        if !is_valid_ecl(ecl) {
            return false;
        }
    } else {
        return false;
    }

    if let Some(pid) = passport.get("pid") {
        if !is_valid_pid(pid) {
            return false;
        }
    } else {
        return false;
    }

    true
}

#[aoc(day4, part2)]
pub fn second(input: &[Passport]) -> usize {
    input.iter().filter(|p| is_valid_complex(p)).count()
}
