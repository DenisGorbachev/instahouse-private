# Instahouse Knowledge Base

## Business models

* [Aggregator for tenants](#aggregator-for-tenants)
* [MLS for agents](#mls-for-agents)

### Aggregator for tenants

* Target audience
  * Tenants (people looking to rent a house)
    * Problems
      * Hard to determine whether the house is suitable based on incomplete information
      * Hard to see all available houses (multiple chats, different languages)
      * Hard to filter the houses manually
      * Risk of renting in advance: making a deposit but not getting the house (and sometimes not even getting the deposit back)
    * Solutions
      * Provide video
      * Provide [structured data](#structured-data)
      * Provide automated filters via chatbot

### MLS for agents

## Plan

* Create a channel
* Setup monitoring of most active channels on Koh Phangan and Koh Samui
* Setup a process of structuring the data from the ads
* Setup a process of contacting the original poster
  * Ask for permission to repost their ad on our channels
  * Ask for permission to reuse their photos
  * Ask for video
  * Ask for data
* Setup a process of posting the ads in different languages
* Setup a process of getting payments
  * For agents: no charge initially (they keep 100% of commission)
  * For regulars: ask for a post on social channels if they actually make a deal

## Questions

* What are the biggest problems of the tenants?
* What are the biggest problems of the agents?
* Is it possible to sign the data-sharing agreements with agents?
* Is there a need for data-sharing platform for the agents? (MLS)

## Notes

* Notes after looking at [Панган Ко-Панган / Жилье | Аренда и продажа](https://t.me/kohphangan/442338)
  * Two agents have posted the same house
    * https://t.me/kohphangan/442338/682607
    * https://t.me/kohphangan/442338/684998
  * Some people have posted [RFPs](#request-for-proposals)
* The landlord may or may not speak the language of the tenant

### Structured data

* Options
  * Allow people to fill the form
    * But it's time-consuming and confusing
  * Use an LLM to extract the data from the current messages
  * Ask people for more data via chat
    * Automate it via LLM, too
  * Reuse existing data from the database
    * Notes
      * If someone posts an ad for the same house, we can reuse some data
        * Notes
          * The data is changed with different frequency
            * Examples
              * Number of rooms changes rarely (but still can change: people can build walls within existing rooms, people can add an extension to an existing building)

## Definitions

### Request for proposals
