type Region = [[i32; 2]; 3];

/// Return the intersection of two regions if there is one
fn intersect(a: &Region, b: &Region) -> Option<Region> {
    let intersection: Region = [
        [a[0][0].max(b[0][0]), a[0][1].min(b[0][1])],
        [a[1][0].max(b[1][0]), a[1][1].min(b[1][1])],
        [a[2][0].max(b[2][0]), a[2][1].min(b[2][1])],
    ];

    // Check if it is still a valid region
    if intersection.iter().all(|[low, high]| low <= high) {
        Some(intersection)
    } else {
        None
    }
}

fn volume(region: &Region) -> usize {
    region
        .iter()
        .fold(1, |acc, &[low, high]| acc * (high - low + 1)) as usize
}

fn cut(a: &Region, b: &Region) -> Vec<Region> {
    let mut new = Vec::new();
    // X axis
    if a[0][0] < b[0][0] {
        new.push([[a[0][0], b[0][0] - 1], a[1], a[2]]);
    }
    if b[0][1] < a[0][1] {
        new.push([[b[0][1] + 1, a[0][1]], a[1], a[2]]);
    }

    // Y axis
    if a[1][0] < b[1][0] {
        new.push([b[0], [a[1][0], b[1][0] - 1], a[2]]);
    }
    if b[1][1] < a[1][1] {
        new.push([b[0], [b[1][1] + 1, a[1][1]], a[2]]);
    }

    // Z axis
    if a[2][0] < b[2][0] {
        new.push([b[0], b[1], [a[2][0], b[2][0] - 1]]);
    }
    if b[2][1] < a[2][1] {
        new.push([b[0], b[1], [b[2][1] + 1, a[2][1]]]);
    }

    new
}

pub fn main() {
    let regions: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|line| {
            let (on, coords) = line.split_once(' ').unwrap();
            let coords: Region = coords
                .splitn(3, ',')
                .map(|range| {
                    let (low, high) = range[2..].split_once("..").unwrap();
                    [low.parse().unwrap(), high.parse().unwrap()]
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            (on == "on", coords)
        })
        .filter(|(_, coords)| coords.iter().flatten().all(|&c| -50 <= c && c <= 50))
        .collect();

    // Regions that have zero overlaps with each other and are on
    let mut no_overlaps = Vec::new();

    for (on, region) in regions {
        let mut cuts = Vec::new();
        no_overlaps.retain(|r| {
            if let Some(intersection) = intersect(&region, &r) {
                cuts.extend(cut(&r, &intersection));
                false
            } else {
                true
            }
        });
        no_overlaps.extend(cuts);
        if on {
            no_overlaps.push(region);
        }
    }

    let total: usize = no_overlaps.iter().map(|r| volume(r)).sum();

    println!("Answear: {}", total);
}
