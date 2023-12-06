struct Race(u64,u64);

fn number_of_ways_to_win(time: u64, distance: u64) -> u64 {
    let mut holding_period = 0;
    let mut number_of_ways_to_win  = 0;
    while holding_period < time  {
        let speed = holding_period;
        let remaining_time = time - holding_period;
        let actual_distance = speed * remaining_time;
        if actual_distance > distance {
            number_of_ways_to_win += 1;
        }
        holding_period += 1;
    }
    number_of_ways_to_win
}

fn main() {
    let race1 = Race(62,644);
    let race2 = Race(73,1023);
    let race3 = Race(75,1240);
    let race4 = Race(65,1023);
    let race5 = Race(62737565,644102312401023);
    let ways_race1 = number_of_ways_to_win(race1.0, race1.1);
    let ways_race2 = number_of_ways_to_win(race2.0, race2.1);
    let ways_race3 = number_of_ways_to_win(race3.0, race3.1);
    let ways_race4 = number_of_ways_to_win(race4.0, race4.1);
    let total = ways_race1 * ways_race2 * ways_race3 * ways_race4;
    let ways_race5 = number_of_ways_to_win(race5.0, race5.1);
    println!("the margin of error is {} (submit 1)", total);
    println!("the margin of error is {} (submit 2)", ways_race5);
}


