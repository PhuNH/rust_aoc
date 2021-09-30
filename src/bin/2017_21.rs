use std::collections::{HashSet, HashMap};
use std::fmt::{Display, Formatter};

#[derive(Clone, Eq, PartialEq)]
struct Pattern {
    size: usize,
    pixels: HashSet<usize>,
}

impl Display for Pattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..(self.size.pow(2)) {
            if i % self.size == 0 && i > 0 {
                write!(f, "/")?;
            }
            if self.pixels.contains(&i) { write!(f, "#")?; }
            else { write!(f, ".")?; }
        }
        write!(f, "")
    }
}

impl Pattern {
    fn from(string: &str) -> Pattern {
        let chars: Vec<_> = string.split('/').flat_map(|s| s.chars()).collect();
        let size = (chars.len() as f64).sqrt() as usize;
        let pixels: HashSet<_> = chars.iter().enumerate()
            .filter(|(_, c)| **c == '#')
            .map(|(i, _)| i).collect();
        Pattern { size, pixels }
    }

    fn rotate(&self) -> Pattern {
        let pixels: HashSet<usize> = self.pixels.iter().map(|p| {
            match *p {
                0 => if self.size == 2 { 1 } else { 2 },
                1 => if self.size == 2 { 3 } else { 5 },
                2 => if self.size == 2 { 0 } else { 8 },
                3 => if self.size == 2 { 2 } else { 1 },
                4 => 4,
                5 => 7,
                6 => 0,
                7 => 3,
                _ => 6
            }
        }).collect();
        Pattern { size: self.size, pixels }
    }

    fn flip(&self) -> Pattern {
        let pixels: HashSet<usize> = self.pixels.iter().map(|p| {
            (*p / self.size) * self.size * 2 + (self.size - 1) - *p
        }).collect();
        Pattern { size: self.size, pixels }
    }

    fn xforms(mut self) -> Vec<Pattern> {
        let mut results = Vec::new();
        results.push(self.clone());
        for _ in 0..3 {
            self = self.rotate();
            if !results.contains(&self) {
                results.push(self.clone());
            }
        }
        let flipped: Vec<_> = results.iter().map(|p| p.flip()).collect();
        results.extend(flipped);
        results
    }

    fn divide(&self, size: usize) -> Vec<(usize, Pattern)> {
        let size_by_patterns = self.size / size;
        let patterns_and_indices: Vec<_> = self.pixels.iter().map(|p| {
            let (x_by_patterns, x_in_pattern) = ((*p % self.size) / size, (*p % self.size) % size);
            let (y_by_patterns, y_in_pattern) = ((*p / self.size) / size, (*p / self.size) % size);
            let index_by_patterns = x_by_patterns + y_by_patterns * size_by_patterns;
            let index_in_pattern = x_in_pattern + y_in_pattern * size;
            (index_by_patterns, index_in_pattern)
        }).collect();
        let pattern_indices: HashSet<_> = patterns_and_indices.iter().map(|(i, _)| *i).collect();
        (0..size_by_patterns.pow(2)).into_iter().map(|out_i| {
            let pixels: HashSet<_> = if pattern_indices.contains(&out_i) {
                patterns_and_indices.iter()
                    .filter(|(o_i, _)| *o_i == out_i)
                    .map(|&ii| ii.1)
                    .collect()
            } else { HashSet::new() };
            (out_i, Pattern { size, pixels })
        }).collect()
    }

    fn with_pattern(&mut self, pattern_index: usize, pattern: &Pattern) {
        let size_by_patterns = self.size / pattern.size;
        let (x_by_patterns, y_by_patterns) = (pattern_index % size_by_patterns, pattern_index / size_by_patterns);
        let converted_indices: Vec<_> = pattern.pixels.iter().map(|p| {
            let x = x_by_patterns * pattern.size + *p % pattern.size;
            let y = y_by_patterns * pattern.size + *p / pattern.size;
            x + y * self.size
        }).collect();
        self.pixels.extend(converted_indices);
    }

    fn find_s3_pattern(p: &Pattern, all_xforms: &Vec<S3PatternXforms>, xform_groups: &S3PatternXformGroups) -> usize {
        *xform_groups.groups[p.pixels.len()].iter().find(|&&i| {
            all_xforms[i].xforms.contains(p)
        }).unwrap()
    }

    fn s3_step_on_count(groups: &HashMap<usize, usize>, steps: usize,
                        s3_maps: &Vec<S3Map>, s3_pattern_xforms: &Vec<S3PatternXforms>) -> usize {
        if steps == 1 {
            groups.iter()
                .map(|(&s3_index, count)| s3_maps[s3_index].s4_pattern.pixels.len() * count)
                .sum()
        } else if steps == 2 {
            groups.iter()
                .map(|(&s3_index, count)| s3_maps[s3_index].s6_pattern.pixels.len() * count)
                .sum()
        } else if steps == 3 {
            groups.iter()
                .map(|(&s3_index, count)|
                    s3_maps[s3_index].s3_on_count(s3_pattern_xforms) * count)
                .sum()
        } else {
            let mut sum_groups: HashMap<usize, usize> = HashMap::new();
            groups.iter()
                .for_each(|(&s3_index, count)| {
                    s3_maps[s3_index].s3_groups.iter().for_each(|(&s3_i, c)| {
                        *sum_groups.entry(s3_i).or_insert(0) += *c * *count;
                    });
                });
            Pattern::s3_step_on_count(&sum_groups, steps-3, s3_maps, s3_pattern_xforms)
        }
    }
}

