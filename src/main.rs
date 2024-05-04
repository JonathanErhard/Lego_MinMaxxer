//use std::fmt;
extern crate json;
use std::fs;

use json::JsonValue;

#[derive(Debug)]
struct LegoSet {
    name: String,
    set_nr: u32,
    points: u32,
    price: f32,
}

impl std::fmt::Display for LegoSet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "name: {:?}, set_nr: {:?}, points: {:?}, price: {:?}",
            self.name, self.set_nr, self.points, self.price
        )
    }
}

/*impl fmt::Display for LegoSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {:?}, Preis: {:?}, Anzahl: {:?}", self.name, self.preis, self.anzahl);
    }
}*/

fn parse_json(path: &str) -> (Vec<LegoSet>, Vec<LegoSet>, f32) {
    let data: String = fs::read_to_string(path).expect("Should have been able to read the file");
    let data: Result<JsonValue, json::Error> = json::parse(data.as_str());
    let data: JsonValue = data.unwrap();

    //read includes
    let mut must_include_nr: Vec<u32> = vec![];
    let json_array = &data["must-include"];
    for i in 0..json_array.len() {
        let set_nr = json_array[i].as_u32().unwrap();
        must_include_nr.push(set_nr)
    }
    //read possible sets
    let mut possible_sets: Vec<LegoSet> = vec![];
    let mut must_include: Vec<LegoSet> = vec![];

    let json_array = &data["list-of-sets"];
    for i in 0..json_array.len() {
        let set = &json_array[i];

        let set: LegoSet = LegoSet {
            name: String::from(set["name"].as_str().unwrap()),
            set_nr: set["set-nr"].as_u32().unwrap(),
            price: set["price"].as_f32().unwrap(),
            points: set["points"].as_u32().unwrap(),
        };
        let mut must_bool = false;
        let set_nr: u32 = set.set_nr;
        for nr in &must_include_nr {
            if *nr == set_nr {
                must_bool = true;
                break;
            }
        }
        if must_bool {
            must_include.push(set);
        } else {
            possible_sets.push(set);
        }
    }

    let budget = data["budget"].as_f32().unwrap();

    //println!("ps:\n{:#?}\n mi:\n{:#?}", possible_sets, must_include);
    (possible_sets, must_include, budget)
}

fn print_result(result: Vec<Vec<Vec<&LegoSet>>>, price: f32, count: usize, points: u32) {
    println!("There are {} different optimal combinations!", result.len());
    println!("price:  {price}, Setcount: {count}, points: {points}");
    //naming needs some work
    for set_set_set in result {
        println!();
        for set_set in set_set_set {
            println!();
            for set in set_set {
                println!("{set}");
            }
        }
    }
}

fn create_partitions<T: Copy>(
    set: &Vec<T>,
    index: usize,
    union: &mut Vec<Vec<T>>,
    result: &mut Vec<Vec<Vec<T>>>,
) {
    if set.len() == index {
        result.push(union.to_vec());
        return;
    }

    for i in 0..union.len() {
        union[i].push(set[index]);
        create_partitions(set, index + 1, union, result);
        //println!("{:?}",union.len());
        union[i].pop();
    }

    union.push(vec![set[index]]);
    create_partitions(set, index + 1, union, result);
    union.pop();
}
fn main() {
    let possible_sets: Vec<LegoSet>;
    let must_include: Vec<LegoSet>;
    let budget: f32;
    let min_cart: f32 = 160.0;

    (possible_sets, must_include, budget) = parse_json("dataset-original.json");

    let mut set_of_best_sets_of_sets: Vec<Vec<Vec<&LegoSet>>> = vec![];

    let length = possible_sets.len();
    let end = 1 << length;
    let mut best_count: usize = 0;
    let mut best_price: f32 = budget;
    let mut best_points: u32 = 0;
    for i in 0..end {
        let mut subset: Vec<&LegoSet> = vec![];
        let mut price: f32 = 0.0;
        let mut points: u32 = 0;
        for pos in 0..length {
            //creating valid subsets
            if i & (1 << pos) == 0 {
                subset.push(&possible_sets[pos]);
                price += possible_sets[pos].price;
                points += possible_sets[pos].points;
            }
        }
        for set in &must_include {
            subset.push(set);
            price += set.price;
            points += set.points;
        }
        //check budget range
        if price > budget {
            continue;
        }
        //splitting subsets into sets of subsets
        let mut result = vec![];
        create_partitions(&subset, 0, &mut vec![], &mut result);
        //check all sets of sets for possible solutions
        for set_of_sets in result {
            //dont bother with sets, that couldn't fill enough carts in the first place
            if set_of_sets.len() < best_count {
                continue;
            } else {
                let mut count: usize = 0;
                //check every cart's price (could break outer, if one of them doesn't reach the threshhold since it cannot be part of the solutions)
                for set in &set_of_sets {
                    let mut cart_price: f32 = 0.0;
                    for lego in set {
                        cart_price += lego.price;
                    }
                    if cart_price >= min_cart {
                        count += 1;
                    }
                }
                //compare to current best ... goto would have been goated ):
                if false {
                    //count > 4 {
                    println!(
                        "size:{} count: {count}, price: {price}, points: {points}",
                        set_of_sets.len()
                    );
                }
                if count < best_count {
                    continue;
                } else if count > best_count {
                    //more free sets
                    set_of_best_sets_of_sets = vec![set_of_sets];
                    best_count = count;
                    best_price = price;
                    best_points = points;
                } else if price > best_price {
                    //worse price
                    continue;
                } else if price < best_price {
                    //better price
                    set_of_best_sets_of_sets = vec![set_of_sets];
                    best_price = price;
                    best_points = points;
                } else if points < best_points {
                    //worse points
                    continue;
                } else if points > best_points {
                    //better points
                    set_of_best_sets_of_sets = vec![set_of_sets];
                    best_points = points;
                } else {
                    //everything is the same
                    set_of_best_sets_of_sets.push(set_of_sets);
                }
            }
        }
    }
    print_result(
        set_of_best_sets_of_sets,
        best_price,
        best_count,
        best_points,
    );
}

/*fn main1() {
    let set = vec![1, 2, 3, 4];
    let mut p = vec![];
    let mut result = vec![];
    create_partitions(&set, 0, &mut p, &mut result);
    println!("{result:?}")
}*/
