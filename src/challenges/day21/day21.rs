#[path = "../../utils/mod.rs"] mod utils;

use std::collections::HashMap;

pub struct Monkey{
    value: i64,
    has_value: bool,
    monkey1_name: String,
    monkey2_name: String,
    compute_monkey_value: Box<dyn Fn(i64, i64) -> i64>
}

pub fn evaluate_monkey(monkey_id: usize, monkey_tree: &mut Vec<Monkey>,name_to_id: &HashMap<String, usize>) -> i64{
    if monkey_tree[monkey_id].has_value {
        return monkey_tree[monkey_id].value;
    }
    let monkey1_name = String::from(&monkey_tree[monkey_id].monkey1_name);
    let monkey2_name = String::from(&monkey_tree[monkey_id].monkey2_name);
    let monkey1_id: usize = *(name_to_id.get(&monkey1_name).unwrap());
    let monkey2_id: usize = *(name_to_id.get(&monkey2_name).unwrap());
    let monkey1_value = evaluate_monkey(monkey1_id,monkey_tree,name_to_id);
    let monkey2_value = evaluate_monkey(monkey2_id,monkey_tree,name_to_id);
    let value = (monkey_tree[monkey_id].compute_monkey_value)(monkey1_value,monkey2_value);
    monkey_tree[monkey_id].value = value;
    monkey_tree[monkey_id].has_value = true;
    return value;
}

pub fn part1() {
    let mut last_id_available: usize = 0;
    let mut name_to_id: HashMap<String, usize> = HashMap::new();
    let mut monkey_tree: Vec<Monkey> = vec![];
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                let split = line_content.split(" ").collect::<Vec<&str>>();
                let monkey_name = split[0].split(":").collect::<Vec<&str>>()[0];
                name_to_id.insert(String::from(monkey_name),last_id_available);
                if split.len() == 2 {
                    let value: i64 = split[1].parse().unwrap();
                    monkey_tree.push(Monkey{value, has_value:true,monkey1_name:String::from(monkey_name),monkey2_name:String::from(monkey_name), compute_monkey_value: Box::new(move |_,_|-> i64{value})});
                }else{
                    let monkey1_name: String = String::from(split[1]);
                    let monkey2_name: String = String::from(split[3]);
                    let mut monkey_operation:Box<dyn Fn(i64, i64) -> i64> = Box::new(|_:i64,_:i64| -> i64{0});
                    if split[2] == "+" {
                        monkey_operation = Box::new(|a:i64, b:i64| -> i64{a + b});
                    }else if split[2] == "-"{
                        monkey_operation = Box::new(|a:i64, b:i64| -> i64{a - b});
                    }else if split[2] == "*"{
                        monkey_operation = Box::new(|a:i64, b:i64| -> i64{a * b});
                    }else if split[2] == "/"{
                        monkey_operation = Box::new(|a:i64, b:i64| -> i64{a / b});
                    }else{
                        println!("operation incompatible '{}'",split[2]);
                    }
                    let compute_monkey_value:Box<dyn Fn(i64, i64) -> i64> = Box::new(move |a:i64,b:i64|-> i64{
                        monkey_operation(a,b)
                    });
                    monkey_tree.push(Monkey{value:0, has_value:false,monkey1_name,monkey2_name, compute_monkey_value});
                }
                last_id_available += 1;
            }
        }
    }else{
        println!("lines not ok");
    }
    let root_id = *(name_to_id.get("root").unwrap());
    let root_value = evaluate_monkey(root_id, &mut monkey_tree, &name_to_id);
    println!("{}",root_value);
}

pub struct AdvancedMonkey{
    value: i64,
    has_value: bool,
    monkey1_name: String,
    monkey2_name: String,
    operation_char: char
}

pub struct ReverseOperation{
    op: char,
    value: i64,
    pos: usize
}

pub enum TryToGetValue{
    Value(i64),
    OperationChain(Vec<ReverseOperation>)
}

