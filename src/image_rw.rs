use image::io::Reader as ImageReader;

const SIZE: (usize, usize) = (480, 480);

pub struct DataNode {
    size: (usize, usize),
    inp: Vec<Vec<f32>>,
    out: Vec<f32>
}

impl DataNode {
    fn new(s: (usize, usize)) -> DataNode {
        DataNode {
            size: s,
            inp: Vec::new(),
            out: Vec::new(),
        }
    }
}

//will both check and convert files
//this function was too simple to be a function,
//it was very limiting and yet uninfluencial
//  fn file_check(name: &str) -> Option<Data_node>;

   

pub fn gather_data() -> Vec<DataNode> {
    let mut nodes: Vec<DataNode> = Vec::new();
    
    //push number
    let out_pref: String = String::from("o_");
    //push designator and interator
    let _in_pref: String = String::from("i_");
   
    //search for output data
    let mut o_indx: usize = 0;
    loop {
        let mut name: String = out_pref.clone();
        name.push_str(&o_indx.clone().to_string());
        
        if std::path::Path::new(&name).exists() {
            let img = ImageReader::open(&name).unwrap().decode().unwrap();
    
            //typecast image to f32 vector
            let out_v: Vec<f32> = {
                //a bunch of 0->255 values
                let tmp = img.into_bytes();
                let mut vec: Vec<f32> = Vec::new();
                for i in 0..tmp.len() {
                    vec.push(tmp[i] as f32);
                }
                vec
            };
            
            //spawn datanode, and set output as our designated vector
            let mut node: DataNode = DataNode::new(SIZE); //SIZE is temporary fix
            node.out = out_v; 
            nodes.push(node); //and add our DataNode to node registery
            o_indx += 1;

        } else {
            break; 
        }   
    }
    
    //o_indx is not node count, but node index.
    //it gets incremented by 1 before, now its unnecesary
    for i in 0..o_indx {

    }

    nodes

}
