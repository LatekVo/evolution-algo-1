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

fn avg(vec: Vec<f32>) -> f32 {
    let mut sum: f32 = 0.0;
    for i in 0..vec.len() {
        sum += vec[i];
    }
    sum / vec.len() as f32
}


mod ai_module;
mod image_rw;

fn main() -> std::io::Result<()> {
    
    image_rw::test();

    let mut ai: ai_module::Ai = ai_module::Ai::new(5, 5); 
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
    let training_data: Vec<(Vec<f32>, Vec<f32>)> = {
        
        let mut td: Vec<(Vec<f32>, Vec<f32>)> = Vec::new();
        
        for _ in 0..0 {
            let mut inp: Vec<f32> = Vec::new();
            let mut out: Vec<f32> = Vec::new();

            inp.push(1.0);
            inp.push(1.0);
            out.push(2.0);
            out.push(3.0);
            td.push((inp, out));
        }
        td
    };

    //run until something happens lol
    ai.train(training_data);


    println!("{:?}", out);

    Ok(())
}
