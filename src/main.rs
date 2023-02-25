use std::env::{args, Args};


fn main() {
    let mut args: Args = args();

    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    println!("{:?} {} {}",first,operator,second);
    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();
    let resultv2: f32 =  operate_v2(operator, first_number, second_number);

    println!("{:?}", output(first_number, operator, second_number, resultv2));

}

fn operate_v2(operator: char, first_number: f32, second_number: f32) -> f32{
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '/' => first_number / second_number,
        '*'  | 'x' | 'X'=> first_number * second_number,
        _ => panic!("Invalid operator used.")
    }
}




fn output(first_number: f32, operator: char, second_number: f32,result: f32) -> String {
    format!("{} {} {} = {}", first_number,operator,second_number,result)
}

// fn nth(&mut self, n: usize) -> Option<String> {
//     // assume n = 0;
//     // inner = ["1","2"]
//     self.inner.next() // "1"
//     // Calling next again results in second element
//     self.inner.next() // "2"
// }