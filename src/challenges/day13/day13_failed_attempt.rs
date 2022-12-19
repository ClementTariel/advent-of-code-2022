use std::borrow::Borrow;
use std::rc::Rc;
use std::cell::RefCell;

#[path = "../../utils/mod.rs"] mod utils;

#[derive(Debug)]
struct Subpacket{
    children: Vec<Rc<RefCell<Subpacket>>>,
    value: i32,
    parent: Option<Rc<RefCell<Subpacket>>>
}

// impl Clone for Subpacket {
//     fn clone(&self) -> Self { 
//         Subpacket {
//             children: self.children,
//             value: self.value,
//             parent: self.parent
//         }
//     }
// }


impl Subpacket {
    pub fn add_subpacket(&mut self, new_node: Rc<RefCell<Subpacket>>) {
        self.children.push(new_node);
    }
    pub fn print(&self) -> String {
        if self.value < 0 {
          return String::from("[")
            + &self
              .children
              .iter()
              .map(|tn| tn.borrow().print())
              .collect::<Vec<String>>()
              .join(",")
            + "]";
        }else{
            return self.value.to_string();
        }
      }
}


pub fn part1() {
    let mut packets: Vec<Rc<RefCell<Subpacket>>> = [].to_vec();
    //let mut packet: Rc<RefCell<Subpacket>>;
    if let Ok(lines) = utils::lines_from_file("./small_input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                let mut children_at_level: Vec<i32> = [].to_vec();
                let mut packet: Rc<RefCell<Subpacket>> = Rc::new(RefCell::new(Subpacket{children:([].to_vec()),value:-1,parent:None}));
                let mut subpacket = Rc::clone(&packet);
                //let mut subpacket = packet.borrow_mut();
                let mut current_value = 0;
                for c in line_content.chars(){
                    if c == '[' {
                        children_at_level.push(0);
                        let mut newpacket = Rc::new(RefCell::new(Subpacket{children:([].to_vec()),value:-1,parent:None}));
                        subpacket.borrow_mut().add_subpacket(Rc::clone(&newpacket));
                        {
                            let mut mut_newpacket = newpacket.borrow_mut();
                            mut_newpacket.parent = Some(Rc::clone(&subpacket));
                        }
                        subpacket = newpacket;
                    }else if c ==']' {
                        children_at_level.pop();
                        let subpacket_clone = Rc::clone(&subpacket);
                        subpacket = Rc::clone(subpacket_clone.borrow().parent.as_ref().unwrap());
                        if current_value >= 0 {
                            let mut newpacket = Rc::new(RefCell::new(Subpacket{children:([].to_vec()),value:current_value,parent:None}));
                            subpacket.borrow_mut().add_subpacket(Rc::clone(&newpacket));
                            {
                                let mut mut_newpacket = newpacket.borrow_mut();
                                mut_newpacket.parent = Some(Rc::clone(&subpacket));
                            }
                            current_value = -1;
                        }
                    }else if c == ',' {
                        let n = children_at_level.len();
                        children_at_level[n-1] += 1;
                        if current_value >= 0 {
                            let mut newpacket = Rc::new(RefCell::new(Subpacket{children:([].to_vec()),value:current_value,parent:None}));
                            subpacket.borrow_mut().add_subpacket(Rc::clone(&newpacket));
                            {
                                let mut mut_newpacket = newpacket.borrow_mut();
                                mut_newpacket.parent = Some(Rc::clone(&subpacket));
                            }
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
                packets.push(packet);
                packet.borrow().print();
            }
        }
    }else{
        println!("lines not ok");
    }
    // for p in packets {
    //     println!("{p:?}");
    // }
    
}

pub fn part2() {
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                
            }
        }
    }else{
        println!("lines not ok");
    }
    
}
