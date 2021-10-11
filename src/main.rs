extern crate enum_index;
#[macro_use]
extern crate enum_index_derive;

mod helper;
mod poker;
mod test;

fn main() {
    let test_data = [
        [
            "3s",
            "3h",
            "3d",
            "5h",
            "5d",
            "5s",
            "8h",
            "8d",
            "3h",
            "2d",
            "4h",
            "5d",
            "Ah"
        ],
        [
            "2s",
            "3s",
            "4s",
            "5s",
            "6s",
            "7s",
            "8s",
            "9s",
            "Ts",
            "Js",
            "Qs",
            "Ks",
            "As",
        ],
        [
            "2d",
            "3d",
            "4d",
            "5d",
            "6d",
            "7d",
            "8d",
            "9d",
            "Td",
            "Jd",
            "Qd",
            "Kd",
            "Ad"
        ],
    ];
    let result = poker::game::parse_input(test_data);
    println!("{}", result.unwrap());
}
