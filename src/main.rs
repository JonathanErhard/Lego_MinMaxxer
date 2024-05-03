//use std::fmt;
extern crate json;
extern crate set_partitions;
use std::fs;

use json::JsonValue;

struct LegoSet{
    name:String,
    set_nr:u32,
    points:u32,
    price:f32
}

/*impl fmt::Display for LegoSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {:?}, Preis: {:?}, Anzahl: {:?}", self.name, self.preis, self.anzahl);
    }
}*/

fn print_set(set: &LegoSet){
    println!("name: {:?}, set_nr: {:?}, points: {:?}, price: {:?}", set.name, set.set_nr, set.points, set.price)
}

fn parse_json(path: &str) -> (Vec<LegoSet>, Vec<LegoSet>, f32){
    let data: String = fs::read_to_string(path)
        .expect("Should have been able to read the file");
    let data: Result<JsonValue, json::Error> = json::parse(data.as_str());
    let data: JsonValue = data.unwrap();

    //read includes
    let mut must_include_nr: Vec<u32> = vec![];
    let json_array = &data["must-include"];
    for i in 0..json_array.len()-1{
        let set_nr = json_array[i].as_u32().unwrap();
        must_include_nr.push(set_nr)
    }

    //read possible sets
    let mut possible_sets: Vec<LegoSet> = vec![];
    let mut must_include: Vec<LegoSet> = vec![];

    let json_array = &data["list-of-sets"];
    for i in 0..json_array.len()-1{
        let set = &json_array[i];
        
        let set: LegoSet = LegoSet{
            name:   String::from(set["name"].as_str().unwrap()),
            set_nr: set["set-nr"].as_u32().unwrap(),
            price:  set["price"].as_f32().unwrap(),
            points: set["points"].as_u32().unwrap()
        };
        let mut must_bool = false;
        let set_nr: u32 = set.set_nr;
        for nr in &must_include_nr{
            if *nr == set_nr{
                must_bool = true;
                break;
            }
        }
        if must_bool {
            must_include.push(set);
        }
        else{
            possible_sets.push(set);
        }
    }

    let budget = data["budget"].as_f32().unwrap();

    (possible_sets,must_include,budget)
}

//I didnt write that but i cant be bothered
/*fn splice(channels: usize, data: Vec<&LegoSet>) -> Vec<Vec<&LegoSet>> {
    let each_len = data.len() / channels + if data.len() % channels == 0 { 0 } else { 1 };
    let mut out = vec![Vec::with_capacity(each_len); channels];
    for (i, d) in data.iter().copied().enumerate() {
        out[i % channels].push(d);
    }
    out
}*/

fn print_result(result:Vec<Vec<Vec<&LegoSet>>>){
    println!("There are {} different optimal combinations!",result.len());
    //naming needs some work
    for set_set_set in result{
        print!("\n");
        for set_set in set_set_set{
            print!("\n");
            for set in set_set {
                print_set(set);
            }
        }
    }
}
fn main() {

    let possible_sets:Vec<LegoSet>;
    let must_include:Vec<LegoSet>;
    let budget:f32;
    (possible_sets, must_include, budget) = parse_json("dataset-original.json");

    let set_of_best_sets_of_sets:Vec<Vec<LegoSet>> = vec![];

    let length = possible_sets.len() as usize;
    let end = 1 << length;
    for i in 0..end{
        //TODO check if sum > budget
        for pos in 0..length{
            //creating valid subsets
            let mut subset:Vec<&LegoSet> = vec![];
            if i & (1 << pos) == 0{
                subset.push(&possible_sets[pos]);
            }
            for set in &must_include{
                subset.push(set);
            }
            
        }
    }
}

/* tested something
fn main(){
    let mut result = vec![];
    let mut option1 = vec![];
    let set: LegoSet = LegoSet{
        name:   String::from("lego1"),
        set_nr: 123,
        price:  2.0,
        points: 32
    };
    option1.push(vec![&set]);
    let mut option2 = vec![];
    let set2: LegoSet = LegoSet{
        name:   String::from("lego1"),
        set_nr: 321,
        price:  4.0,
        points: 2
    };
    option2.push(vec![&set2]);
    result.push(option1);
    result.push(option2);
    print_result(result);
}
*/