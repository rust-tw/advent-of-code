use pan93412_aoc_2022::AocInput;

fn main() {
    let input = AocInput::new(include_str!("../inputs.txt"));
    let blk = input.get_sum_block();

    println!("Answer one: {}", blk.get_answer_one());
    println!("Answer two: {}", blk.get_answer_two());
}
