use std::error::Error;
use std::path::PathBuf;
use common::base_day::BaseDay;
use common::file::get_input_path;

pub struct Day13 {
    file_path: PathBuf,
}

impl Day13 {
    pub fn new() -> Self { Day13 { file_path: get_input_path(2024, 13) } }

    // Extended Euclidean algorithm: returns (gcd, x, y) such that a*x + b*y = gcd
    fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
        if b == 0 {
            (a, 1, 0)
        } else {
            let (g, x1, y1) = Self::extended_gcd(b, a % b);
            (g, y1, x1 - (a / b) * y1)
        }
    }

    // Solve the system:
    // dx1 * a + dx2 * b = px
    // dy1 * a + dy2 * b = py
    // Returns Some(min_tokens) if solution exists with a,b >=0
    fn min_tokens(&self, dx1: i128, dy1: i128, dx2: i128, dy2: i128, px: i128, py: i128) -> Option<i128> {
        // Solve first equation dx1*a + dx2*b = px
        let (g, x0, y0) = Self::extended_gcd(dx1, dx2);
        if px % g != 0 {
            return None; // no solution
        }
        let factor = px / g;
        let mut a0 = x0 * factor;
        let mut b0 = y0 * factor;
        let kx = dx2 / g;
        let ky = dx1 / g;

        // Now second equation: dy1*(a0 + k*kx) + dy2*(b0 - k*ky) = py
        // Solve for k: k*(dy1*kx - dy2*ky) = py - dy1*a0 - dy2*b0
        let coeff = dy1 * kx - dy2 * ky;
        let rhs = py - dy1 * a0 - dy2 * b0;

        if coeff == 0 {
            if rhs != 0 {
                return None; // no solution
            } else {
                // any k works, pick k that makes a0 + k*kx >=0 and b0 - k*ky >=0
                let k_min = (-a0 + kx - 1) / kx;
                let k_max = b0 / ky;
                if k_min > k_max {
                    return None;
                }
                let k = k_min;
                a0 += k * kx;
                b0 -= k * ky;
                return Some(3*a0 + b0);
            }
        }

        if rhs % coeff != 0 {
            return None; // no integer solution
        }

        let k = rhs / coeff;
        let a = a0 + k * kx;
        let b = b0 - k * ky;

        if a < 0 || b < 0 {
            return None;
        }
        Some(3*a + b)
    }
}

impl BaseDay for Day13 {
    fn get_day_number(&self) -> u32 { 13 }

    fn part_1(&mut self) -> Result<String, Box<dyn Error>> {
        Ok(String::new())
    }

    fn part_2(&mut self) -> Result<String, Box<dyn Error>> {
        // Remove empty lines
        let lines: Vec<String> = self.read_file_into_vec()
            .into_iter()
            .filter(|line| !line.trim().is_empty())
            .collect();

        let mut total_tokens: i128 = 0;

        for chunk in lines.chunks(3) {
            if chunk.len() < 3 { continue; } // skip incomplete chunks

            let parse = |s: &String| {
                s.split(|c| c == '=' || c == '+' || c == ',')
                    .filter_map(|x| x.trim().parse::<i128>().ok())
                    .collect::<Vec<i128>>()
            };

            let a = parse(&chunk[0]); // [dx1, dy1]
            let b = parse(&chunk[1]); // [dx2, dy2]
            let p = parse(&chunk[2]); // [px, py]

            // Skip if parsing failed
            if a.len() < 2 || b.len() < 2 || p.len() < 2 { continue; }

            // Part 2: add 10_000_000_000_000
            let px = p[0] + 10_000_000_000_000;
            let py = p[1] + 10_000_000_000_000;

            if let Some(tokens) = self.min_tokens(a[0], a[1], b[0], b[1], px, py) {
                total_tokens += tokens;
            }
        }

        if total_tokens > u64::MAX as i128 {
            Ok(u64::MAX.to_string())
        } else {
            Ok(total_tokens.to_string())
        }
    }

    fn get_input_file_path(&self) -> PathBuf {
        self.file_path.clone()
    }
}
