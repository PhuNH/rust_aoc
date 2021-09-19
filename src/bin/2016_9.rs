fn main() {
    let input = aoc::read_input();
    let input = input.lines().next().unwrap();
    one(input);
    two(input);
}

fn decompress(input: &str, version: u32) -> usize {
    let open_parts: Vec<&str> = input.splitn(2, '(').collect();
    if open_parts.len() == 1 { input.len() }
    else {
        let close_parts: Vec<&str> = open_parts[1].splitn(2, ')').collect();
        if close_parts.len() == 1 { input.len() }
        else {
            let marker_parts: Vec<&str> = close_parts[0].split('x').collect();
            if marker_parts.len() != 2 { input.len() }
            else {
                let marker_parts: Vec<u32> = marker_parts.into_iter().map(|s| s.parse().unwrap()).collect();
                let (len, rep) = (marker_parts[0], marker_parts[1]);
                let mut compressed_chars: Vec<_> = close_parts[1].chars().collect();
                let still_compressed = compressed_chars.split_off(len as usize);
                if version == 1 {
                    open_parts[0].len() + compressed_chars.len() * rep as usize +
                        decompress(still_compressed.into_iter().collect::<String>().as_str(), version)
                } else {
                    open_parts[0].len() +
                        decompress(compressed_chars.into_iter().collect::<String>().as_str(), version) * rep as usize +
                        decompress(still_compressed.into_iter().collect::<String>().as_str(), version)
                }
            }
        }
    }
}

fn one(compressed: &str) {
    println!("{}", decompress(compressed, 1));
}

fn two(compressed: &str) {
    println!("{}", decompress(compressed, 2));
}