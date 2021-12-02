use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;



fn main() -> io::Result<()> {

let mut instructions = Vec::new();
   let f = BufReader::new(File::open("input.txt")?);
   for line in f.lines() {
       let current_Line=line.unwrap();
       instructions.push(current_Line);
    
     }
 
    pt1(&instructions);
    pt2(&instructions);

 
     Ok(())


 }

fn pt1(instructions:&Vec<String>){
    let mut x=0;
    let mut y=0;
    for instr in instructions{
        let new_dir:Vec<&str>=instr.split(" ").collect();
        let val:i32=new_dir[1].parse::<i32>().unwrap();
        match new_dir[0]{
            "forward"=>(x+=val),
            "up"=>(y-=val),
            "down"=>(y+=val),
            _ => panic!(),
        }
    }
    println!("{}", x*y)
}


fn pt2(instructions:&Vec<String>){
    let mut x=0;
    let mut y=0;
    let mut aim=0;
    for instr in instructions{
        let new_dir:Vec<&str>=instr.split(" ").collect();
        let val:i32=new_dir[1].parse::<i32>().unwrap();

        match new_dir[0]{
            "forward"=>||->(){x+=val; y+=val*aim}(),
            "up"=>||->(){aim-=val}(),
            "down"=>||->(){aim+=val}(),
            _ => panic!(),
        }
    }
    println!("{}", x*y)
}