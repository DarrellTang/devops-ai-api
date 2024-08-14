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
                suggested_questions: vec![
                    "How is GitHub different from Git?".to_string(),
                    "Why do developers use GitHub?".to_string(),
                    "Is GitHub only for programmers?".to_string(),
                ],
            },
            Step {
                title: "Create a GitHub account".to_string(),
                prompt: "Outline a concise, step-by-step guide on how to create a GitHub account. Focus only on the essential steps, keeping the instructions clear and easy to follow for new users.".to_string(),
                suggested_questions: vec![
                    "What information do I need to create a GitHub account?".to_string(),
                    "Is it free to create a GitHub account?".to_string(),
                    "Can I use my work email to sign up?".to_string(),
                ],
            },
            Step {
                title: "Install Git on your local machine".to_string(),
                prompt: "Explain how to install Git on a local machine. Provide clear instructions for common operating systems (Windows, macOS, Linux). Keep the explanation concise but informative.".to_string(),
                suggested_questions: vec![
                    "How do I check if Git is already installed?".to_string(),
                    "Are there different installation methods for Windows and Mac?".to_string(),
                    "Do I need admin rights to install Git?".to_string(),
                ],
            },
            Step {
                title: "Set up SSH keys for secure authentication".to_string(),
                prompt: "Provide a brief, step-by-step guide on how to set up SSH keys for GitHub authentication. Ensure the instructions are clear and easy to follow for users who might be new to this concept.".to_string(),
                suggested_questions: vec![
                    "Why should I use SSH keys instead of passwords?".to_string(),
                    "Can I use the same SSH key for multiple GitHub accounts?".to_string(),
                    "What if I lose my SSH key?".to_string(),
                ],
            },
            Step {
                title: "Configure Git with your GitHub credentials".to_string(),
                prompt: "Explain how to configure Git with GitHub credentials. Focus on the essential commands, providing clear instructions for users to follow. Include any necessary explanations of what each command does.".to_string(),
                suggested_questions: vec![
                    "How do I update my Git configuration if I change my GitHub username?".to_string(),
                    "Can I use different Git configurations for different projects?".to_string(),
                    "What's the difference between local and global Git configurations?".to_string(),
                ],
            },
            Step {
                title: "Create your first repository".to_string(),
                prompt: "Describe how to create a new repository on GitHub. Cover only the basic steps, ensuring the instructions are clear and concise for new users.".to_string(),
                suggested_questions: vec![
                    "What's the difference between public and private repositories?".to_string(),
                    "Should I initialize the repository with a README?".to_string(),
                    "How do I choose a good name for my repository?".to_string(),
                ],
            },
            Step {
                title: "Clone the repository to your local machine".to_string(),
                prompt: "Explain how to clone a GitHub repository to a local machine. Include the basic command and a brief explanation of what cloning means and why it's important.".to_string(),
                suggested_questions: vec![
                    "Can I clone someone else's repository?".to_string(),
                    "What's the difference between cloning with HTTPS and SSH?".to_string(),
                    "Where should I clone my repository to on my local machine?".to_string(),
                ],
            },
            Step {
                title: "Make changes and commit them".to_string(),
                prompt: "Provide instructions on how to make changes to files and commit them using Git. Focus on the essential commands, explaining each step clearly for new users.".to_string(),
                suggested_questions: vec![
                    "What's a good practice for writing commit messages?".to_string(),
                    "How often should I commit my changes?".to_string(),
                    "Can I undo a commit?".to_string(),
                ],
            },
            Step {
                title: "Push changes to GitHub".to_string(),
                prompt: "Explain how to push local commits to GitHub. Include the basic command and a brief explanation of what pushing means in the context of Git and GitHub.".to_string(),
                suggested_questions: vec![
                    "What happens if someone else pushed changes before me?".to_string(),
                    "Can I push to someone else's repository?".to_string(),
                    "How do I know if my push was successful?".to_string(),
                ],
            },
            Step {
                title: "Create a branch and make a pull request".to_string(),
                prompt: "Describe how to create a branch and make a pull request on GitHub. Cover the essential steps, explaining the concepts of branching and pull requests in a way that's easy for beginners to understand.".to_string(),
                suggested_questions: vec![
                    "Why should I create a branch instead of working on the main branch?".to_string(),
                    "How do I name my branches?".to_string(),
                    "What happens after I create a pull request?".to_string(),
                ],
            },
            Step {
                title: "Collaborate on a project".to_string(),
                prompt: "Provide an overview of how to start collaborating on a GitHub project. Mention key concepts like forking and contributing, explaining them in a way that's accessible to new users. Include basic steps for getting involved in open-source projects.".to_string(),
                suggested_questions: vec![
                    "How do I find projects to contribute to?".to_string(),
                    "What's the difference between forking and cloning?".to_string(),
                    "How do I suggest changes to someone else's project?".to_string(),
                ],
            },
        ],
    }
}
