import "time"

#project: {
	simulator: {
		filters: {
			can_output_tasks_for_agents: true
			supports_multiple_agents:    true
		}
		results: [_]: #simulation
		results: {
			with_final_capital_more_than_initial_capital: {
				states: {
					initial: #state
					final:   #state
				}
				filters: {
					final_capital_higher_than_initial_capital: true & states.final.capital > states.initial.capital
				}
			}
			// there must be a simulation that shows
			with_final_capital_less_or_equal_to_initial_capital: {
				states: {
					initial: #state
					final:   #state
				}
				filters: {
					final_capital_higher_than_initial_capital: true & states.final.capital <= states.initial.capital
				}
			}
		}
	}
	#simulation: {
		seed: #seed
		states: [_]: #state
		states: {
			initial: #state
			final:   #state
		}
		...
	}
	#state: {
		capital: uint
		...
	}
	simulator: {
		filters: {
			can_simulate_sale_of_property: {
				can_show_revenue_expected_value: {
					can_be_positive: true
					can_be_positive: true
				}
			}
		}
		filters_signoff: #signoff
	}
}

// seed is a path to the file; this file contains a sequence of bytes that are used as seed for the data
// see https://docs.rs/arbitrary/1.4.1/arbitrary/index.html
#seed: #path

// TODO: validate the string as a filesystem path relative to the project root
#path: string

_tasksAll: #nodes
_tasksAll: [
	{
		node: "Implement a simulator"
		filters: [
			["Must simulate the sale of property", [
				["Must show that it's possible to operate at a loss", []],
				["Must show a positive mathematical expectation", [
					["Using 'Must' because lots of agencies are profitable", []],
				]],
				["Must show the maximum amount of money to be made without hiring managers", []],
				["Ensure that amount of money in the system is limited by total supplies and changes with inflation over time", []],
			]],
		]
		decisions: []
	},
	{
		node: "Choose the next task"
		options: [
			{
				node: "Build a database & workflow system for agents"
			},
			{
				node: "Build an aggregator of RFQs for agents"
				nodes: [
					["Listen to the chats", []],
					["Use an LLM to filter the RFQs", []],
					["Use an LLM to extract data from RFQs", []],
					["Let the agents subscribe & create their filters", []],
					["Promote & get the initial audience via direct messages", []],
				]
			},
			["Build a chatbot prototype for the agents", [
				["Filters", [
					["Must support sending photos", []],
					["Must have a database of current properties (at least 20 rows)", []],
				]],
				["Decisions", [
					["Decide on a database", [
						["Filters", []],
						["Estimators", [
							["Should support enums", []],
						]],
						["Options", [
							["PostgreSQL", []],
							["Convex", []],
						]],
						["Notes", [
							["Correct data model is very important", []],
							["We can setup an export from Convex to PostgreSQL", []],
						]],
					]],
				]],
				["Options", [
					["Build a Custom GPT", [
						["Questions", [
							["How to uniquely identify the houses? - By optional UUID", [
								["Options", [
									["By location", [
										["But we don't know the exact location of many houses", []],
									]],
									["By hash of description", [
										["But changing a single character would change the hash", []],
									]],
									["By UUID", []],
									["By sequential id", []],
									["By optional UUID", []],
								]],
							]],
							["Does it support sending photos?", []],
							["What parameters should the /search endpoint have?", [
								["Notes", [
									["Convex can't execute arbitrary queries", []],
									["PostgreSQL can execute arbitrary queries", []],
								]],
							]],
							["Notes", [
								["We can use Actions + Convex HTTP endpoints", []],
								["We can provide multiple different endpoints, and the Custom GPT will select the best one", []],
								["Custom GPT & Convex both support OAuth", []],
							]],
						]],
						["Build a RAG pipeline", [
							["Docs", [
								["https://github.com/lehoanglong95/rag-all-in-one", []],
							]],
							["Find a startup that's developing a bot which can query the database", []],
						]],
					]],
				]],
			]],
		]
	},

]

_tasksActive: [
	for _task in _tasksAll
	if (*_task.active_after | _time.zero) < _now {_task},
]

// This value should be printed by `cue eval`
task:  _tasksActive[0]
tasks: _tasksActive

_now: time.Time @tag(n,var=now)

_time: zero: time.Unix(0, 0)

#user: "JuliaLebedeva" | "DenisGorbachev"

#datetime: time.Format(time.UnixDate)
_datetime_examples: [...#datetime]
_datetime_examples: ["Mon Jan 02 15:04:05 MST 2006"]

#signoff: [#user, #datetime]

// TODO: Import from a package
// The struct variant is deliberately left open to allow the user to add custom keys
#node: string | [string, #nodes] | {
	node?:  #node
	nodes?: #nodes
	...
}
#nodes: [...#node]
