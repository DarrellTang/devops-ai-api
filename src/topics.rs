use crate::types::{Topic, Step};

pub fn get_all_topics() -> Vec<Topic> {
    vec![get_github_setup_topic()]
}

pub fn get_github_setup_topic() -> Topic {
    Topic {
        id: "github-setup".to_string(),
        title: "GitHub Setup".to_string(),
        description: "Learn how to set up your GitHub account and start using Git".to_string(),
        initial_message: "Welcome to the GitHub Setup guide! This interactive tutorial will help you set up and use a GitHub account. Here's how it works:

1. Steps and Prompts:
   - On the left, you'll see a list of steps in your GitHub learning journey with a greyed out checkmark.
   - Each step is also a pre-written question that you can send to me, your AI assistant.
   - Clicking on a step will send its associated question, and I'll provide detailed instructions or information.

2. Learning Process:
   - Start with the first step and work your way down the list.
   - Click on a step to see instructions for that part of the setup process.
   - Follow the instructions and ask any additional questions you have in the chat.

3. Marking Progress:
   - After completing a step, click the checkmark icon next to the step itself to mark it as done.
   - This helps you keep track of your progress and tells me you're ready for the next step.

4. Flexibility:
   - If you're already familiar with some steps, you can mark them as complete and move on.

5. Additional Questions:
   - At any point, you can type your own questions in the chat for more clarification or help.

Remember, I'm here to assist you throughout the process. Don't hesitate to ask for more explanations or examples if something isn't clear.

Are you ready to begin? Click on the first step whenever you're ready to start your GitHub setup journey!".to_string(),
        steps: vec![
            Step {
                title: "Create a GitHub account".to_string(),
                prompt: "Provide a concise, step-by-step guide on how to create a GitHub account, focusing only on the essential steps.".to_string(),
            },
            Step {
                title: "Install Git on your local machine".to_string(),
                prompt: "Provide a short, clear explanation on how to install Git on a local machine, mentioning steps for common operating systems.".to_string(),
            },
            Step {
                title: "Set up SSH keys for secure authentication".to_string(),
                prompt: "Provide a brief, step-by-step guide on how to set up SSH keys for GitHub authentication.".to_string(),
            },
            Step {
                title: "Configure Git with your GitHub credentials".to_string(),
                prompt: "Explain concisely how to configure Git with GitHub credentials, focusing only on the essential commands.".to_string(),
            },
            Step {
                title: "Create your first repository".to_string(),
                prompt: "Explain succinctly how to create a new repository on GitHub, covering only the basic steps.".to_string(),
            },
            Step {
                title: "Clone the repository to your local machine".to_string(),
                prompt: "Provide a concise explanation of how to clone a GitHub repository to a local machine, including the basic command.".to_string(),
            },
            Step {
                title: "Make changes and commit them".to_string(),
                prompt: "Explain briefly how to make changes to files and commit them using Git, focusing on the essential commands.".to_string(),
            },
            Step {
                title: "Push changes to GitHub".to_string(),
                prompt: "Provide a short, clear explanation of how to push local commits to GitHub, including the basic command.".to_string(),
            },
            Step {
                title: "Create a branch and make a pull request".to_string(),
                prompt: "Explain concisely how to create a branch and make a pull request on GitHub, covering only the essential steps.".to_string(),
            },
            Step {
                title: "Collaborate on a project".to_string(),
                prompt: "Provide a brief overview of how to start collaborating on a GitHub project, mentioning key concepts like forking and contributing.".to_string(),
            },
        ],
    }
}
