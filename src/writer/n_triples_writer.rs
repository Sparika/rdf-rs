use writer::formatter::rdf_formatter::*;
use writer::formatter::n_triples_formatter::NTriplesFormatter;
use writer::rdf_writer::RdfWriter;
use graph::Graph;
use node::Node;
use triple::*;
use error::*;
use Result;


pub struct NTriplesWriter {
  formatter: NTriplesFormatter
}

impl RdfWriter for NTriplesWriter {
  fn write_to_string(&self, graph: &Graph) -> Result<String> {
    let mut output_string = "".to_string();

    for triple in graph.triples_iter() {
      match self.triple_to_n_triples(&triple) {
        Ok(str) => output_string.push_str(&str),
        Err(error) => return Err(error),
      }
    }

    Ok(output_string)
  }
}


impl NTriplesWriter {
  pub fn new() -> NTriplesWriter {
    NTriplesWriter {
      formatter: NTriplesFormatter::new()
    }
  }

  pub fn triple_to_n_triples(&self, triple: &Triple) -> Result<String> {
    let mut output_string = "".to_string();

    match self.node_to_n_triples(triple.subject(), TripleSegment::Subject) {
      Ok(str) => output_string.push_str(&str),
      Err(error) => return Err(error),
    }

    output_string.push_str(" ");

    match self.node_to_n_triples(triple.predicate(), TripleSegment::Predicate) {
      Ok(str) => output_string.push_str(&str),
      Err(error) => return Err(error),
    }

    output_string.push_str(" ");

    match self.node_to_n_triples(triple.object(), TripleSegment::Object) {
      Ok(str) => output_string.push_str(&str),
      Err(error) => return Err(error),
    }

    output_string.push_str(" .");

    Ok(output_string)
  }


  pub fn node_to_n_triples(&self, node: &Node, segment: TripleSegment) -> Result<String> {
    // todo: literal node either data type or language
    match node {
      &Node::BlankNode { id: _ } =>
        if segment == TripleSegment::Predicate {
          return Err(Error::InvalidWriterOutput)
        },
      _ => {},
    }

    let formatter = NTriplesFormatter::new();

    Ok(formatter.format_node(node))
  }
}


#[cfg(test)]
mod tests {
  use node::Node;
  use triple::*;
  use writer::formatter::n_triples_formatter::NTriplesFormatter;
  use uri::Uri;
  use writer::n_triples_writer::NTriplesWriter;

  #[test]
  fn test_n_triples_writer() {

    let subject = Node::BlankNode { id: "blank".to_string() };
    let object = Node::LiteralNode { literal: "literal".to_string(), prefix: None, data_type: None, language: Some("en".to_string()) };
    let predicate = Node::UriNode { uri: Uri::new("http://example.org/show/localName".to_string()) };

    let trip = Triple::new(subject, predicate, object);

    let result = "_:blank <http://example.org/show/localName> \"literal\"@en .".to_string();

    let writer = NTriplesWriter::new();
    match writer.triple_to_n_triples(&trip) {
      Ok(str) => assert_eq!(result, str),
      Err(err) => assert!(false)
    }
  }
}