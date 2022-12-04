use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let groups = input.split("\n\n");
    let groups_by_line = groups.map(|group| group.lines());
    let inventories = groups_by_line.map(|group| group.map(|line| line.parse::<i32>().unwrap()));
    let totaled_inventories = inventories.map(|inventory| inventory.sum::<i32>());
    let biggest_total = totaled_inventories.max().unwrap();

    println!(
        "The elf who's inventory contains the most calories is carrying {:?} calories",
        biggest_total
    );
}
