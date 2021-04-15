//use std::io;
//use std::io::prelude::*;
//use std::fs::File;
use std::fs;

use super::ai_module as ai_module;

// will change to ai_module::Ai and use setter vvv
pub fn save(name: String, ai: &Vec<Vec<ai_module::Node>>) {
    
    //NOTE: Hyper-important, first save off and then mult. Vec are separated by ','

    let mut name: String = name.clone();
    name.push_str(".save");

    let mut file = std::fs::File::create(&name); 
    
    //initial metadata
    let mut f_data: String = String::new();
    //node, len of first nested vec
    f_data.push_str(&ai[0].len().to_string());
    f_data.push(';');
    //layer, len of vec, redundant
    f_data.push_str(&ai.len().to_string());
    f_data.push(';');

    fs::write(&name, f_data.as_bytes()).unwrap();
   
    //actual ai
    for l in 0..ai.len() {
        for n in 0..ai[l].len() {
            //im dumb, project halted, 14th april 2021, in unusable state
            for i in ai[l][n].inp_off.len() {
                fs::write(&name, ai[l][n].inp_off[i].as_bytes()).unwrap();
                if i != ai[l][n].inp_off.len() - 1 {
                    fs::write(&name, ",".as_bytes()).unwrap();
                } else { 
                    fs::write(&name, ";".as_bytes()).unwrap();
                }
            }
        }
    }

}
pub fn load(f_name: &str) -> Option<ai_module::Ai> {
    let mut name: String = String::from("ai_files/");
    name.push_str(f_name);
    name.push_str(".save");

    //first few characters are already parsed from start
    
    //metadata
    let mut n: String = String::new();
    let mut l: String = String::new();
    //let _gen: String = String::new();
    
    let mut ai_s: String;
    if std::path::Path::new(&name).exists() {
        
        ai_s = String::from(std::fs::read_to_string(name).expect("could not open file").trim());
        //initial metadata loop   
        
        println!("Loaded ai: {:?}", ai_s); //DEBUG
        
        let mut n_b = true;
        //let mut l_b = true;
        //let mut gen_b = true;

        loop {
            println!("{:?}", ai_s); //DEBUG
            let c = ai_s.chars().nth(0).expect("ai_rw -> Opened file is empty or too short");
            ai_s.remove(0);
            println!("'c': '{}'", c);

            //tried using 'continue' but it was broken, it skipped 'break' but didn't skip 'if',
            //resolved to using just if-else instead of if-continue-if
            if n_b {
                if c == ';' {
                    n_b = false;
                } else { 
                    n.push(c);
                }
            } else {
                if c == ';' {
                    break; 
                } else { 
                    l.push(c);
                }
            }
            
        }


    } else {
        //quiet error
        return None; 
    }

    //metadata: String to usize 
    println!("parsing metadata: 'n': '{}' and 'l': '{}'", n, l);
    let n: usize = n.parse().unwrap();
    let l: usize = l.parse().unwrap();
    //let _gen: usize = gen.parse().unwrap();

    /*             *\
     * Parser part *
    \*             */
    let mut ai_o = ai_module::Ai::new(n, l);
    
    //all following values are temporary and change every 3/4 loops over 'main
    let mut order: usize = 0; //decides which of Node's values will be set
    let mut tmp_action: usize = 0;
    let mut tmp_inp_off: Vec<f32> = Vec::new();
    let mut tmp_inp_mult: Vec<f32> = Vec::new();
    tmp_inp_off.resize(n, 0.0);
    tmp_inp_mult.resize(n, 0.0); 
    let mut layer_v: Vec<ai_module::Node> = Vec::new();
    let node_cnt: usize = 0;
    //all these values will ONLY be overwritten, so manually editing the save file may lead to UB

    // check values, store them temporarly, then pump all of temps to Node.
    let mut val: String = String::new();
    loop {
       
        //choose first char
        let c = match ai_s.chars().nth(0) {
            //if there is a char, remove it and assign it, if not, break
            Some(x) => { 
                ai_s.remove(0);
                x 
            },
            _ => {
                break;
            }
        };
        
        //actual computation
        match c {
            //if end of value is reached, check to which of Node's value it should be set to
            ';' | ',' => {
                match order {
                    0 => {
                        tmp_action = val.parse().unwrap(); 
                    },
                    //1, 2 is for vec, so its not changed *that* frequently
                    1 | 2 => {
                        match order {
                            1 => {
                                tmp_inp_off.push(val.parse().unwrap());
                            },
                            2 => {
                                tmp_inp_mult.push(val.parse().unwrap());
                            },
                            _ => {
                                panic!("this... this cannot be explained");
                            }
                        }
                    },
                    _ => {
                        //this will be called only in case of UB
                        panic!("Came across very unusual behaviour at parsing loop");
                    }
                }
                //if its ',' value belongs to the same vector
                if c == ';' {
                    order += 1;
                }
                if order > 2 {
                    
                    //hmm, so now we have a single node, what am i supposed to do with it
                    //so that node is pushed to ai, and after whole row of them is there,
                    //i send a setter to add new layer
                    let new_node = ai_module::Node::from(
                        match tmp_action {
                            //coppied from impl Node
                            0 => ai_module::Type::Add,
                            1 => ai_module::Type::Mult,
                            2 => ai_module::Type::InvAdd,
                            3 => ai_module::Type::InvMult,
                            4 => ai_module::Type::Ifs,
                            _ => {
                                panic!("Unexpected number at node creation")
                            }
                        },
                        tmp_inp_off,
                        tmp_inp_mult
                    );
                    //if all fields are filled, create new node and push to vector
                    
                    ai_o.push_node(new_node);
                    node_cnt += 1;
                    //TODO: Add layer and node counting system
                    /*
                     * if node_cnt > nodes {
                     *      ai_o.add_layer();
                     * }
                     */
                    order = 0;
                }
            }
            _ => {
                val.push(c);
            }
        }
    }

    //gonna add this later
    //ai_o.set_gen(gen);

     
    Some(ai_o)
}
