pub mod basic_combinator{
    #[derive(Debug,Clone)]
    pub struct BasicCombinator<'a> {
        name: &'a str,
        label: u8,
        main_port: Option<&'a Self>,
        aux_port: Option<Vec<Option<&'a Self>>>,
        port_num: usize,
        next_free: Option<usize>,
    }

    impl <'a> BasicCombinator<'a> {
        pub fn new(name: &'a str, label: u8, aux_port: Option<Vec<Option<&'a Self>>> ) -> Self {
            Self{
                label: label,
                name: name,
                main_port: None,
                port_num :{
                    match &aux_port {
                        None => 1,
                        Some(sl) =>{
                            sl.len() + 1
                        },
                    }
                },
                aux_port: aux_port,
                next_free : Some(0),
            }
        }

        pub fn get_free_port(&self) -> Option<usize> {
            self.next_free
        }

        pub fn nex_free_port(&mut self){
            if let Some(nf) = self.next_free{
                if nf < self.port_num{
                    self.next_free = Some(nf+1)
                }else{
                    self.next_free = None
                }
            }
        }

        pub fn link(&mut self,node:  &'a BasicCombinator<'a>, port: usize) {
            if port < self.port_num {
                match port {
                    0 => {
                        self.main_port = Some(node)
                    },
                    x =>{
                        if let Some(ref mut buf) = self.aux_port {
                            buf[x] = Some(node)
                        }
                    },
                    
                }              
            }
        }

        pub fn get_name(&self) -> &'a str{
            self.name
        }

        pub fn get_label(&self) -> u8{
            self.label
        }
    }
}
