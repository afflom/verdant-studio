Feature: project

  A Verdant Studio project as its own isolated instance, in app/src/Studio/Project.lex.tex.
  The app answers studio TAB project TAB maintainer TAB PROJECT TAB OWNER TAB PRISMID=LOGIN...
  with the project's maintainer, and studio TAB project TAB access TAB PROJECT TAB TARGET TAB
  REQUESTER TAB SCOPE with the signing request it hands to Verdant Engine, or a refusal.

  @VS-38 @build
  Scenario: A Verdant Studio project's maintainer, by Studio.Project, is the Prism ID whose linked GitHub identity is the project's owner, each link a Prism ID attributed a GitHub login as in W3C PROV-O wasAttributedTo
    Given the project model in app/src/Studio/Project.lex.tex
    When prism-code check has Lean's kernel prove Studio.Project.the_owners_linked_prism_id_maintains_the_project
    Then of the links prism:agent=claude and prism:alex=afflom, the Prism ID linked to afflom, the owner of verdant-studio, maintains it
    And prism-code build replays that maintainer request, answered maintainer of verdant-studio: prism:alex

  @VS-39 @build @rejects
  Scenario: Studio.Project refuses to name a maintainer for a project whose owner has no linked Prism ID, naming the owner and the project
    Given the project model in app/src/Studio/Project.lex.tex
    When prism-code check has Lean's kernel prove Studio.Project.refuses_a_project_whose_owner_has_no_linked_prism_id
    Then a project whose owner afflom has no linked Prism ID is answered with the refusal that names both
    And prism-code build replays the maintainer request with only the link prism:agent=claude, answered with that refusal

  @VS-40 @build @rejects
  Scenario: Each Verdant Studio project is its own isolated instance, so Studio.Project refuses a request about any other project, naming the instance that answers it
    Given the project model in app/src/Studio/Project.lex.tex
    When prism-code check has Lean's kernel prove Studio.Project.refuses_a_request_about_another_project
    Then an access request to verdant-studio about other-game is answered with the isolation refusal
    And prism-code build replays that request, answered with that refusal

  @VS-41 @build
  Scenario: Studio.Project hands an access request to its own project on to Verdant Engine's signing-request and admin-approval scheme, VE-70, as the typed request that names the requester, the project and the scope for its maintainers to approve or reject, and decides none of it itself
    Given the project model in app/src/Studio/Project.lex.tex
    When prism-code check has Lean's kernel prove Studio.Project.hands_an_access_request_to_the_engine
    Then an access request of prism:agent to verdant-studio for contribute is answered with the signing request for VE-70 that names all three
    And prism-code build replays that request, answered with that signing request

  @VS-42 @open
  Scenario: Verdant Engine's VE-70 signing-request and admin-approval scheme is callable from Verdant Studio, so that the signing request Studio.Project hands on for an access request is decided by the engine; REQ-PROJECT-INSTANCE depends on it, and until the engine exposes it Studio holds the request it would send
    Given the signing request of VS-41
    When Verdant Engine exposes VE-70 to Verdant Studio
    Then the engine decides the request, which Studio does not do itself
