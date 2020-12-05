aoc_day!(2020, 4);

fn answer_one() -> String {
    parse_passports(&input())
        .iter()
        .filter(|p| p.is_valid1())
        .count()
        .to_string()
}

fn answer_two() -> String {
    parse_passports(&input())
        .iter()
        .filter(|p| p.is_valid2().is_some())
        .count()
        .to_string()
}

fn parse_passports(input: &str) -> Vec<Passport> {
    input.split("\n\n").map(|p| Passport::parse(p)).collect()
}

#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn parse(string: &str) -> Passport {
        let mut passport = Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        };

        for part in string.split([' ', '\n'].as_ref()) {
            if part.is_empty() {
                continue;
            }
            let mut subparts = part.split(":");
            let field = subparts.next().unwrap();
            let value = subparts.next().unwrap().to_owned();

            match field {
                "byr" => {
                    passport.byr = Some(value);
                }
                "iyr" => {
                    passport.iyr = Some(value);
                }
                "eyr" => {
                    passport.eyr = Some(value);
                }
                "hgt" => {
                    passport.hgt = Some(value);
                }
                "hcl" => {
                    passport.hcl = Some(value);
                }
                "ecl" => {
                    passport.ecl = Some(value);
                }
                "pid" => {
                    passport.pid = Some(value);
                }
                "cid" => {
                    passport.cid = Some(value);
                }
                _ => unreachable!(),
            }
        }

        passport
    }

    fn is_valid1(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_valid2(&self) -> Option<()> {
        let byr = self.byr.clone()?.parse::<u32>().ok()?;
        if byr < 1920 || byr > 2002 {
            return None;
        }
        let iyr = self.iyr.clone()?.parse::<u32>().ok()?;
        if iyr < 2010 || iyr > 2020 {
            return None;
        }
        let eyr = self.eyr.clone()?.parse::<u32>().ok()?;
        if eyr < 2020 || eyr > 2030 {
            return None;
        }
        let hgt = self.hgt.clone()?;
        let hgt_value = hgt
            .chars()
            .take_while(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<u32>()
            .ok()?;
        let hgt_unit = hgt
            .chars()
            .rev()
            .take_while(|c| c.is_alphabetic())
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>();
        match hgt_unit.as_str() {
            "cm" => {
                if hgt_value < 150 || hgt_value > 193 {
                    return None;
                }
            }
            "in" => {
                if hgt_value < 59 || hgt_value > 76 {
                    return None;
                }
            }
            _ => return None,
        }
        let hcl = self.hcl.clone()?;
        if hcl.len() != 7 || !hcl.starts_with("#") || !hcl[1..].chars().all(|c| c.is_digit(16)) {
            return None;
        }
        if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&self.ecl.clone()?.as_ref())
        {
            return None;
        }
        self.pid.clone()?.parse::<u32>().ok()?;
        if self.pid.clone()?.len() != 9 {
            return None;
        }

        Some(())
    }
}
