use stats::Stats;
use syntax::{parse, visit, ast};

pub struct StatsBuilder {
    stats: Stats,
}

impl StatsBuilder {
    pub fn new() -> StatsBuilder {
        let mut stats = Stats::new();

        stats.add_metric("let_decls", "Let declarations");
        stats.add_metric("fn_decls", "Function declarations");

        StatsBuilder {
            stats: stats,
        }
    }
}

impl<'a> visit::Visitor<'a> for StatsBuilder {
    fn visit_mac(&mut self, macro: &'a ast::Mac) {
        visit::walk_mac(self, macro);
    }

    fn visit_local(&mut self, local: &'a ast::Local) {
        self.stats.incr_metric("let_decls", 1);

        visit::walk_local(self, local);
    }

    fn visit_item(&mut self, item: &'a ast::Item) {
        match item.node {
            ast::Item_::ItemFn(..) => {
                self.stats.incr_metric("fn_decls", 1);
            }
            _ => {}
        }

        visit::walk_item(self, item);
    }
}

pub fn parse(path: &Path) -> Option<Stats> {
    let sess = parse::new_parse_sess();
    let krate = parse::parse_crate_from_file(path, Vec::new(), &sess);

    let mut builder = StatsBuilder::new();
    visit::walk_crate(&mut builder, &krate);

    Some(builder.stats)
}
