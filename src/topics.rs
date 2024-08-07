use crate::types::{Topic, Step};

pub fn get_all_topics() -> Vec<Topic> {
    vec![get_github_setup_topic()]
}

pub fn get_github_setup_topic() -> Topic {
    Topic {
        id: "github-setup".to_string(),
        title: "GitHub Setup".to_string(),
        description: "Learn how to set up your GitHub account and start using Git".to_string(),
        initial_message: r#"# Welcome to the GitHub Setup Guide!

Here's a quick overview of how to use this tutorial:

1. **Chat Window**: This is where we'll interact. I'll provide instructions and you can ask questions.

2. **Next Step**: Use the 'Next Step' button at the bottom right to progress through the tutorial.

3. **Ask Questions**: Feel free to type any questions or ask for clarification at any time.

**Let's begin!** Click the 'Next Step' button to start your GitHub setup journey."#.to_string(),
        steps: vec![
            Step {
                title: "Introduction to GitHub".to_string(),
                prompt: "Provide a brief introduction to GitHub, explaining what it is and its main purposes for developers. Keep the explanation simple and engaging for beginners.".to_string(),
            },
            Step {
                title: "Create a GitHub account".to_string(),
                prompt: "Outline a concise, step-by-step guide on how to create a GitHub account. Focus only on the essential steps, keeping the instructions clear and easy to follow for new users.".to_string(),
            },
            Step {
                title: "Install Git on your local machine".to_string(),
                prompt: "Explain how to install Git on a local machine. Provide clear instructions for common operating systems (Windows, macOS, Linux). Keep the explanation concise but informative.".to_string(),
            },
            Step {
                title: "Set up SSH keys for secure authentication".to_string(),
                prompt: "Provide a brief, step-by-step guide on how to set up SSH keys for GitHub authentication. Ensure the instructions are clear and easy to follow for users who might be new to this concept.".to_string(),
            },
            Step {
                title: "Configure Git with your GitHub credentials".to_string(),
                prompt: "Explain how to configure Git with GitHub credentials. Focus on the essential commands, providing clear instructions for users to follow. Include any necessary explanations of what each command does.".to_string(),
            },
            Step {
                title: "Create your first repository".to_string(),
                prompt: "Describe how to create a new repository on GitHub. Cover only the basic steps, ensuring the instructions are clear and concise for new users.".to_string(),
            },
            Step {
                title: "Clone the repository to your local machine".to_string(),
                prompt: "Explain how to clone a GitHub repository to a local machine. Include the basic command and a brief explanation of what cloning means and why it's important.".to_string(),
            },
            Step {
                title: "Make changes and commit them".to_string(),
                prompt: "Provide instructions on how to make changes to files and commit them using Git. Focus on the essential commands, explaining each step clearly for new users.".to_string(),
            },
            Step {
                title: "Push changes to GitHub".to_string(),
                prompt: "Explain how to push local commits to GitHub. Include the basic command and a brief explanation of what pushing means in the context of Git and GitHub.".to_string(),
            },
            Step {
                title: "Create a branch and make a pull request".to_string(),
                prompt: "Describe how to create a branch and make a pull request on GitHub. Cover the essential steps, explaining the concepts of branching and pull requests in a way that's easy for beginners to understand.".to_string(),
            },
            Step {
                title: "Collaborate on a project".to_string(),
                prompt: "Provide an overview of how to start collaborating on a GitHub project. Mention key concepts like forking and contributing, explaining them in a way that's accessible to new users. Include basic steps for getting involved in open-source projects.".to_string(),
            },
        ],
    }
}