pub fn reverse_evaluate_monkey(monkey_id: usize, monkey_tree: &mut Vec<AdvancedMonkey>,name_to_id: &HashMap<String, usize>) -> TryToGetValue{
    if monkey_tree[monkey_id].has_value {
        return TryToGetValue::Value(monkey_tree[monkey_id].value);
    }
    let monkey1_name = String::from(&monkey_tree[monkey_id].monkey1_name);
    let mut monkey1_value: i64 = 0;
    let monkey1_enum_value: TryToGetValue;
    let mut found_monkey1_val = false;
    let monkey2_name = String::from(&monkey_tree[monkey_id].monkey2_name);
    let mut monkey2_value: i64 = 0;
    let monkey2_enum_value: TryToGetValue;
    let mut found_monkey2_val = false;
    
    if monkey1_name != "humn"{
        let monkey1_id: usize = *(name_to_id.get(&monkey1_name).unwrap());
        monkey1_enum_value = reverse_evaluate_monkey(monkey1_id,monkey_tree,name_to_id);
        match monkey1_enum_value {
            TryToGetValue::Value(monkey1_enum_value) => {
                monkey1_value = monkey1_enum_value;
                found_monkey1_val = true;
            },
            TryToGetValue::OperationChain(_) => {}
        }
    }else{
        monkey1_enum_value = TryToGetValue::OperationChain(vec![ReverseOperation{op:'=',value:0,pos:0}]);
    }

    if monkey2_name != "humn"{
        let monkey2_id: usize = *(name_to_id.get(&monkey2_name).unwrap());
        monkey2_enum_value = reverse_evaluate_monkey(monkey2_id,monkey_tree,name_to_id);
        match monkey2_enum_value {
            TryToGetValue::Value(monkey2_enum_value) => {
                monkey2_value = monkey2_enum_value;
                found_monkey2_val = true;
            },
            TryToGetValue::OperationChain(_) => {}
        }
    }else{
        monkey2_enum_value = TryToGetValue::OperationChain(vec![ReverseOperation{op:'=',value:0,pos:0}]);
    }

    if found_monkey2_val && found_monkey1_val {
        let mut value = 0;
        if monkey_tree[monkey_id].operation_char == '+'{
            value = monkey1_value + monkey2_value;
        }else if monkey_tree[monkey_id].operation_char == '-'{
            value = monkey1_value - monkey2_value;
        }else if monkey_tree[monkey_id].operation_char == '*'{
            value = monkey1_value * monkey2_value;
        }else if monkey_tree[monkey_id].operation_char == '/'{
            value = monkey1_value / monkey2_value;
        }
        monkey_tree[monkey_id].value = value;
        monkey_tree[monkey_id].has_value = true;
        return TryToGetValue::Value(value);
    }
    let mut monkey_reverse_operations: TryToGetValue = TryToGetValue::OperationChain(vec![]);
    if !found_monkey1_val{
        match monkey1_enum_value{
            TryToGetValue::OperationChain(mut monkey1_operation_chain) =>{
                let operation_char = monkey_tree[monkey_id].operation_char;
                let value = monkey2_value;
                let pos = 1;
                let new_reverse_operation = ReverseOperation{op:operation_char,value,pos};
                monkey1_operation_chain.push(new_reverse_operation);
                monkey_reverse_operations = TryToGetValue::OperationChain(monkey1_operation_chain);
            },
            TryToGetValue::Value(_) => {}
        }
    }
    if !found_monkey2_val{
        match monkey2_enum_value{
            TryToGetValue::OperationChain(mut monkey2_operation_chain) =>{
                let operation_char = monkey_tree[monkey_id].operation_char;
                let value = monkey1_value;
                let pos = 2;
                let new_reverse_operation = ReverseOperation{op:operation_char,value,pos};
                monkey2_operation_chain.push(new_reverse_operation);
                monkey_reverse_operations = TryToGetValue::OperationChain(monkey2_operation_chain);
            },
            TryToGetValue::Value(_) => {}
        }
    }
    return monkey_reverse_operations;
}

pub fn part2() {
    let mut last_id_available: usize = 0;
    let mut name_to_id: HashMap<String, usize> = HashMap::new();
    let mut monkey_tree: Vec<AdvancedMonkey> = vec![];
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                let split = line_content.split(" ").collect::<Vec<&str>>();
                let monkey_name = split[0].split(":").collect::<Vec<&str>>()[0];
                name_to_id.insert(String::from(monkey_name),last_id_available);
                last_id_available += 1;
                if monkey_name == "humn"{
                    monkey_tree.push(AdvancedMonkey{value:0, has_value:false,monkey1_name:String::from(monkey_name),monkey2_name:String::from(monkey_name), operation_char:'='});
                    continue;
                }
                if split.len() == 2 {
                    let value: i64 = split[1].parse().unwrap();
                    monkey_tree.push(AdvancedMonkey{value, has_value:true,monkey1_name:String::from(monkey_name),monkey2_name:String::from(monkey_name), operation_char:'='});
                }else{
                    let monkey1_name: String = String::from(split[1]);
                    let monkey2_name: String = String::from(split[3]);
                    let mut operation_char: char = '-';
                    if monkey_name == "root"{
                        monkey_tree.push(AdvancedMonkey{value:0, has_value:false,monkey1_name,monkey2_name, operation_char});
                        continue;
                    }
                    if split[2] == "+" {
                        operation_char = '+';
                    }else if split[2] == "-"{
                        operation_char = '-';
                    }else if split[2] == "*"{
                        operation_char = '*';
                    }else if split[2] == "/"{
                        operation_char = '/';
                    }else{
                        println!("operation incompatible '{}'",split[2]);
                    }
                    monkey_tree.push(AdvancedMonkey{value:0, has_value:false,monkey1_name,monkey2_name, operation_char});
                }
            }
        }
    }else{
        println!("lines not ok");
    }
    let root_id = *(name_to_id.get("root").unwrap());
    let root_value = reverse_evaluate_monkey(root_id, &mut monkey_tree, &name_to_id);
    match root_value{
        TryToGetValue::OperationChain(mut root_operations) => {
            let mut res = 0;
            while root_operations[root_operations.len()-1].op != '='{
                let new_operation = root_operations.pop().unwrap();
                let op = new_operation.op;
                if op == '+'{
                    res = res - new_operation.value;
                }else if op == '-'{
                    if new_operation.pos == 1{
                        res = res + new_operation.value;
                    }else{
                        res = new_operation.value - res;
                    }
                }else if op == '*'{
                    res = res / new_operation.value;
                }else if op == '/'{
                    if new_operation.pos == 1{
                        res = res * new_operation.value;
                    }else{
                        res = new_operation.value / res;
                    }
                }
            }
            println!("{}",res);
        },
        TryToGetValue::Value(root_value) => {
            println!("value : {}",root_value);
        }
    }
    println!("end");
}
