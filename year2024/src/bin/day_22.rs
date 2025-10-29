use common::file::get_input_path;
use common::{base_day::BaseDay, test_utils::init_logger};
use log::info;
use rusqlite::Connection;
use std::error::Error;
use std::path::PathBuf;

const SECRETS: u128 = 2_000;

fn next_secret(number: u128) -> u128 {
    let mut result = number ^ (number * 64u128) % 16_777_216;
    result = result ^ (result / 32u128) % 16_777_216;
    result = result ^ (result * 2048u128) % 16_777_216;

    result
}

fn day_secret(number: u128) -> u128 {
    let mut secret = number;

    for _ in 0..SECRETS {
        secret = next_secret(secret);
    }

    secret
}

fn day_prices(number: u128) -> Vec<u128> {
    let mut secret = number;
    let mut prices = Vec::new();

    prices.push(secret % 10);

    for _ in 0..SECRETS {
        secret = next_secret(secret);
        prices.push(secret % 10);
    }

    prices
}

fn diffs(prices: &[u128]) -> Vec<i128> {
    prices
        .windows(2)
        .map(|pair| pair[1] as i128 - pair[0] as i128)
        .collect()
}

fn max_bananas(conn: Connection) -> Result<u64, Box<dyn Error>> {
    let mut stmt = conn.prepare("
select qq.diff, sum(qq.price) from (
    select q.diff, q.price as price
    from (
        select sequence.secret, sequence.diff, price.price
        from sequence
        join price on price.price_index = sequence.start_index + 4 and sequence.secret = price.secret
        order by sequence.secret, sequence.start_index
    ) q
    group by q.secret, q.diff
) qq
group by 1
order by 2 desc
limit 1
;
        ")?;

    let result = stmt.query_one([], |row| row.get::<_, i64>(1))?;

    Ok(result as u64)
}

fn create_tables(conn: &Connection) -> Result<(), Box<dyn Error>> {
    conn.execute(
        "CREATE TABLE if not exists sequence (secret INTEGER, start_index INTEGER, diff TEXT)",
        (),
    )?;
    conn.execute(
        "CREATE TABLE if not exists price (secret INTEGER, price_index INTEGER, price INTEGER)",
        (),
    )?;

    Ok(())
}

fn clear_tables(conn: &Connection) -> Result<(), Box<dyn Error>> {
    conn.execute("DELETE FROM sequence;", ())?;
    conn.execute("DELETE FROM price;", ())?;

    Ok(())
}

fn solve(secrets: &[u128]) -> Result<u64, Box<dyn Error>> {
    let mut conn = Connection::open("day_22.sqlite")?;
    create_tables(&conn)?;
    clear_tables(&conn)?;

    let tx = conn.transaction()?;
    {
        let mut insert_into_diff = tx.prepare("INSERT INTO sequence VALUES (?, ?, ?)")?;
        let mut insert_into_price = tx.prepare("INSERT INTO price VALUES (?, ?, ?)")?;

        for &secret in secrets.iter() {
            let day_prices = day_prices(secret);
            let diffs = diffs(&day_prices);

            for start_index in 0..diffs.len() - 4 {
                let seq = format!(
                    "{},{},{},{}",
                    diffs[start_index],
                    diffs[start_index + 1],
                    diffs[start_index + 2],
                    diffs[start_index + 3]
                );
                insert_into_diff.execute((secret as i64, start_index, seq))?;
            }

            for (price_index, &price) in day_prices.iter().enumerate() {
                insert_into_price.execute((secret as i64, price_index, price as i64))?;
            }
        }
    }
    tx.commit()?;

    Ok(max_bananas(conn)?)
}

pub struct Day22 {
    day_number: u32,
    file_path: PathBuf,
}

impl Day22 {
    pub fn new() -> Self {
        Self {
            day_number: 22,
            file_path: get_input_path(2024, 22),
        }
    }
}

impl BaseDay for Day22 {
    fn get_day_number(&self) -> u32 {
        self.day_number
    }

    fn part_1(&mut self) -> Result<String, Box<dyn Error>> {
        let mut result = 0u128;
        for line in self.read_file_into_vec() {
            let secret = day_secret(line.parse::<u128>().unwrap());
            result += secret;
        }

        Ok(result.to_string())
    }

    fn part_2(&mut self) -> Result<String, Box<dyn Error>> {
        Ok(solve(
            &self
                .read_file_into_vec()
                .iter()
                .map(|line| line.parse::<u128>().unwrap())
                .collect::<Vec<u128>>(),
        )?
        .to_string())
    }

    fn get_input_file_path(&self) -> PathBuf {
        self.file_path.clone()
    }
}

#[allow(dead_code)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut day = Day22::new();
    init_logger();
    common::file::download_input_file(2024, day.day_number).await?;

    let result = day.run_day()?;
    info!("Day {} - part 1: {:?}", result.day, result.part_1);
    info!("Day {} - part 2: {:?}", result.day, result.part_2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::Path;

    #[test]
    fn next_secret_test() {
        let expected_results = [
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];
        let mut secret = 123;

        for expected in expected_results {
            let result = next_secret(secret);
            assert_eq!(result, expected);
            secret = result;
        }

        assert_eq!(next_secret(123), 15_887_950);
    }

    #[test]
    fn part_1_1_test() {
        let secret = 1u128;
        let expected = 8_685_429u128;

        let result = day_secret(secret);
        assert_eq!(result, expected);
    }

    #[test]
    fn part_1_2_test() {
        let secret = 10u128;
        let expected = 4_700_978u128;

        let result = day_secret(secret);
        assert_eq!(result, expected);
    }

    #[test]
    fn part_1_3_test() {
        let secret = 100u128;
        let expected = 15_273_692u128;

        let result = day_secret(secret);
        assert_eq!(result, expected);
    }

    #[test]
    fn part_1_4_test() {
        let secret = 2024u128;
        let expected = 8_667_524;

        let result = day_secret(secret);
        assert_eq!(result, expected);
    }

    #[test]
    fn part_1_test() {
        let mut day = Day22::new();
        let expected = "37327623";

        let project_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        let path = project_root.join("data/2024/day_22/example_1.txt");
        day.file_path = path;

        let result = day.part_1();
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn day_prices_test() {
        let expected = [3, 0, 6, 5, 4, 4, 6, 4, 4, 2];

        let prices = day_prices(123u128);
        assert_eq!(prices[0..10], expected);
    }

    #[test]
    fn diffs_test() {
        let prices: [u128; 10] = [3, 0, 6, 5, 4, 4, 6, 4, 4, 2];
        let expected: [i128; 9] = [-3, 6, -1, -1, 0, 2, -2, 0, -2];

        let result: Vec<i128> = diffs(&prices);
        assert_eq!(result, expected);
    }

    #[test]
    fn part_2_test() {
        let expected = 23.to_string();

        let mut day = Day22::new();

        let project_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        let path = project_root.join("data/2024/day_22/example_2.txt");
        day.file_path = path;

        let result = day.part_2();
        assert_eq!(result.unwrap(), expected);
    }
}
