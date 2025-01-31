use std::cmp::Ordering;
use std::env::var;
use std::io;
use std::io::Write;

fn main() {
    println!("something!");
    let mut input= String::new();
    let mut tmp = String::new();

    print!("input (end with a '|'): ");
    io::stdout().flush().expect("failed to flush input buffer!");

    loop{
        io::stdin().read_line(&mut tmp).expect("failed to read line");
        //println!("read a line!!");
        match tmp.as_str() {
            "|" => break,
            "|\n" => break,
             _  => input.push_str(&tmp),
        }
        tmp.clear();
    }

    input.pop(); //remove last newline

    //println!("{input}");

    let result = aoc2_2(&input);

    println!("{result}");
    
}


fn aoc1_1(s: &str) -> u32{ //day one part 1
    let mut vec1:Vec<u32>  = vec![];
    let mut vec2:Vec<u32>  = vec![];

    for line in s.split("\n") {
        //println!("{line}");
        let mut items  = line.split_whitespace();

        let item1 = items.next().expect("failed to get next item!").parse::<u32>().expect("failed to parse!");
        let item2 = items.next().expect("failed to get next item!").parse::<u32>().expect("failed to parse!");
        vec1.push(item1);
        vec2.push(item2);
    }

    vec1.sort();
    vec2.sort();

    let mut total = 0;

    if vec1.len() == vec2.len() {
        for i in 0..vec1.len(){
            total += vec1[i].abs_diff(vec2[i]);
        }
    }
    else{
        
    }

    return total;
}


fn aoc1_2(s: &str) -> u32{ //day one part 2
    let mut vec1:Vec<u32>  = vec![];
    let mut vec2:Vec<u32>  = vec![];

    for line in s.split("\n") {
        //println!("{line}");
        let mut items  = line.split_whitespace();

        let item1 = items.next().expect("failed to get next item!").parse::<u32>().expect("failed to parse!");
        let item2 = items.next().expect("failed to get next item!").parse::<u32>().expect("failed to parse!");
        vec1.push(item1);
        vec2.push(item2);
    }

    vec1.sort();
    vec2.sort();

    let mut total = 0;

    if vec1.len() == vec2.len() {
        for i in &vec1{
            let mut repeats: u32 = 0;
            for j in &vec2{
                if i == j{
                    repeats += 1;
                }
            }
            total += i * repeats;
        }
    }
    else{
        
    }

    return total;
}


fn aoc2_1(s: &str) -> u32{ //day two part 1
    let mut total:u32 = 0;
    for line in s.split("\n"){
        let mut safe:bool = true;

        let mut items = Vec::new();

        for item in line.split_whitespace(){ //converting the line to a u32 Vec
            let item = item.parse::<u32>().expect("convertion failed!");
            items.push(item);
        }


        let mut prev:&u32       = &0;
        let mut loop_count = 0;
        let mut increasing:bool = true;
        for item in &items{ //now process it
            if loop_count == 0{
                prev = item;
                loop_count+=1;
                continue; //the first item is always valid
            }
            else{
                if loop_count == 1{ //decide if it's increasing or not
                    match item.cmp(prev){
                        Ordering::Less => increasing = false,
                        Ordering::Greater => (),
                        Ordering::Equal => {
                            safe = false;
                        },
                    }
                }

                if (item.cmp(prev) == Ordering::Less && increasing) || (item.cmp(prev) == Ordering::Greater && !increasing){
                    safe = false;
                }

                let diff = item.abs_diff(*prev);
                if diff < 1 || diff > 3{
                    safe = false;
                }
            }
            prev = item;
            loop_count+=1;
        }

        if safe{
            total +=1 ;
        }
    }

    return total;
}


fn aoc2_2(s: &str) -> u32{ //day two part 2
    let mut total:u32 = 0;
    for line in s.split("\n"){
        

        let mut items = Vec::new();

        for item in line.split_whitespace(){ //converting the line to a u32 Vec
            let item = item.parse::<u32>().expect("convertion failed!");
            items.push(item);
        }


        let mut variations: Vec<Vec<u32>> = Vec::new();
        variations.push(items.clone()); //include the entire list too

        for (i,_thing) in items.iter().enumerate(){
            let mut tmp = items.clone();
            tmp.remove(i);
            variations.push(tmp);
        }
        //println!("{variations:?}");

        
        
        'outer: for v in variations{
            let mut safe:bool = true;
            let mut prev:&u32       = &0;
            let mut loop_count = 0;
            let mut increasing:bool = true;
            'inner: for item in &v{ //now process it
                if loop_count == 0{
                    prev = item;
                    loop_count+=1;
                    continue; //the first item is always valid
                }
                else{
                    if loop_count == 1{ //decide if it's increasing or not
                        match item.cmp(prev){
                            Ordering::Less => increasing = false,
                            Ordering::Greater => (),
                            Ordering::Equal => {
                                safe = false;
                            },
                        }
                    }

                    if (item.cmp(prev) == Ordering::Less && increasing) || (item.cmp(prev) == Ordering::Greater && !increasing){
                        safe = false;
                    }

                    let diff = item.abs_diff(*prev);
                    if diff < 1 || diff > 3{
                        safe = false;
                    }
                }
                prev = item;
                loop_count+=1;
            }

            if safe{
                total +=1;
                break 'outer;
            }
        }
    }

    return total;
}