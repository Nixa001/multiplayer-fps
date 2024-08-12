
pub struct Enemys {
    pub id: i32,
    pub name: String,
    pub position: (usize, usize, usize)
}

impl Enemys {
    pub fn  new(id: i32, name: String, position: (usize, usize, usize)) -> Self {
        Enemys {
            id,
            name,
            position
        }
    }
}