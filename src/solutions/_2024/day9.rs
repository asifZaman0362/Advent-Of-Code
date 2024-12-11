use crate::solutions::*;

//pub struct Solver;

struct Region {
    idx: u32,
    size: u8,
}

fn frag(mut block_data: Vec<u32>) -> usize {
    let mut checksum = 0;
    for i in 0..block_data.len() {
        if block_data[i] == 0 {
            if let Some((pos, found)) = block_data
                .iter()
                .enumerate()
                .rev()
                .find(|&(pos, byte)| *byte != 0 && pos > i)
            {
                block_data[i] = *found;
                block_data[pos] = 0;
            } else {
                break;
            }
        }
        checksum += (block_data[i] - 1) as usize * i;
    }
    checksum
}

fn rearr_blocks(mut block_data: Vec<u32>, file_sizes: Vec<u8>, mut spaces: Vec<Region>) -> usize {
    let n = block_data.len();
    let mut moved = HashSet::<u32>::new();
    let mut i = n - 1;
    let mut checksum = 0;
    while i > 0 {
        let block = block_data[i];
        if block == 0 {
            i -= 1;
        } else {
            let file_size = file_sizes[block as usize];
            if !moved.contains(&block) {
                let mut idx = i - (file_size as usize - 1);
                if let Some(space) = spaces
                    .iter_mut()
                    .find(|found| (found.size >= file_size) && (found.idx as usize) < i)
                {
                    moved.insert(block);
                    for j in 0..file_size {
                        block_data[(space.idx + j as u32) as usize] = block;
                        block_data[i - j as usize] = 0;
                    }
                    idx = space.idx as usize;
                    space.size -= file_size;
                    space.idx += file_size as u32;
                }
                let s = file_size as usize;
                let sm = s * ((s - 1) + 2 * idx) / 2;
                checksum += sm * (block - 1) as usize;
            }
            if i < file_size as usize {
                break;
            }
            i -= file_size as usize;
        }
    }
    checksum
}

/*impl Solution for Solver {
type Answer = usize;
fn solve(input: Input) -> (Self::Answer, Self::Answer) {*/
r#macro::solution!(2024, 9, usize, {
    let mut block_data: Vec<u32> = Vec::with_capacity(1 << 32);
    let mut spaces = Vec::<Region>::with_capacity(1024);
    let mut file_sizes = Vec::<u8>::with_capacity(1024);
    file_sizes.push(0);
    let input = input.lines().collect::<Vec<_>>();
    let mut blocks = 0;
    for (idx, (file, space)) in input[0]
        .as_bytes()
        .chunks(2)
        .map(|entry| (entry[0] - 48, entry.get(1).map_or(0, |x| x - 48)))
        .enumerate()
    {
        (0..file).for_each(|_| block_data.push((idx + 1) as u32));
        (0..space).for_each(|_| block_data.push(0));
        blocks += file as u32;
        spaces.push(Region {
            idx: blocks,
            size: space,
        });
        blocks += space as u32;
        file_sizes.push(file);
    }
    let part1 = frag(block_data.clone());
    let part2 = rearr_blocks(block_data, file_sizes, spaces);
    (part1, part2)
});
