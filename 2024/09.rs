const INPUT: &str = include_str!("input");

fn main() {
    let disk = parse_disk();
    println!(
        "{}\n{}",
        fill_free_space(&disk),
        fill_free_space_without_fragmentation(&disk)
    );
}

/// Part two. Moves whole files starting from the highest file
/// ID, to the rightmost free space that fits the file.
///
/// # Arguments:
/// * `disk` - The disk that should be compacted.
///
/// # Returns:
/// The checksum of the compacted disk.
fn fill_free_space_without_fragmentation(disk: &[i32]) -> i64 {
    let mut disk = disk.to_owned();
    let mut hi = disk.len();
    while let Some((file_start, file_size)) = get_file(&disk, hi) {
        if let Some(dest_start) = get_free_spot(&disk, file_size, hi) {
            for i in 0..file_size {
                disk.swap(dest_start + i, file_start + i);
            }
        }
        hi = file_start;
    }

    calculate_checksum(&disk)
}

/// Finds the leftmost free spot of at least `file_size` size.
///
/// # Arguments:
/// `disk` - The disk in which to search for the free spot.
/// `file_size` - The minimum size of the free spot.
/// `limit` - The rightmost block where the search should end.
///
/// # Returns:
///
/// An `Option<usize>`:
/// - `Some(start_index)` if the free spot is found.
///   `start_index` is the index of the first block of the free spot.
/// - `None` if there a free spot of the specified size was not found.
fn get_free_spot(disk: &[i32], file_size: usize, limit: usize) -> Option<usize> {
    let mut i = 0;
    while i < limit {
        if disk[i] == -1 {
            let free_spot_size = disk[i..limit].iter().position(|&block| block != -1)?;
            if free_spot_size >= file_size {
                return Some(i);
            }
            i += free_spot_size;
        } else {
            i += 1;
        }
    }
    None
}

/// Finds the rightmost file before `limit` to be moved.
///
/// # Arguments:
/// `disk` - the disk in which to search for the file.
/// `limit` - the rightmost index from which to begin the search.
///
/// # Returns:
///
/// An `Option<(usize, usize)>` representing the file:
/// - `Some((file_start_index, file_size))` if the file is found.
///   `file_start_index` is the index of the beginning of the file (inclusive), and
///   `file_size` is the total number of blocks of the file starting from
///   `file_start_index`.
/// - `None` if there are no more files.
fn get_file(disk: &[i32], limit: usize) -> Option<(usize, usize)> {
    let file_end = disk[..limit].iter().rposition(|&block| block != -1)?;

    if file_end == 0 {
        return None;
    }

    let file_start = disk[..file_end]
        .iter()
        .rposition(|&block| block != disk[file_end])?;

    Some((file_start + 1, file_end - file_start))
}

/// Part one. Moves file blocks one at a time starting from the highest file
/// ID, to the rightmost free space.
///
/// # Arguments:
/// * `disk` - The disk that should be compacted.
///
/// # Returns:
/// The checksum of the compacted disk.
fn fill_free_space(disk: &[i32]) -> i64 {
    let mut disk = disk.to_owned();

    let (mut lo, mut hi) = (0, disk.len() - 1);
    while lo < hi {
        if disk[lo] != -1 {
            lo += 1
        } else if disk[hi] == -1 {
            hi -= 1
        } else {
            disk.swap(lo, hi);
        }
    }

    calculate_checksum(&disk)
}

/// Adds up the result of multiplying each of the blocks' position with the file
/// ID number it contains. The leftmost block is in position `0`. If a block
/// contains free space, skips it instead.
///
/// # Arguments:
/// - `disk` - The disk on which to calculate the checksum.
fn calculate_checksum(disk: &[i32]) -> i64 {
    disk.iter().enumerate().fold(0, |acc, (i, &digit)| {
        if digit == -1 {
            acc
        } else {
            i as i64 * digit as i64 + acc
        }
    })
}

/// Parses and expands the **disk map**.
///
/// # Arguments:
/// * `disk` - the disk map, such as
///
/// # Returns:
/// The disk transformed from the format `2333133121414131402`
/// to the format `00...111...2...333.44.5555.6666.777.888899`.
fn parse_disk() -> Vec<i32> {
    INPUT
        .trim_end() // remove \n
        .chars()
        .map(|c| c.to_digit(10).expect("input contains only digits") as usize)
        .enumerate()
        .flat_map(|(file_id, size)| {
            let block_value = if file_id.is_multiple_of(2) {
                (file_id / 2).try_into().expect("file IDs fit into i32")
            } else {
                -1
            };
            vec![block_value; size]
        })
        .collect()
}
