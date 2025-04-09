#database_name: "PostgreSQL" | "Convex"

#language_name: "Coda" | "AppSmith" | "TypeScript" | "Rust" | "SQL"

#storage: {git: true} | {saas: true} | {database: #database_name}

#architecture: {
	storage!: #storage
	languages!: [#language_name, ...#language_name]
}

architectures: [_]: #architecture
architectures: {
	custom: {
		storage: {database: "Convex"}
		languages: ["TypeScript"]
	}
	coda: {
		storage: {saas: true}
		languages: ["Coda"]
	}
	appsmith: {
		storage: {database: "Convex"}
		languages: ["AppSmith"]
	}
}
