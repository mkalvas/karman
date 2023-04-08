use anyhow::Context;
use karman::{Message, Node};
use serde_json::Deserializer;

fn main() -> anyhow::Result<()> {
    let inlock = std::io::stdin().lock();
    let strin = Deserializer::from_reader(inlock).into_iter::<Message>();
    let mut outlock = std::io::stdout().lock();
    let mut node = Node::new();
    node.run(strin, &mut outlock)
        .context("failed running node")?;
    Ok(())
}
