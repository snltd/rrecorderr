use bigdecimal::BigDecimal;

#[derive(Debug)]
pub(crate) struct RecordToAssay {
    pub(crate) id: i32,
    pub(crate) discogs_uri: Option<String>,
    pub(crate) title: Option<String>,
}

pub enum PossibleValues {
    VgPlusValue(BigDecimal),
    CheapestUk(BigDecimal),
    CheapestWorld(BigDecimal),
}
