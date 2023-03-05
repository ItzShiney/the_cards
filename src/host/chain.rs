pub enum Chain<Continue, Result = ()> {
    Continue(Continue),
    Break(Result),
}
