
#[deriving(Show, Clone, Encodable, Decodable)]
pub struct Stats {
    pub metrics: Vec<Metric>,
}

#[deriving(Show, Clone, Encodable, Decodable)]
pub struct Metric {
    pub id: String,
    pub name: String,
    pub value: uint,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            metrics: Vec::new(),
        }
    }

    pub fn get_metric(&self, id: &str) -> Option<&Metric> {
        self.metrics.iter().find(|metric| metric.id == id)
    }

    pub fn get_metric_mut(&mut self, id: &str) -> Option<&mut Metric> {
        self.metrics.iter_mut().find(|metric| metric.id == id)
    }

    pub fn add_metric(&mut self, id: &str, name: &str) {
        if let None = self.get_metric(id) {
            self.metrics.push(Metric::new(id, name));
        }
    }

    pub fn incr_metric(&mut self, id: &str, amount: uint) {
        if let Some(ref mut metric) = self.get_metric_mut(id) {
            metric.incr(amount);
        }
    }
}

impl Metric {
    pub fn new(id: &str, name: &str) -> Metric {
        Metric {
            id: id.into_string(),
            name: name.into_string(),
            value: 0,
        }
    }

    pub fn incr(&mut self, amount: uint) -> &mut Metric {
        self.value += amount;
        self
    }
}
