mod generic_combinator;

pub mod zero;
pub mod inc;
pub mod dec;

pub trait GenericCombinator{
    fn get_lable_id(&self) -> u8;
    fn get_lable_name(&self) -> &str;
}
