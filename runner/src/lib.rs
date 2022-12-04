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
    ]
}
