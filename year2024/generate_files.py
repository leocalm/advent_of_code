def generate_files():
    for day in range(4, 26):
        # print(day)
        print(f"use crate::day_{day}::Day{day};")
#         with open(f"/opt/personal/advent_of_code_2024_rust_new/src/day_{day}.rs", "w") as f:
#             f.write(
#                 """use crate::day::Day;

# pub struct Day{day} {{
#     day_number: u32,
# }}

# impl Day{day} {{
#     pub fn new() -> Day{day} {{
#         Day{day} {{
#             day_number: {day},
#         }}
#     }}
# }}

# impl Day for Day{day} {{
#     fn get_day_number(&self) -> u32 {{ self.day_number }}

#     fn part_1_internal(&mut self, _: &String) -> u64 {{
#         let result: u64 = 0;
#         result
#     }}

#     fn part_2_internal(&mut self, _: &String) -> u64 {{
#         let result: u64 = 0;
#         result
#     }}
# }}
#                 """.format(day=day)
#             )

if __name__ == "__main__":
    generate_files()