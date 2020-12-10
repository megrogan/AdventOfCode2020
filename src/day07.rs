use std::collections::HashMap;
use std::fs;

pub fn run() -> (usize, u64) {
    let input = fs::read_to_string("input/day7.txt").unwrap();

    let graph = build_graph(&input);
        
    (
        graph.count_bags_containing("shiny gold"), 
        graph.count_bags_contained_in("shiny gold")
    )
}

fn build_graph(input: &str) -> BagGraph {
    let rules: Vec<_> = input
        .lines()
        .filter_map(BagRule::parse)
        .collect();

    BagGraph::build(rules)
}

struct BagGraph {
    map: HashMap<String, Vec<(String, u32)>>
}

impl BagGraph {
    pub fn build(rules: Vec< BagRule>) -> BagGraph {
        
        let mut graph = HashMap::new();
    
        for rule in rules {
            graph.insert(rule.name, rule.bags);
        }
    
        BagGraph {
            map: graph
        }
    }

    pub fn count_bags_containing(&self, bag: &str) -> usize {

        fn is_bag_inside(graph: &BagGraph, outer: &str, inner: &str) -> bool {

            if outer == inner {
                return true;
            }

            let children = graph.map.get(outer);

            if children.is_none() {
                return false;
            }

            children
                .unwrap()
                .iter()
                .any(|child|is_bag_inside(graph, &child.0, inner))
        }

        self
            .map
            .iter()
            .filter(|p| p.0 != bag)
            .filter(|p| is_bag_inside(self, p.0.as_str(), bag))
            .count()
    }    

    pub fn count_bags_contained_in(&self, bag: &str) -> u64 {

        let children = self.map.get(bag);

        if children.is_none() {
            return 0;
        }

        children
            .unwrap()
            .iter()
            .map(|child| (child.1 as u64) * (1 + self.count_bags_contained_in(&child.0)))
            .sum()
    }    
}

struct BagRule {
    name: String,
    bags: Vec<(String, u32)>
}

impl BagRule {
    fn parse(text: &str) -> Option<BagRule> {

        let mut parts = text.split("bags contain");

        let mut rule = BagRule {
            name: parts.next()?.trim().to_string(),
            bags: Vec::new()
        };

        let bags_text = parts.next()?.trim().trim_end_matches('.');

        if bags_text == "no other bags" {
            return Some(rule);
        }

        for bag_text in bags_text.split(",") {
            let mut words = bag_text.split_whitespace();
            let count = words.next()?.trim().parse::<u32>().ok()?;
            let name = format!(
                "{} {}",
                words.next()?.trim(),
                words.next()?.trim());
            rule.bags.push((name, count));
        }

        Some(rule)
    }
}

#[cfg(test)]
mod tests {
    use crate::day07::*;

    #[test]
    fn test1() {
        let input = r"
            shiny gold bags contain 2 dark red bags.
            dark red bags contain 2 dark orange bags.
            dark orange bags contain 2 dark yellow bags.
            dark yellow bags contain 2 dark green bags.
            dark green bags contain 2 dark blue bags.
            dark blue bags contain 2 dark violet bags.
            dark violet bags contain no other bags.  
            ";

        let graph = build_graph(input);

        assert_eq!(126, graph.count_bags_contained_in("shiny gold"));
    }
}
