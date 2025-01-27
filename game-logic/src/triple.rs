use crate::single::Single;

#[derive(Debug, Clone)]
pub struct Triple {
    pub top: Single,
    pub mid: Single,
    pub bot: Single,
}

impl Triple {
    pub fn new(top: Single, mid: Single, bot: Single) -> Self {
        Self { top, mid, bot }
    }
    pub fn rotate_up(&mut self) {
        let temp = self.top.clone();
        self.top = self.mid.clone();
        self.mid = self.bot.clone();
        self.bot = temp;
    }
    pub fn from_random(top: u64, mid: u64, bot: u64) -> Triple {
        let top = Single::from_random(top);
        let mid = Single::from_random(mid);
        let bot = Single::from_random(bot);

        Self { top, mid, bot }
    }
}

macro_rules! triple {
    ($i:ident, $j:ident) => {
        Triple::new($i, $j, $k)
    };
}
