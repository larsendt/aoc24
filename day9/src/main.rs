#![feature(let_chains)]

use aoclib::AocData;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Block {
    Empty,
    File(usize),
}

fn parse_disk(a: &AocData) -> Vec<Block> {
    let mut is_file = true;
    let mut disk = vec![];
    let mut file_id = 0;

    for c in a.input.chars() {
        let nblocks = c.to_digit(10).unwrap();
        for _ in 0..nblocks {
            if is_file {
                disk.push(Block::File(file_id));
            } else {
                disk.push(Block::Empty);
            }
        }

        if is_file {
            file_id += 1;
        }
        is_file = !is_file;
    }

    disk
}

fn compact(disk: &mut [Block]) {
    let mut empty_space_ptr = 0;
    let mut file_block_ptr = disk.len() - 1;

    while empty_space_ptr <= file_block_ptr {
        while let Block::File(_) = disk[empty_space_ptr]
            && empty_space_ptr < disk.len()
        {
            empty_space_ptr += 1;
        }

        while let Block::Empty = disk[file_block_ptr]
            && file_block_ptr >= 0
        {
            file_block_ptr -= 1;
        }

        if empty_space_ptr >= file_block_ptr {
            break;
        }

        // print_disk_with_pointers(disk, empty_space_ptr, file_block_ptr);

        disk[empty_space_ptr] = disk[file_block_ptr];
        disk[file_block_ptr] = Block::Empty;
    }
}

fn compact_files(disk: &mut [Block]) {
    let mut fstart = disk.len() - 1;
    let mut fstop = disk.len() - 1;

    loop {
        while let Block::Empty = disk[fstop]
            && fstop >= 1
        {
            fstop -= 1;
        }

        let file_id = match disk[fstop] {
            Block::Empty => panic!(),
            Block::File(id) => id,
        };

        fstart = fstop;
        while fstart >= 1
            && let Block::File(id) = disk[fstart - 1]
            && id == file_id
        {
            fstart -= 1;
        }

        let mut estart = 0;
        let mut estop = 0;

        while estart < disk.len() && estop < disk.len() {
            while estart < disk.len()
                && let Block::File(_) = disk[estart]
            {
                estart += 1;
            }

            estop = estart;

            while estop < disk.len() - 1
                && let Block::Empty = disk[estop + 1]
            {
                estop += 1;
            }

            // Found a viable set of empty blocks
            if estop - estart >= fstop - fstart {
                break;
            } else {
                // Keep looking
                estart += 1;
            }
        }

        // No place to put this file
        if estart >= disk.len() || estart >= fstart {
            if fstart >= 1 {
                fstop = fstart - 1;
                fstart -= 1;
                continue;
            } else {
                break;
            }
        }

        for fblock in fstart..fstop + 1 {
            disk[estart] = disk[fblock];
            disk[fblock] = Block::Empty;
            estart += 1;
        }

        if fstart >= 1 {
            fstop = fstart - 1;
            fstart -= 1;
        } else {
            break;
        }
    }
}

fn print_disk(disk: &[Block]) {
    for block in disk {
        match block {
            Block::Empty => print!("."),
            Block::File(id) => print!("{}", id),
        }
    }
    println!();
}

fn print_disk_with_pointers(disk: &[Block], empty_block_ptr: usize, file_block_ptr: usize) {
    print_disk(disk);
    for i in 0..disk.len() {
        if i == empty_block_ptr {
            print!("^");
        } else if i == file_block_ptr {
            print!("|");
        } else {
            print!(" ");
        }
    }
    println!();
}

fn get_checksum(disk: &[Block]) -> usize {
    let mut cksum = 0;
    for (i, block) in disk.iter().enumerate() {
        if let Block::File(id) = block {
            cksum += i * id;
        }
    }

    cksum
}

fn main() {
    let a = AocData::new("input.txt").unwrap();

    let mut disk = parse_disk(&a);
    compact(&mut disk);
    let part1 = get_checksum(&disk);

    let mut disk = parse_disk(&a);
    compact_files(&mut disk);
    print_disk(&disk);
    let part2 = get_checksum(&disk);

    dbg!(part1, part2);
}
