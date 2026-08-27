use crate::models::Agent;

pub fn seed_agents() -> Vec<Agent> {
    vec![
        Agent {
            id: "email-writer".into(),
            name: "Email Writer".into(),
            prompt: "Write a {{tone}} email to {{recipient}} about {{topic}}. Keep it under {{length}} words.".into(),
            model: None,
            provider: Default::default(),
            runs: vec![],
        },
        Agent {
            id: "product-desc".into(),
            name: "Product Description Generator".into(),
            prompt: "Write a compelling product description for {{product_name}}, targeted at {{audience}}. Highlight the key benefit: {{key_benefit}}.".into(),
            model: None,
            provider: Default::default(),
            runs: vec![],
        },
        Agent {
            id: "code-explainer".into(),
            name: "Code Explainer".into(),
            prompt: "Explain the following {{language}} code to a {{audience_level}} developer in plain language:\n\n{{code_snippet}}".into(),
            model: None,
            provider: Default::default(),
            runs: vec![],
        },
        Agent {
            id: "meeting-summarizer".into(),
            name: "Meeting Summarizer".into(),
            prompt: "Summarize the meeting notes below into {{format}}, focused on decisions and action items. Attendees: {{attendees}}.\n\nNotes:\n{{meeting_notes}}".into(),
            model: None,
            provider: Default::default(),
            runs: vec![],
        },
        Agent {
            id: "social-caption".into(),
            name: "Social Caption Creator".into(),
            prompt: "Write a {{platform}} caption for a post about {{subject}} in a {{tone}} voice.".into(),
            model: None,
            provider: Default::default(),
            runs: vec![],
        },
    ]
}