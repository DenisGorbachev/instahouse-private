# TODO

* Sort out the tasks
  * #tasks
    * Build an aggregator of RFQs for agents
      * #notes
        * Listen to the chats
        * Use an LLM to filter the RFQs
        * Use an LLM to extract data from RFQs
        * Let the agents subscribe & create their filters
        * Promote & get the initial audience via direct messages
* Build a prototype for the agents
  * Filters
    * Must support sending photos
    * Must have a database of current properties (at least 20 rows)
  * Decisions
    * Decide on a database
      * Filters
      * Estimators
        * Should support enums
      * Options
        * PostgreSQL
        * Convex
      * Notes
        * Correct data model is very important
        * We can setup an export from Convex to PostgreSQL
  * Options
    * Build a Custom GPT
      * Questions
        * How to uniquely identify the houses? - By optional UUID
          * Options
            * By location
              * But we don't know the exact location of many houses
            * By hash of description
              * But changing a single character would change the hash
            * By UUID
            * By sequential id
            * By optional UUID
        * Does it support sending photos?
        * What parameters should the /search endpoint have?
          * Notes
            * Convex can't execute arbitrary queries
            * PostgreSQL can execute arbitrary queries
      * Notes
        * We can use Actions + Convex HTTP endpoints
        * We can provide multiple different endpoints, and the Custom GPT will select the best one
        * Custom GPT & Convex both support OAuth
    * Build a RAG pipeline
      * Docs
        * https://github.com/lehoanglong95/rag-all-in-one
    * Find a startup that's developing a bot which can query the database
