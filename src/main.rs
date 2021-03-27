/* * * * * * * * * * * * * * * * * 
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
 * * * * * * * * */

/* * *
 * If all nodes usage is high, add layer, if node usage is small,
 * TRY, but not force more aggresivly changing the value.
 *
 * My main drive here is no constants, but sometimes constant is
 * better than randomness, so SUB count will stay constant for now
 * * */

mod ai_module;

fn main() -> std::io::Result<()> {
    
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

    println!("{:?}", out);

    Ok(())
}
