use anyhow::Context;
use karman::Node;

fn main() -> anyhow::Result<()> {
    let stdin_lock = std::io::stdin().lock();
    let stdout_lock = std::io::stdout().lock();
    let mut node = Node::new(stdout_lock);
    node.run(stdin_lock).context("failed running node")?;
    Ok(())
}
