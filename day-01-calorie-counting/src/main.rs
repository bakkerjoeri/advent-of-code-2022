use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let groups = input.split("\n\n");
    let groups_by_line = groups.map(|group| group.lines());
    let inventories = groups_by_line.map(|group| group.map(|line| line.parse::<i32>().unwrap()));
    let totaled_inventories = inventories.map(|inventory| inventory.reduce(|sum, item| sum + item));
    let biggest_total =
        totaled_inventories.reduce(|largest, item| if item > largest { item } else { largest });

    println!(
        "The elf who's inventory contains the most calories is carrying {:?} calories",
        biggest_total.unwrap().unwrap()
    )
}
