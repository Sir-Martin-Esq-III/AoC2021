use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;


fn main() -> io::Result<()> {
    let mut depths_vec= Vec::new();
    //Depressing File io stuff :(
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);
    for line in f.lines() {
        let current_Line=line.unwrap();
        let num:i32 =current_Line.parse().unwrap();
        depths_vec.push(num);
    
    }

    pt1(depths_vec.as_slice());
    pt2(depths_vec.as_slice());

    Ok(())
}

fn pt1(number:&[i32]){
    let mut prev_line=number[0];
    let mut count=0;
    for value in number[1..=number.len()-1].iter(){
        if *value>prev_line{
            count+=1;
        }
        prev_line=*value
    }
    println!("{}",count)

}

fn pt2(numbers: &[i32]){
    let mut counter=0;
    let mut prev_num=numbers[0..=2].iter().sum();
    for i in 1..numbers.len()-2{
        let sum:i32=numbers[i..i+3].iter().sum();
        if sum>prev_num{
            counter+=1;
        }
        prev_num=sum;
    }
    println!("{}",counter)
}