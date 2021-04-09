//use std::io;
//use std::io::prelude::*;
//use std::fs::File;
//use std::fs;

use super::ai_module as ai_module;

/* General save idea:
    n, l, l(layers( n(Type, off, mult))
    
    here is Ai for reference

    Ai {
        inp: Vec<f32>, // will be used as storage, but never read by struct impl
        brain: Vec<Vec<Node>>, //Vec<Layers<Nodes>>
        out: Vec<f32>, //may be unnecesary
    }

    so we only need vec of nodes.
   
    Node {
        inp_mult: Vec<f32>, //input multipliers
        inp_off: Vec<f32>, //input addition
        out: f32, //this one is changed every calculation
        out_base: f32, //only read once every calculation, never changed
        action: Type,
    }

    inp isnt used by Ai, and is there only for storage
    So:

    Ai {
        brain: 
            Layers< 
                Nodes< 
                    Node {
                        inp_mult: Vec<f32>
                        inp_off: Vec<f32>
                        action: Type
                    } 
                > 
            >
    }

    i ignored out, and out_base, as these are redundant, can be detrived from Type
*/
pub fn save(ai: &ai_module::Ai) {
     

}
pub fn load(name: &str) -> Option<ai_module::Ai> {
 
    //first few characters are already parsed from start
    
    //metadata
    let n: String = String::new();
    let l: String = String::new();
    let gen: String = String::new();
    
    let ai_s: String;
    let mut ai_v: Vec<&str> = Vec::new();
    if std::path::Path::new(name).exists() {
        
        ai_s = String::from(std::fs::read_to_string(name).unwrap().trim());
        //initial metadata loop   
        
        let n_b = true;
        let l_b = true;
        let _gen_b = true;

        loop {
            let c = ai_s.chars().nth(0).expect("ai_rw -> Opened file is empty");
            ai_s.remove(0);

            if n_b {
                if c == ';' {
                    n_b = false;
                    continue;
                } else { 
                    n.push(c);
                }
            }
            if l_b {
                if c == ';' {
                    l_b = false;
                    continue;
                } else { 
                    n.push(c);
                }
            }

            break;

        }


    } else {
        //return quiet error
        return None; 
    }

    //metadata: &str to usize 
    let n: usize = n.parse().unwrap();
    let l: usize = l.parse().unwrap();
    let gen: usize = gen.parse().unwrap();

    ai_v.push(&ai_s);
    ai_v.split(|c| c.chars().next().unwrap() == ';');
    //Parser part
    let ai_o = ai_module::Ai::new(n, l);
   
    println!("{:?}", ai_v); //DEBUG

    //gonna add this later
    //ai_o.set_gen(gen);

    Some(ai_o)
}
