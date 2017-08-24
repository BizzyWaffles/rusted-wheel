pub trait ParseFrom<T, Out = Self> {
    fn parse (from: T) -> Result<Out, String>;
}
