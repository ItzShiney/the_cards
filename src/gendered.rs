#[derive(Clone, Copy)]
pub enum RuGender {
    Masculine,
    Feminine,
    Neuter,
    Plural,
}

#[derive(Clone, Copy)]
pub struct Gendered<T> {
    pub ru_gender: RuGender,
    pub value: T,
}
