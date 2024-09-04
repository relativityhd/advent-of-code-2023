use rayon::prelude::*;
use std::{cmp::Ordering, iter::zip, ops::Range};

advent_of_code::solution!(5);

#[derive(Debug, PartialEq)]
struct SeedRange {
    start: usize,
    end: usize,
    length: usize,
}

impl SeedRange {
    fn new(start: usize, length: usize) -> SeedRange {
        SeedRange {
            start: start,
            end: start + length,
            length: length,
        }
    }
}

impl Clone for SeedRange {
    fn clone(&self) -> Self {
        SeedRange {
            start: self.start,
            end: self.end,
            length: self.length,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct MapRange {
    source: Range<usize>,
    destination: Range<usize>,
    length: usize,
    distance: isize,
}

impl MapRange {
    fn new(source: usize, destination: usize, length: usize) -> MapRange {
        MapRange {
            source: source..source + length,
            destination: destination..destination + length,
            length: length,
            distance: destination as isize - source as isize,
        }
    }

    fn get(&self, value: usize) -> Option<usize> {
        if self.source.contains(&value) {
            Some((value as isize + self.distance) as usize)
        } else {
            None
        }
    }

    fn get_range(&self, range: &SeedRange) -> Option<SeedRange> {
        // Check the four cases:
        // 1. Range is completely outside of this map range
        // 2. Range is completely inside of this map range
        // 3. Range starts inside of this map range
        // 4. Range ends inside of this map range

        if range.end < self.source.start || range.start > self.source.end {
            // Case 1
            None
        } else if range.start >= self.source.start && range.end <= self.source.end {
            // Case 2
            Some(SeedRange::new(
                (range.start as isize + self.distance) as usize,
                range.length,
            ))
        } else if range.start >= self.source.start {
            // Case 3
            Some(SeedRange::new(
                (range.start as isize + self.distance) as usize,
                self.source.end - range.start,
            ))
        } else {
            // Case 4
            Some(SeedRange::new(
                self.destination.start,
                range.end - self.source.start,
            ))
        }
    }
}

impl Ord for MapRange {
    fn cmp(&self, other: &Self) -> Ordering {
        self.source.start.cmp(&other.source.start)
    }
}

impl PartialOrd for MapRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn fill_gaps(map: &mut Vec<MapRange>) {
    // Panic if map is empty
    if map.is_empty() {
        panic!("Map is empty");
    }

    // Sort the map
    map.sort_by(|a, b| a.cmp(&b));

    // Fill gap between 0 and first range
    if map[0].source.start > 0 {
        map.insert(0, MapRange::new(0, 0, map[0].source.start));
    }

    // Fill the gaps
    let mut current: &MapRange;
    let mut next: &MapRange;
    for i in 0..map.len() - 1 {
        current = &map[i];
        next = &map[i + 1];
        if current.source.end + 1 < next.source.start {
            map.insert(
                i + 1,
                MapRange::new(
                    current.source.end,
                    current.source.end,
                    next.source.start - current.source.end,
                ),
            );
        }
    }

    // Fill gap between last range and MAX
    let last = map[map.len() - 1].source.end;
    if last < usize::MAX {
        map.push(MapRange::new(last, last, usize::MAX - last));
    }
}

fn reduce_ranges(mut ranges: Vec<SeedRange>) -> Vec<SeedRange> {
    ranges.sort_by(|a, b| a.start.cmp(&b.start));

    let mut result = Vec::new();
    let mut current;
    let mut prev;
    for i in 0..ranges.len() {
        current = &ranges[i];
        if i == 0 {
            result.push(current.clone());
            continue;
        }
        prev = result.last().unwrap();

        if prev.end >= current.start {
            // Remove prev from result and insert merged range
            let new_range = SeedRange::new(prev.start, current.end - prev.start);
            result.pop();
            result.push(new_range);
        } else {
            result.push(current.clone());
        }
    }
    result
}

fn get_from_map(map: &Vec<MapRange>, value: usize) -> Option<usize> {
    for map_range in map {
        if let Some(result) = map_range.get(value) {
            return Some(result);
        }
    }
    None
}

fn get_ranges_from_map(map: &Vec<MapRange>, range: &SeedRange) -> Vec<SeedRange> {
    let mut result = Vec::new();
    let mut used_maps = Vec::new();

    for map_range in map {
        if let Some(new_range) = map_range.get_range(range) {
            result.push(new_range);
            used_maps.push(map_range);
        }
    }

    if result.is_empty() {
        result.push(range.clone());
        return result;
    }

    // Check if there are gap between the ranges
    let mut current_start = range.start;
    let mut gap_result = Vec::new();

    for (new_range, used_map) in zip(result.iter(), used_maps.iter()) {
        let orig_start = (new_range.start as isize - used_map.distance) as usize;
        let orig_end = orig_start + new_range.length;
        if orig_start > current_start {
            gap_result.push(SeedRange::new(current_start, orig_start - current_start));
        }
        current_start = orig_end;
    }

    if current_start < range.end {
        gap_result.push(SeedRange::new(current_start, range.end - current_start));
    }

    result.append(&mut gap_result);
    result = reduce_ranges(result);
    result
}

struct Maps {
    seed_soil: Vec<MapRange>,
    soil_fertilizer: Vec<MapRange>,
    fertilizer_water: Vec<MapRange>,
    water_light: Vec<MapRange>,
    light_temperature: Vec<MapRange>,
    temperature_humidity: Vec<MapRange>,
    humidity_location: Vec<MapRange>,
}

impl Maps {
    fn to_array(&self) -> [&Vec<MapRange>; 7] {
        [
            &self.seed_soil,
            &self.soil_fertilizer,
            &self.fertilizer_water,
            &self.water_light,
            &self.light_temperature,
            &self.temperature_humidity,
            &self.humidity_location,
        ]
    }
}

enum CurrentMap {
    SeedSoil,
    SoilFertilizer,
    FertilizerWater,
    WaterLight,
    LightTemperature,
    TemperatureHumidity,
    HumidityLocation,
}

fn parse_input(input: &str) -> (Vec<SeedRange>, Maps) {
    let mut seed_soil: Vec<MapRange> = Vec::new();
    let mut soil_fertilizer: Vec<MapRange> = Vec::new();
    let mut fertilizer_water: Vec<MapRange> = Vec::new();
    let mut water_light: Vec<MapRange> = Vec::new();
    let mut light_temperature: Vec<MapRange> = Vec::new();
    let mut temperature_humidity: Vec<MapRange> = Vec::new();
    let mut humidity_location: Vec<MapRange> = Vec::new();

    let mut seeds: Vec<SeedRange> = Vec::new();
    let mut current_map = None;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if line.starts_with("seeds: ") {
            let nums: Vec<usize> = line
                .split(" ")
                .skip(1)
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            for i in (0..nums.len()).step_by(2) {
                seeds.push(SeedRange::new(nums[i], nums[i + 1]));
            }
            continue;
        } else if line.starts_with("seed-to-soil map:") {
            current_map = Some(CurrentMap::SeedSoil);
            continue;
        } else if line.starts_with("soil-to-fertilizer map:") {
            current_map = Some(CurrentMap::SoilFertilizer);
            continue;
        } else if line.starts_with("fertilizer-to-water map:") {
            current_map = Some(CurrentMap::FertilizerWater);
            continue;
        } else if line.starts_with("water-to-light map:") {
            current_map = Some(CurrentMap::WaterLight);
            continue;
        } else if line.starts_with("light-to-temperature map:") {
            current_map = Some(CurrentMap::LightTemperature);
            continue;
        } else if line.starts_with("temperature-to-humidity map:") {
            current_map = Some(CurrentMap::TemperatureHumidity);
            continue;
        } else if line.starts_with("humidity-to-location map:") {
            current_map = Some(CurrentMap::HumidityLocation);
            continue;
        }
        let mut parts = line.split(" ");

        let destination = parts.next().unwrap().parse::<usize>().unwrap();
        let source = parts.next().unwrap().parse::<usize>().unwrap();
        let length = parts.next().unwrap().parse::<usize>().unwrap();
        let new_range = MapRange::new(source, destination, length);

        match current_map {
            Some(CurrentMap::SeedSoil) => seed_soil.push(new_range),
            Some(CurrentMap::SoilFertilizer) => soil_fertilizer.push(new_range),
            Some(CurrentMap::FertilizerWater) => fertilizer_water.push(new_range),
            Some(CurrentMap::WaterLight) => water_light.push(new_range),
            Some(CurrentMap::LightTemperature) => light_temperature.push(new_range),
            Some(CurrentMap::TemperatureHumidity) => temperature_humidity.push(new_range),
            Some(CurrentMap::HumidityLocation) => humidity_location.push(new_range),

            None => panic!("No current map"),
        }
    }

    (
        seeds,
        Maps {
            seed_soil,
            soil_fertilizer,
            fertilizer_water,
            water_light,
            light_temperature,
            temperature_humidity,
            humidity_location,
        },
    )
}

pub fn part_one(input: &str) -> Option<u64> {
    let (seeds, maps) = parse_input(input);
    let seeds: Vec<usize> = seeds
        .iter()
        .flat_map(|seed| [seed.start, seed.length])
        .collect();
    Some(
        seeds
            .iter()
            .map(|seed| {
                let soil = get_from_map(&maps.seed_soil, *seed).unwrap_or(*seed);
                let fertilizer = get_from_map(&maps.soil_fertilizer, soil).unwrap_or(soil);
                let water = get_from_map(&maps.fertilizer_water, fertilizer).unwrap_or(fertilizer);
                let light = get_from_map(&maps.water_light, water).unwrap_or(water);
                let temperature = get_from_map(&maps.light_temperature, light).unwrap_or(light);
                let humidity =
                    get_from_map(&maps.temperature_humidity, temperature).unwrap_or(temperature);
                let location = get_from_map(&maps.humidity_location, humidity).unwrap_or(humidity);
                location as u64
            })
            .min()
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (seeds, maps) = parse_input(input);
    /* Some(
        seeds
            .par_iter()
            .map(|seed_range| {
                let mut min_location = usize::MAX;
                for seed in seed_range.start..seed_range.end {
                    let soil = get_from_map(&maps.seed_soil, seed).unwrap_or(seed);
                    let fertilizer = get_from_map(&maps.soil_fertilizer, soil).unwrap_or(soil);
                    let water =
                        get_from_map(&maps.fertilizer_water, fertilizer).unwrap_or(fertilizer);
                    let light = get_from_map(&maps.water_light, water).unwrap_or(water);
                    let temperature = get_from_map(&maps.light_temperature, light).unwrap_or(light);
                    let humidity = get_from_map(&maps.temperature_humidity, temperature)
                        .unwrap_or(temperature);
                    let location =
                        get_from_map(&maps.humidity_location, humidity).unwrap_or(humidity);
                    if location < min_location {
                        min_location = location;
                    }
                }
                min_location as u64
            })
            .min()
            .unwrap(),
    ) */
    let seed_locs: Vec<SeedRange> = seeds
        .iter()
        .flat_map(|seed| {
            let mut ranges = get_ranges_from_map(&maps.seed_soil, seed);
            ranges = ranges
                .iter()
                .flat_map(|range| get_ranges_from_map(&maps.soil_fertilizer, range))
                .collect();
            ranges = reduce_ranges(ranges);
            ranges = ranges
                .iter()
                .flat_map(|range| get_ranges_from_map(&maps.fertilizer_water, range))
                .collect();
            ranges = reduce_ranges(ranges);
            ranges = ranges
                .iter()
                .flat_map(|range| get_ranges_from_map(&maps.water_light, range))
                .collect();
            ranges = reduce_ranges(ranges);
            ranges = ranges
                .iter()
                .flat_map(|range| get_ranges_from_map(&maps.light_temperature, range))
                .collect();
            ranges = reduce_ranges(ranges);
            ranges = ranges
                .iter()
                .flat_map(|range| get_ranges_from_map(&maps.temperature_humidity, range))
                .collect();
            ranges = reduce_ranges(ranges);
            ranges = ranges
                .iter()
                .flat_map(|range| get_ranges_from_map(&maps.humidity_location, range))
                .collect();
            ranges = reduce_ranges(ranges);
            ranges
        })
        .collect();

    Some(
        seed_locs
            .iter()
            .map(|range| range.start as u64)
            .min()
            .unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ranges_from_map() {
        let map = vec![MapRange::new(10, 0, 100)];

        // Test single range starting inside
        let range = SeedRange::new(50, 100);
        let result = get_ranges_from_map(&map, &range);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], SeedRange::new(40, 60));
        assert_eq!(result[1], SeedRange::new(110, 40));

        // Test single range completely inside
        let range = SeedRange::new(20, 50);
        let result = get_ranges_from_map(&map, &range);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], SeedRange::new(10, 50));

        // Test single range completely outside
        let range = SeedRange::new(200, 50);
        let result = get_ranges_from_map(&map, &range);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], SeedRange::new(200, 50));
    }

    #[test]
    fn test_fill_gaps() {
        // Test single range
        let mut map = vec![MapRange::new(10, 0, 100)];
        fill_gaps(&mut map);
        assert_eq!(map.len(), 3);
        assert_eq!(map[0], MapRange::new(0, 0, 10));
        assert_eq!(map[1], MapRange::new(10, 0, 100));
        assert_eq!(map[2], MapRange::new(110, 110, usize::MAX - 110));

        // Test two ranges
        let mut map = vec![MapRange::new(10, 0, 100), MapRange::new(200, 0, 100)];
        fill_gaps(&mut map);
        assert_eq!(map.len(), 5);
        assert_eq!(map[0], MapRange::new(0, 0, 10));
        assert_eq!(map[1], MapRange::new(10, 0, 100));
        assert_eq!(map[2], MapRange::new(110, 110, 90));
        assert_eq!(map[3], MapRange::new(200, 0, 100));
        assert_eq!(map[4], MapRange::new(300, 300, usize::MAX - 300));

        // Test map starting at 0
        let mut map = vec![MapRange::new(0, 0, 100), MapRange::new(200, 0, 100)];
        fill_gaps(&mut map);
        assert_eq!(map.len(), 4);
        assert_eq!(map[0], MapRange::new(0, 0, 100));
        assert_eq!(map[1], MapRange::new(100, 100, 100));
        assert_eq!(map[2], MapRange::new(200, 0, 100));
        assert_eq!(map[3], MapRange::new(300, 300, usize::MAX - 300));

        // Test map ending at MAX
        let mut map = vec![
            MapRange::new(10, 0, 100),
            MapRange::new(usize::MAX - 100, 0, 100),
        ];
        fill_gaps(&mut map);
        assert_eq!(map.len(), 4);
        assert_eq!(map[0], MapRange::new(0, 0, 10));
        assert_eq!(map[1], MapRange::new(10, 0, 100));
        assert_eq!(map[2], MapRange::new(110, 110, usize::MAX - 110 - 100));
        assert_eq!(map[3], MapRange::new(usize::MAX - 100, 0, 100));
    }

    #[test]
    fn test_input_parser() {
        let (seeds, mut maps) = parse_input(&advent_of_code::template::read_file("examples", DAY));
        fill_gaps(maps.seed_soil.as_mut());
        fill_gaps(maps.temperature_humidity.as_mut());
        assert_eq!(seeds[0], SeedRange::new(79, 14));
        assert_eq!(get_from_map(&maps.seed_soil, 98), Some(50));
        assert_eq!(get_from_map(&maps.seed_soil, 99), Some(51));
        assert_eq!(get_from_map(&maps.seed_soil, 53), Some(55));
        assert_eq!(get_from_map(&maps.temperature_humidity, 69), Some(0));
        assert_eq!(get_from_map(&maps.temperature_humidity, 2), Some(3));
        assert_eq!(get_from_map(&maps.temperature_humidity, 70), Some(70));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two_a() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(0));
    }
}

/*
location:
0

humidity:
262282387

temperature:
502588504

light:
502588504

water:


*/
