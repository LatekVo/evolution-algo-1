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
pub fn load() -> ai_module::Ai {


}
