todo: [
	"Import #DatabaseName and #LanguageName from tms.cue",
]

#DatabaseName: "PostgreSQL" | "Convex"

#LanguageName: "Coda" | "TypeScript" | "Rust" | "SQL"

#Storage: {git: true} | {saas: true} | {database: #DatabaseName}

#Architecture: {
	storage!: #Storage
	languages!: [#LanguageName, ...#LanguageName]
}

architectures: [_]: #Architecture
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

	}
}
