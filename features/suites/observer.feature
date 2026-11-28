Feature: observer

  The Verdant Observer of Studio, in app/src/Studio/Observer.lex.tex. The app answers studio TAB observer TAB PRISM-ID TAB
  and a Studio.User request with the observation of that participant, then the answer Studio.User gives.

  @VS-211 @build
  Scenario: An agent communes with the Verdant Observer as a participant (REQ-AGENT-OBSERVER), so Studio.Observer answers an agent's perception of a journey with a W3C SOSA observation of it, associated with the agent's Prism ID by PROV-O, before the view Studio.User gives it
    Given the Observer model in app/src/Studio/Observer.lex.tex
    When prism-code check has Lean's kernel prove Studio.Observer.observes_an_agent_as_a_participant
    Then prism:agent perceiving the approval journey is answered with its observation through verdant-studio://gui/approval, then the approvalPending view
    And prism-code build replays that request, answered with that answer

  @VS-212 @build
  Scenario: The Verdant Observer observes every stakeholder and channel of Verdant (VERDANT-ENGINE-OBSERVER-2.22.4), so Studio.Observer observes a person's action on a journey just as it observes an agent's, before the answer Studio.User gives
    Given the Observer model in app/src/Studio/Observer.lex.tex
    When prism-code check has Lean's kernel prove Studio.Observer.observes_a_person_as_a_participant
    Then prism:alex enacting grant is answered with its observation through the Studio UI, then the grant enacted
    And prism-code build replays that request, answered with that answer

  @VS-213 @build @rejects
  Scenario: A PROV-O association names its agent, and a Verdant participant is named by its Prism ID (REQ-SELF-RUNTIME), so Studio.Observer refuses a participant that names no Prism ID
    Given the Observer model in app/src/Studio/Observer.lex.tex
    When prism-code check has Lean's kernel prove Studio.Observer.refuses_a_participant_with_no_prism_id
    Then a participant named alex is answered with the refusal
    And prism-code build replays that request, answered with that answer

  @VS-214 @build
  Scenario: Observing never changes what a participant experiences, so Studio.Observer answers every participant and request with its observation followed by exactly the answer Studio.User gives
    Given the Observer model in app/src/Studio/Observer.lex.tex
    When prism-code check has Lean's kernel prove Studio.Observer.observing_never_changes_what_a_participant_experiences
    Then the answer after the observation is the Studio.User answer
