use {super::{mocks::*, model::*},
     dioxus::prelude::ScopeState,
     fermi::*};

// List of skills:
// MUI, Storybook
// effector, Redux
// Ramda, fp-ts
// Deno, Node.js
// Ruby, Ruby on Rails
// Elixir, Elixir Phoenix
// GraphQL, RPC, REST
// Linux, macOS, CI/CD

static PROFILE_MOCK: Atom<Expert> = Atom(|_| {
	let roles_mock = vec![
	                      "Senior Frontend Engineer".into(),
	                      "Full-stack Engineer".into(),
	                      "Software Architect".into(),
	                      "Junior Smart Contract Developer".into(),
	];

	let links_mock = Links { website:  Some(WEBSITE_URL_MOCK.into()),
	                         linkedin: Some(LINKEDIN_URL_MOCK.into()),
	                         github:   Some(GITHUB_URL_MOCK.into()),
	                         telegram: Some(TELEGRAM_URL_MOCK.into()),
	                         x:        None, };

	let skills_mock = vec![
	                       Skill { name: "Rust".into(),
	                               icon: None, },
	                       Skill { name: "Dioxus".into(),
	                               icon: None, },
	                       Skill { name: "Blockchain".into(),
	                               icon: None, },
	                       Skill { name: "Smart Contracts".into(),
	                               icon: None, },
	                       Skill { name: "Software Architecture".into(),
	                               icon: None, },
	                       Skill { name: "Functional Programming".into(),
	                               icon: None, },
	                       Skill { name: "Object-Oriented Programming".into(),
	                               icon: None, },
	                       Skill { name: "Feature-Sliced Design".into(),
	                               icon: None, },
	                       Skill { name: "Domain Driven Development".into(),
	                               icon: None, },
	                       Skill { name: "Test Driven Development".into(),
	                               icon: None, },
	                       Skill { name: "NEAR Protocol".into(),
	                               icon: None, },
	                       Skill { name: "TypeScript".into(),
	                               icon: None, },
	                       Skill { name: "React".into(),
	                               icon: None, },
	                       Skill { name: "Preact".into(),
	                               icon: None, },
	                       Skill { name: "JavaScript".into(),
	                               icon: None, },
	                       Skill { name: "HTML".into(),
	                               icon: None, },
	                       Skill { name: "CSS".into(),
	                               icon: None, },
	                       Skill { name: "CSS-in-JS".into(),
	                               icon: None, },
	                       Skill { name: "UnoCSS".into(),
	                               icon: None, },
	                       Skill { name: "UI/UX Design".into(),
	                               icon: None, },
	                       Skill { name: "Tailwind CSS".into(),
	                               icon: None, },
	                       Skill { name: "Material UI".into(),
	                               icon: None, },
	                       Skill { name: "Next.js".into(),
	                               icon: None, },
	                       Skill { name: "Vue.js".into(),
	                               icon: None, },
	                       Skill { name: "Nuxt.js".into(),
	                               icon: None, },
	                       Skill { name: "Clojure".into(),
	                               icon: None, },
	                       Skill { name: "ClojureScript".into(),
	                               icon: None, },
	                       Skill { name: "Dart".into(),
	                               icon: None, },
	                       Skill { name: "Flutter".into(),
	                               icon: None, },
	];

	let frontend_engineer_architect_mock = vec![
	                                            "Senior Frontend Engineer".into(),
	                                            "Frontend Architect".into(),
	];

	let experience_mock =
		vec![
		     Position { project_name: "NEAR DevHub".into(),
		                project_url:  Some("https://neardevhub.org".into()),
		                roles:        vec!["Senior Frontend Engineer".into()],
		                term:         "Full-time".into(),
		                start_date:   1682884800,
		                end_date:     Some(1700092800),
		                summary:      Some("Something about DevHub".into()), },
		     Position { project_name: "Enter The Sphere".into(),
		                project_url:  Some("https://enter-the-sphere.com".into()),
		                roles:        frontend_engineer_architect_mock.to_owned(),
		                term:         "Part-time".into(),
		                start_date:   1673726400,
		                end_date:     Some(1680724800),
		                summary:      Some(ETS_POSITION_SUMMARY_MOCK.into()), },
		     Position { project_name: "NEAR Multicall".into(),
		                project_url:  Some("https://x.com/near_multicall".into()),
		                roles:        frontend_engineer_architect_mock.to_owned(),
		                term:         "Part-time".into(),
		                start_date:   1661976000,
		                end_date:     Some(1685476800),
		                summary:      Some("Something about NEAR Multicall experience.".into()), }
		];

	Expert { name:       NAME_MOCK.into(),
	         roles:      roles_mock,
	         avatar_url: AVATAR_URL_MOCK.into(),
	         banner_url: Some(BANNER_URL_MOCK.into()),
	         links:      links_mock,
	         location:   None,
	         status:     Some(STATUS_MOCK.into()),
	         contacts:   Contacts { email: "cvo.akaia@gmail.com".into(), },
	         summary:    SUMMARY_MOCK.into(),
	         skills:     skills_mock,
	         experience: experience_mock, }
});

pub fn use_profile(cx: &ScopeState) -> &Expert {
	use_read(cx, &PROFILE_MOCK)
}
