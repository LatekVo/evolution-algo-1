/* * * * * * * * *\
 *
 *  evolution-version-2
 *
 *  Ignacy Łątka [Armado/LatekVon]
 *
 * * * * * * * * * 
 *  This version of evolving algorithm has some notable
 *  improvements, it has ai saving system, auto-expansion,
 *  and experimental sub-nodes - nodes with multiple
 *  calculations and calculation types inside.
 *
 *  This evolution of my algorithm is quite modular,
 *  i created quite a lot of redundant structs, which
 *  MAY be handful, but may cause some bottlenecks as well.
 *
 *  Could refactor this code to change structs to just tuples.
\* * * * * * * * */

/* * *\
 * If all nodes usage is high, add layer, if node usage is small,
 * TRY, but not force more aggresivly changing the value.
 *
 * My main drive here is no constants, but sometimes constant is
 * better than randomness, so SUB count will stay constant for now
\* * */

// ai cluster removed, as im trying to focus on having single instance
// of ai, that just evolves, not many pararell. 
// To accompany for this, i need:
//  -node adjusting their Type, new chance for unused nodes?
//  or
//  -consistant nodes
//  or
//  -manual selection of ai

//do i need this?
fn _avg(vec: Vec<f32>) -> f32 {
    let mut sum: f32 = 0.0;
    for i in 0..vec.len() {
        sum += vec[i];
    }
    sum / vec.len() as f32
}


mod ai_module;
mod image_rw;

//any 'use' in here are probably temporary or for testing porpouses
use image_rw::*;

fn main() -> std::io::Result<()> {

    let mut ai: ai_module::Ai = ai_module::Ai::new(480, 12); 
    ai.list();
    let mut inp: Vec<f32> = Vec::new();
    inp.push(4.4);
    inp.push(2.2);
    inp.push(2.2);
    inp.push(2.2);
    inp.push(2.2);
    let out = ai.calculate(inp);
   
    ai.list();

    //get training data
    let training_data: Vec<DataNode> = {
        let mut td: Vec<DataNode> = Vec::new();
        
        for i in 0..0 {
            let mut out: Vec<f32> = Vec::new();
        }
        for i in 0..0 {
            let mut inp: Vec<f32> = Vec::new();
        }
        td
    };

    //run until something happens lol
    ai.train(training_data);

    println!("{:?}", out);

    Ok(())
}
