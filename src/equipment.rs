
///Define basic parts of Equipment.
pub trait Equipment{
    fn name(&self) -> &String;

    fn upgrade(&self) -> Result<(), ()>;
}