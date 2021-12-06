use crate::utils;

pub fn run() {
    utils::day(6);
    part1();
    part2();
    utils::close();
}

fn part1() {
    utils::part(1);

    let input = utils::read_lines_to_vec("data/day6.txt");
    let mut population: Vec<u8> = input[0].split(",").map(|s| s.parse().unwrap()).collect();

    // This is the brute force variant
    for _ in 0..80 {
        for i in 0..population.len() {
            if population[i] == 0 {
                population.push(8);
                population[i] = 6;
            } else {
                population[i] -= 1;
            }
        }
    }

    utils::answer(population.len());
}

fn part2() {
    utils::part(2);

    let input = utils::read_lines_to_vec("data/day6.txt");
    let population: Vec<i32> = input[0].split(",").map(|s| s.parse().unwrap()).collect();

    // Smarter solution
    let mut fc = FishCounter{cache: vec![u64::MAX; 300]};

    let day: i32 = 256;
    let answer: u64 = population.iter().map(|n| fc.count((day + (6 - n)) as i32 )).sum();

    utils::answer(answer);
}

struct FishCounter {
    cache: Vec<u64>
}

impl FishCounter {
    fn count(&mut self, day: i32) -> u64 {
        if day <= 0 {
            return 1;
        }

        if self.cache[day as usize] != u64::MAX {
            return self.cache[day as usize];
        }

        let mut sum = 0;
        let mut d = day;
        while d >= 0 {
            let x = self.count(d - 9);
            sum += x;
            d -= 7
        }

        self.cache[day as usize] = sum;
        return sum;
    }
}
