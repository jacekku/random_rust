use std::fmt::Error;

#[derive(PartialEq, Debug)]
pub enum FlowType<'a> {
    Standalone(&'a str),
    Standard(&'a str, &'a str, &'a str),
}
pub struct Diagram<'a> {
    title: &'a str,
    flows: Vec<FlowType<'a>>,
}

pub fn parse_diagram(diagram_description: &str) -> Result<Diagram, Error> {
    let mut title: &str = "Untitled";
    let title_pattern = "title:";
    let mut flows = Vec::new();
    for line in diagram_description.lines() {
        if let Some(idx) = line.find(title_pattern) {
            title = &line[(idx + title_pattern.len())..];
            title = title.trim();
        } else {
            let line = line.trim();
            if line.contains("->") {
                let mut split = line.split("->");
                let first = split.next().unwrap();
                let rest = split.next().unwrap();
                let mut rest = rest.split(":");
                let second = rest.next().unwrap();

                let description = match rest.next() {
                    Some(it) => it.trim(),
                    None => "",
                };

                flows.push(FlowType::Standard(first, second, description));
            } else {
                flows.push(FlowType::Standalone(line))
            }
        }
    }

    Ok(Diagram { title, flows })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_diagram_has_untitled_title() {
        let result = parse_diagram("").unwrap();
        assert_eq!(result.title, "Untitled");
    }

    #[test]
    fn given_title_has_valid_title() {
        let result = parse_diagram("title: Diagram").unwrap();
        assert_eq!(result.title, "Diagram");
    }

    #[test]
    fn given_single_identifier_returns_standalone_box() {
        let result = parse_diagram(
            "title: Diagram 
        Service",
        )
        .unwrap();
        assert_eq!(result.flows[0], FlowType::Standalone("Service"));
    }

    #[test]
    fn given_service_to_service_flow_returns_self_case() {
        let result = parse_diagram("Service->Service").unwrap();
        assert_eq!(
            result.flows[0],
            FlowType::Standard("Service", "Service", "")
        );
    }

    #[test]
    fn given_standard_with_description() {
        let result = parse_diagram("Service->Other Service: Does something").unwrap();
        assert_eq!(
            result.flows[0],
            FlowType::Standard("Service", "Other Service", "Does something")
        );
    }
}
