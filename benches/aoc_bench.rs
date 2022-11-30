extern crate advent_of_code_2022;
#[macro_use]
extern crate criterion;

use criterion::BenchmarkId;
use criterion::Criterion;

#[derive(Clone, Copy)]
struct DayPart(u8, u8, fn(&str) -> advent_of_code_2022::solutions::Solution);

impl std::fmt::Display for DayPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:02} part {}", self.0, self.1)
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let days = 1..=advent_of_code_2022::solutions::MAX_SOLVED_DAY;
    let day_parts: Vec<DayPart> = days
        .clone()
        .flat_map(|d| {
            let mut foo: Vec<DayPart> = vec![];

            let (part_a, part_b) = advent_of_code_2022::solutions::get_solution(d);
            if let Some(part_a) = part_a {
                foo.push(DayPart(d, 1, part_a))
            }
            if let Some(part_b) = part_b {
                foo.push(DayPart(d, 2, part_b))
            }

            foo
        })
        .collect();

    let inputs: Vec<String> = days
        .map(|d| {
            let input_file = format!("day{:0>2}.txt", d);
            let path = std::env::current_dir()
                .unwrap()
                .join("input")
                .join(&input_file);

            println!("Path: {:?}", path);
            std::fs::read_to_string(&path).unwrap()
        })
        .collect();

    let mut group = c.benchmark_group("day");
    for day_part in &day_parts {
        group.bench_with_input(
            BenchmarkId::from_parameter(day_part),
            &day_part,
            |b, &day_part| {
                let file = &inputs[(day_part.0 as usize) - 1];
                b.iter(|| day_part.2(&file));
            },
        );
    }
    group.finish();

    c.bench_function("all", |b| {
        b.iter(|| {
            for day_part in &day_parts {
                let file = &inputs[(day_part.0 as usize) - 1];
                day_part.2(&file);
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
