pub struct AocInput<'a>(&'a str);

pub struct AocInputBlock(Vec<u64>);

impl<'a> AocInput<'a> {
    pub fn get_sum_block(&self) -> AocInputBlock {
        let dataset = self.0.split("\n\n");

        let mut blk = dataset.map(|v| {
            let t = v.split('\n');
            t.map(|v| v.parse::<u64>().unwrap()).sum::<u64>()
        }).collect::<Vec<u64>>();

        blk.sort_unstable();

        AocInputBlock(blk)
    }
}

impl AocInputBlock {
    pub fn get_answer_one(&self) -> u64 {
        *self.0.iter().last().unwrap()
    }

    pub fn get_answer_two(&self) -> u64 {
        self.0.iter().rev().take(3).sum()
    }
}


fn main() {
    println!("Hello, world!");
}