struct S3PatternXforms {
    xforms: Vec<Pattern>,
    on_count: usize,
}

impl S3PatternXforms {
    fn from(p: Pattern) -> S3PatternXforms {
        let on_count = p.pixels.len();
        S3PatternXforms { xforms: p.xforms(), on_count }
    }
}

struct S3PatternXformGroups {
    groups: Vec<Vec<usize>>
}

impl S3PatternXformGroups {
    fn from(pattern_xforms: &Vec<S3PatternXforms>) -> S3PatternXformGroups {
        let groups: Vec<Vec<_>> = (0..9).map(|on_count| {
            pattern_xforms.iter().enumerate()
                .filter(|(_, x)| x.on_count == on_count)
                .map(|(i, _)| i).collect()
        }).collect();
        S3PatternXformGroups { groups }
    }
}

struct S3Map {
    s4_pattern: Pattern,
    s6_pattern: Pattern,
    s3_groups: HashMap<usize, usize>,
}

impl S3Map {
    fn from(s4_pattern: Pattern,
            s2_patterns: &Vec<Vec<Pattern>>, s2_maps: &Vec<Pattern>,
            s3_pattern_xforms: &Vec<S3PatternXforms>, s3_xform_groups: &S3PatternXformGroups) -> S3Map {
        // step 2
        let indexed_s2_4_patterns = s4_pattern.divide(2);
        let mut s6_pattern = Pattern { size: 6, pixels: HashSet::new() };
        indexed_s2_4_patterns.into_iter().for_each(|(pattern_index, pattern)| {
            let s2_index = s2_patterns.iter().enumerate()
                .find(|(_, ps)| ps.contains(&pattern)).unwrap().0;
            s6_pattern.with_pattern(pattern_index, &s2_maps[s2_index]);
        });
        // step 3
        let indexed_s2_6_patterns = s6_pattern.divide(2);
        let mut s3_groups: HashMap<usize, usize> = HashMap::new();
        indexed_s2_6_patterns.into_iter().for_each(|(_, pattern)| {
            let s2_index = s2_patterns.iter().enumerate()
                .find(|(_, ps)| ps.contains(&pattern)).unwrap().0;
            let s3_index = Pattern::find_s3_pattern(&s2_maps[s2_index], s3_pattern_xforms, s3_xform_groups);
            *s3_groups.entry(s3_index).or_insert(0) += 1;
        });
        S3Map { s4_pattern, s6_pattern, s3_groups }
    }

    fn s3_on_count(&self, all_xforms: &Vec<S3PatternXforms>) -> usize {
        self.s3_groups.iter()
            .map(|(&s3_index, count)| all_xforms[s3_index].on_count * count)
            .sum()
    }
}

fn main() {
    let input = aoc::read_input();
    let (mut s2_maps, mut s3_enhancements) = (Vec::new(), Vec::new());
    let mut s2_patterns: Vec<Vec<Pattern>> = Vec::new();
    let mut s3_pattern_xforms: Vec<S3PatternXforms> = Vec::new();

    input.lines().for_each(|l| {
        let mut parts = l.split(" => ");
        let src_pattern = Pattern::from(parts.next().unwrap());
        let dst_pattern = Pattern::from(parts.next().unwrap());
        if src_pattern.size == 2 {
            s2_patterns.push(src_pattern.xforms());
            s2_maps.push(dst_pattern);
        } else {
            s3_pattern_xforms.push(S3PatternXforms::from(src_pattern));
            s3_enhancements.push(dst_pattern);
        }
    });

    let s3_xform_groups = S3PatternXformGroups::from(&s3_pattern_xforms);
    let s3_maps: Vec<_> = s3_enhancements.into_iter().map(|s4_pattern|
        S3Map::from(s4_pattern, &s2_patterns, &s2_maps, &s3_pattern_xforms, &s3_xform_groups)).collect();
    one_and_two(&s3_maps, &s3_pattern_xforms, &s3_xform_groups);
}

fn one_and_two(s3_maps: &Vec<S3Map>, s3_pattern_xforms: &Vec<S3PatternXforms>, s3_xform_groups: &S3PatternXformGroups) {
    let pixels = vec![1, 5, 6, 7, 8].into_iter().collect();
    let image = Pattern { size: 3, pixels };
    let mut groups = HashMap::new();
    let s3_index = Pattern::find_s3_pattern(&image, s3_pattern_xforms, s3_xform_groups);
    groups.insert(s3_index, 1);
    println!("5: {}", Pattern::s3_step_on_count(&groups, 5, s3_maps, s3_pattern_xforms));
    println!("18: {}", Pattern::s3_step_on_count(&groups, 18, s3_maps, s3_pattern_xforms));
}