#[derive(Debug,PartialEq,Eq)]
pub struct DeoxyribonucleicAcid(String);

impl DeoxyribonucleicAcid {
    #![warn(dead_code)]
    pub fn new(strand: &str) -> DeoxyribonucleicAcid {
        DeoxyribonucleicAcid(strand.to_string())
    }

    pub fn to_rna(&self) -> RibonucleicAcid {
        RibonucleicAcid::new(&self.0
            .chars()
            .map(|x| match x {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => unreachable!(),
            })
            .collect::<String>())
    }
}

#[derive(Debug,PartialEq,Eq)]
pub struct RibonucleicAcid(String);

impl RibonucleicAcid {
    #![warn(dead_code)]
    pub fn new(strand: &str) -> RibonucleicAcid {
        RibonucleicAcid(strand.to_string())
    }
}
