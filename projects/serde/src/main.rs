use serde::{Deserialize, Serialize};
use serde_json::{to_string_pretty, from_str};

#[derive(Serialize, Deserialize, Debug)]
//#[serde(rename_all="camelCase")]
struct Dog {
    name: String,
    year_born: i32,
    owner: DogOwner,
}

#[derive(Serialize, Deserialize, Debug)]
//#[serde(rename_all="camelCase")]
//#[serde(deny_unknown_fields)]
struct DogOwner {
    first_name: String,
    last_name: String,
}

fn main() {
    //do_serialize();
    do_deserialize();
}

fn do_serialize() {
    let owner01: DogOwner = DogOwner{first_name: "Darren".to_string(),
                                        last_name: "Barklie".to_string() }; 
    let dog01: Dog = Dog{name: "Khoa".to_string(), year_born: 2016, owner: owner01};
    let dog_ser = to_string_pretty(&dog01);
    if dog_ser.is_ok() {
        println!("{}", dog_ser.ok().unwrap());
    } else {
        println!("{:#?}", dog_ser.err());
    }
}

fn do_deserialize() {
    let json_string = r#"
        {
            "name": "Khoa",
            "year_born": 2016,
            "owner": {
                "first_name": "Darren",
                "last_name": "Barklie"
            }
        }
    "#;
    let dog_deser = from_str::<Dog>(json_string);
    if dog_deser.is_ok() {
        println!("{:#?}", dog_deser.ok().unwrap());
    } else {
        println!("{:#?}", dog_deser.err());
    }
}
