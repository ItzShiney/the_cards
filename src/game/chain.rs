pub enum Chain<Continue, Result = ()> {
    Continue(Continue),
    Break(Result),
}

#[macro_export]
macro_rules! terminate {
    () => {
        return $crate::game::chain::Chain::Break(Err($crate::game::Terminated))
    };
}
