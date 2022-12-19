#[path = "../../utils/mod.rs"] mod utils;

#[derive(Debug,Clone)]
pub struct Subpacket{
    children: Vec<usize>,
    value: Option<i32>,
    parent: usize,
    id: usize
}

pub fn compare_packets(packet1: &Vec<Subpacket>, packet2: &Vec<Subpacket>, subpacket1: &Subpacket, subpacket2: &Subpacket) -> i32 {
    let value1:i32;
    let value2:i32;
    match subpacket1.value {
        None => value1=-1,
        Some(val) => value1=val,
    }
    match subpacket2.value {
        None => value2=-1,
        Some(val) => value2=val,
    }
    if (value1 > -1) && (value2 > -1){
        if value1 < value2 {
            return 1;
        }else if value1 > value2 {
            return -1;
        }else{
            return 0;
        }
    }
    if (value1 == -1) && (value2 == -1){
        let mut res:i32 = 0;
        let mut k:usize = 0;
        while (k < subpacket1.children.len()) && (k < subpacket2.children.len()) {
            let i1 = subpacket1.children[k];
            let i2 = subpacket2.children[k];
            res = compare_packets(packet1, packet2, &packet1[i1], &packet2[i2]);
            if res != 0 {
                return res;
            }
            k += 1;
        }
        if subpacket1.children.len() < subpacket2.children.len() {
            return 1;
        }else if subpacket1.children.len() > subpacket2.children.len() {
            return -1;
        }else{
            return 0;
        }
    }
    if value1 == -1 {
        let subpacket_from_value2 = Subpacket{children:[subpacket2.id].to_vec(),value:None,parent:subpacket2.parent, id:subpacket2.id};
        return compare_packets(packet1, packet2, subpacket1, &subpacket_from_value2);;
    }
    if value2 == -1 {
        let subpacket_from_value1 = Subpacket{children:[subpacket1.id].to_vec(),value:None,parent:subpacket1.parent, id:subpacket1.id};
        return compare_packets(packet1, packet2, &subpacket_from_value1, subpacket2);
    }
    return 0;
}

impl Subpacket {
    pub fn print(&self, packet: &Vec<Subpacket>) -> String {
        if let Some(value) = self.value {
            return value.to_string();
        }else{
            let mut str_packet: String = String::from("[");
            let mut first_elem = true;
            for i in 0..self.children.len() {
                if first_elem{
                    first_elem = false;
                }else{
                    str_packet+=",";
                }
                let str_subpacket: String = packet[self.children[i]].print(packet);
                str_packet += &str_subpacket;
            }
            str_packet+="]";
            return str_packet;
        }
    }
}

pub fn part1() {
    let mut packets: Vec<Vec<Subpacket>> = [].to_vec();
    let mut index_count:usize = 0;
    let mut index_sum:usize = 0;
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                if line_content == ""{
                    packets = [].to_vec();
                }else{
                    let mut packet: Vec<Subpacket> = [].to_vec();
                    let mut last_packet_id: usize = 0;
                    let mut parent_at_level: Vec<usize> = [0].to_vec();
                    let mut newpacket:Subpacket = Subpacket { children: [].to_vec(), value: None, parent:0, id:last_packet_id};
                    packet.push(newpacket);
                    let mut current_value = -1;
                    for c in line_content.chars(){
                        if c == '[' {
                            last_packet_id += 1;
                            let current_parent = parent_at_level[&parent_at_level.len()-1];
                            let mut newpacket:Subpacket = Subpacket { children: [].to_vec(), value: None, parent:current_parent, id:last_packet_id};
                            parent_at_level.push(last_packet_id);
                            packet[current_parent].children.push(last_packet_id);
                            packet.push(newpacket); 
                        }else if c ==']' {
                            if current_value >= 0 {
                                last_packet_id += 1;
                                let current_parent = parent_at_level[&parent_at_level.len()-1];
                                let mut newpacket:Subpacket = Subpacket { children: [].to_vec(), value: Some(current_value), parent:current_parent, id:last_packet_id};
                                packet[current_parent].children.push(last_packet_id);
                                packet.push(newpacket);
                                current_value = -1;
                            }
                            parent_at_level.pop();
                        }else if c == ',' {
                            if current_value >= 0 {
                                last_packet_id += 1;
                                let current_parent = parent_at_level[&parent_at_level.len()-1];
                                let mut newpacket:Subpacket = Subpacket { children: [].to_vec(), value: Some(current_value), parent:current_parent, id:last_packet_id};
                                packet[current_parent].children.push(last_packet_id);
                                packet.push(newpacket);
                                current_value = -1;
                            }
                        }else{
                            if current_value < 0{
                                current_value = 0;
                            }
                            current_value*=10;
                            current_value += (c as i32) - ('0' as i32);
                        }
                    }
                    println!("{}",packet[0].print(&packet));
                    packets.push(packet);
                    if packets.len() == 2 {
                        index_count += 1;
                        let res:i32 = compare_packets(&packets[0], &packets[1], &packets[0][0], &packets[1][0]);
                        if res >= 0 {
                            index_sum += index_count;
                        }
                        println!("index count : {}, res : {}, index_sum : {}",index_count, res, index_sum);
                    }
                    
                }
            }
        }
    }else{
        println!("lines not ok");
    }
    println!("index sum : {}",index_sum);
    
    
}

