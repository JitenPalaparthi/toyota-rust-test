use std::collections::HashMap;
use std::fs::File;

use std::io;
use std::io::{BufRead, BufReader};

fn main() {
    let result1 = compare_datas("test_data/data1".to_string());
    let result2 = compare_datas("test_data/data2".to_string());
    let result3 = compare_datas("test_data/data3".to_string());

    println!("Result of test_data, dataset-1: {}", result1);

    println!("Result of test_data, dataset-2: {}", result2);

    println!("Result of test_data, dataset-3: {}", result3);

}

fn compare_datas(filename: String) -> bool {
    let file = File::open(filename).unwrap();

    let reader = io::BufReader::new(file);

    let mut map1: HashMap<String, i32> = std::collections::HashMap::new();
    let mut map2: HashMap<String, i32> = std::collections::HashMap::new();

    let mut sepMap: bool = false;

    for line_result in reader.lines() {
        let line = line_result.unwrap();

        //println!("{:?}",line);
        if line == "" {
            sepMap = true;
            continue;
        }

        if line == "year,state,month,number" {
            continue;
        }
        if line == "year,number" {
            continue;
        }

        let splitlines: Vec<&str> = line.split(",").collect();
        if splitlines.len() == 4 && sepMap == false {
            if map1.contains_key(splitlines[0]) {
                let mut v1 = map1.get(splitlines[0]);
                let mut v2 = splitlines[3].parse::<i32>().unwrap();
                map1.insert(splitlines[0].to_string(), v1.unwrap_or(&0) + v2);
            } else {
                map1.insert(
                    splitlines[0].to_string(),
                    splitlines[3].parse::<i32>().unwrap(),
                );
            }
        } else if splitlines.len() == 2 && sepMap {
            if map2.contains_key(splitlines[0]) {
                let v1 = map2.get(splitlines[0]).unwrap_or(&0);
                let v2: i32 = splitlines[1].parse::<i32>().unwrap();
                map2.insert(splitlines[0].to_string(), v1 + v2);
            } else {
                map2.insert(
                    splitlines[0].to_string(),
                    splitlines[1].parse::<i32>().unwrap(),
                );
            }
        }
    }
    let mut yes: bool = false;
    for (k1, v1) in map2 {
        let v2 = map1.get(&k1).unwrap_or(&0);

        if v1 == *v2 {
            yes = true;
        } else {
           // yes = false;
            return false;
        }
    }
    return yes;
}
