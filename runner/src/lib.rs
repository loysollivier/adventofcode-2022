pub fn jobs() -> &'static [(fn(), &'static str)] {
    &[
        (day1a::main, "day1a"),
        (day1b::main, "day1b"),
        (day2a::main, "day2a"),
        (day2b::main, "day2b"),
        (day3a::main, "day3a"),
        (day3b::main, "day3b"),
        (day4a::main, "day4a"),
        (day4b::main, "day4b"),
        (day5a::main, "day5a"),
        (day5b::main, "day5b"),
        (day6a::main, "day6a"),
        (day6b::main, "day6b"),
        (day7a::main, "day7a"),
        (day7b::main, "day7b"),
        (day8a::main, "day8a"),
        (day8b::main, "day8b"),
    ]
}
