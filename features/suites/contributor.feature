Feature: contributor

  The implementing agent as a Contributor, in app/src/Studio/Agent/Contributor.lex.tex. The app answers
  studio TAB contributor TAB act TAB AGENT TAB MAINTAINER TAB AGENT=MAINTAINER... and studio TAB contributor TAB
  permission TAB AGENT TAB MAINTAINER TAB SCOPE TAB blake3:DIGEST TAB AGENT=MAINTAINER... with its answer, or a refusal.

  @VS-59 @build
  Scenario: The implementing agent acts in Verdant Studio as a Contributor of the maintainer under its own Prism ID, by Studio.Agent.Contributor, when that Prism ID is linked to the maintainer's, each link a Prism ID acting on behalf of another as in W3C PROV-O actedOnBehalfOf
    Given the contributor model in app/src/Studio/Agent/Contributor.lex.tex
    When prism-code check has Lean's kernel prove Studio.Agent.Contributor.the_linked_agent_contributes
    Then prism:agent, linked to prism:alex, contributes on behalf of prism:alex
    And prism-code build replays that request, answered with that answer

  @VS-60 @build @rejects
  Scenario: Studio.Agent.Contributor refuses an agent whose Prism ID is not linked to the maintainer's, naming both
    Given the contributor model in app/src/Studio/Agent/Contributor.lex.tex
    When prism-code check has Lean's kernel prove Studio.Agent.Contributor.refuses_an_agent_not_linked_to_the_maintainer
    Then prism:agent with no link to prism:alex is answered with the unlinked refusal
    And prism-code build replays that request, answered with that refusal

  @VS-61 @build @rejects
  Scenario: The implementing agent has a Prism ID of its own, so Studio.Agent.Contributor refuses the maintainer's own Prism ID as a contributor
    Given the contributor model in app/src/Studio/Agent/Contributor.lex.tex
    When prism-code check has Lean's kernel prove Studio.Agent.Contributor.refuses_the_maintainers_own_prism_id_as_the_agent
    Then prism:alex named as its own contributor is answered with that refusal
    And prism-code build replays that request, answered with that refusal

  @VS-62 @build
  Scenario: Studio.Agent.Contributor hands a linked contributor's permission request, scoped to what it may change and naming its content by a BLAKE3 content address, on to Verdant Engine's VE-70 signing-request scheme for the maintainer to grant or reject
    Given the contributor model in app/src/Studio/Agent/Contributor.lex.tex
    When prism-code check has Lean's kernel prove Studio.Agent.Contributor.hands_a_contributors_permission_request_to_the_engine
    Then prism:agent's request for app/src on that content is handed to VE-70
    And prism-code build replays that request, answered with that answer

  @VS-63 @build @rejects
  Scenario: A contributor's permission request is content-addressed, so Studio.Agent.Contributor refuses one whose content is not named by blake3: and 64 hex digits
    Given the contributor model in app/src/Studio/Agent/Contributor.lex.tex
    When prism-code check has Lean's kernel prove Studio.Agent.Contributor.refuses_a_permission_request_without_a_content_address
    Then a request naming latest is answered with the content address refusal
    And prism-code build replays that request, answered with that refusal

  @VS-64 @open
  Scenario: The implementing agent's changes reach Verdant Studio as the VE-70 permission requests its maintainer grants, superseding GitHub App pull requests and owner approval (REQ-AGENT-IDENTITY), with the GitHub App only pushing the mirror when a release is published; until VE-70 is callable from Studio (VS-42), the App pull requests carry them
    Given the permission request of VS-62
    When Verdant Engine exposes VE-70 to Verdant Studio
    Then the maintainer grants the request in Verdant, and the GitHub App pushes only the mirror on a release
