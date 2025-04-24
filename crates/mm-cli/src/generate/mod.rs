#[derive(Debug)]
pub struct ProjectConfig {
    /// project name
    pub name: String,
    /// project language
    pub language: String,
    /// keywords
    pub keywords: Vec<String>,
}

pub fn generate(project_config: ProjectConfig) -> anyhow::Result<()> {
    println!("generate project_config: {:?}", &project_config);
    // path+ project_name exists ?

    Ok(())
}
