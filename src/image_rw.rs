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

    let mut o_indx: usize = 0;
    loop {
        let mut name: String = out_pref.clone();
        name.push_str(&o_indx.clone().to_string());
        
        let exists = std::path::Path::new(&name).exists();
        if exists {
            let img = ImageReader::open(&name).unwrap().decode().unwrap();

            //this *may* not be the brigtest idea, still, gonna convert this, 
            //Ai *should* handle this al'right
            let out_v: Vec<f32> = {
                let tmp = img.into_bytes();
                let mut vec: Vec<f32> = Vec::new();
                for i in 0..tmp.len() {
                    vec.push(tmp[i] as f32);
                }
                vec
            };
            
            let mut node: DataNode = DataNode::new(SIZE); 
            node.out = out_v; 
            nodes.push(node);
            o_indx += 1;

        } else {
            break; 
        }   
    }
    
    nodes

}