pub fn part2() {
    let mut packets: Vec<Vec<Subpacket>> = [].to_vec();
    let mut index_count:usize = 0;
    let mut index_sum:usize = 0;
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                if line_content == ""{
                    //packets = [].to_vec();
                }else{
                    let mut packet: Vec<Subpacket> = [].to_vec();
                    let mut last_packet_id: usize = 0;
                    let mut parent_at_level: Vec<usize> = [0].to_vec();
                    let mut newpacket:Subpacket = Subpacket { children: [].to_vec(), value: None, parent:0, id:last_packet_id};
                    packet.push(newpacket);
                    let mut current_value = -1;
                    for c in line_content.chars(){
                        if c == '[' {
                            last_packet_id += 1;
                            let current_parent = parent_at_level[&parent_at_level.len()-1];
                            let mut newpacket:Subpacket = Subpacket { children: [].to_vec(), value: None, parent:current_parent, id:last_packet_id};
                            parent_at_level.push(last_packet_id);
                            packet[current_parent].children.push(last_packet_id);
                            packet.push(newpacket); 
                        }else if c ==']' {
                            if current_value >= 0 {
                                last_packet_id += 1;
                                let current_parent = parent_at_level[&parent_at_level.len()-1];
                                let mut newpacket:Subpacket = Subpacket { children: [].to_vec(), value: Some(current_value), parent:current_parent, id:last_packet_id};
                                packet[current_parent].children.push(last_packet_id);
                                packet.push(newpacket);
                                current_value = -1;
                            }
                            parent_at_level.pop();
                        }else if c == ',' {
                            if current_value >= 0 {
                                last_packet_id += 1;
                                let current_parent = parent_at_level[&parent_at_level.len()-1];
                                let mut newpacket:Subpacket = Subpacket { children: [].to_vec(), value: Some(current_value), parent:current_parent, id:last_packet_id};
                                packet[current_parent].children.push(last_packet_id);
                                packet.push(newpacket);
                                current_value = -1;
                            }
                        }else{
                            if current_value < 0{
                                current_value = 0;
                            }
                            current_value*=10;
                            current_value += (c as i32) - ('0' as i32);
                        }
                    }
                    println!("{}",packet[0].print(&packet));
                    packets.push(packet);
                    index_count += 1;
                }
            }
        }
    }else{
        println!("lines not ok");
    }
    let mut sorted_packets: Vec<Vec<Subpacket>> = [].to_vec();
    while packets.len() > 0 {
        let n = packets.len();
        let packet: Vec<Subpacket> = packets[n-1].clone();
        packets.remove(n-1);
        let mut i:usize = 0;
        while (i<sorted_packets.len()) && (1 == compare_packets(&sorted_packets[i], &packet, &sorted_packets[i][0], &packet[0])){
            i += 1;
        }
        sorted_packets.insert(i, packet);
    }
    println!("{}",sorted_packets.len());
    println!("{}",packets.len());
    let mut final_res:usize = 1;
    let mut packet_2: Vec<Subpacket> = [].to_vec();
    let packet_from_value2 = Subpacket{children:[1].to_vec(),value:None,parent:0, id:0};
    packet_2.push(packet_from_value2);
    let subpacket_from_value2 = Subpacket{children:[2].to_vec(),value:None,parent:0, id:1};
    packet_2.push(subpacket_from_value2);
    let subsubpacket_from_value2 = Subpacket{children:[3].to_vec(),value:None,parent:1, id:2};
    packet_2.push(subsubpacket_from_value2);
    let subsubsubpacket_from_value2 = Subpacket{children:[].to_vec(),value:Some(2),parent:2, id:3};
    packet_2.push(subsubsubpacket_from_value2);
    for i in 0..sorted_packets.len() {
        //println!("{}",sorted_packets[i][0].print(&sorted_packets[i]));
        if (0 > compare_packets(&sorted_packets[i], &packet_2, &sorted_packets[i][0], &packet_2[0])){
            println!("i : {}",i+1);
            final_res *= (i+1);
            break;
        }
    }
    let mut packet_6: Vec<Subpacket> = [].to_vec();
    let packet_from_value6 = Subpacket{children:[1].to_vec(),value:None,parent:0, id:0};
    packet_6.push(packet_from_value6);
    let subpacket_from_value6 = Subpacket{children:[2].to_vec(),value:None,parent:0, id:1};
    packet_6.push(subpacket_from_value6);
    let subsubpacket_from_value6 = Subpacket{children:[3].to_vec(),value:None,parent:1, id:2};
    packet_6.push(subsubpacket_from_value6);
    let subsubsubpacket_from_value6 = Subpacket{children:[].to_vec(),value:Some(6),parent:2, id:3};
    packet_6.push(subsubsubpacket_from_value6);
    for i in 0..sorted_packets.len() {
        //println!("{}",sorted_packets[i][0].print(&sorted_packets[i]));
        if (0 > compare_packets(&sorted_packets[i], &packet_6, &sorted_packets[i][0], &packet_6[0])){
            println!("i : {}",i+2);
            final_res *= (i+2);
            break;
        }
    }
    println!("finale res : {}",final_res);
}