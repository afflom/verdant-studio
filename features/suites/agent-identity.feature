Feature: agent-identity

  The workspace's own identity, apart from the owner, in app/src/Studio/AgentIdentity.lex.tex. The app answers studio TAB identity TAB change TAB AUTHOR TAB WORKSPACE TAB OWNER.

  @VS-69 @build
  Scenario: The workspace has its own identity, apart from the owner, so Studio.AgentIdentity accepts a lifecycle change authored by that identity through the repository API, for the owner to approve, here by the studio check under the owner's automated approvals (2026-11-26)
    Given the agent identity model in app/src/Studio/AgentIdentity.lex.tex
    When prism-code check has Lean's kernel prove Studio.AgentIdentity.the_workspace_identity_runs_the_lifecycle
    Then a change by afflom-verdant-studio[bot] for afflom is accepted
    And prism-code build replays that request, answered with that answer

  @VS-70 @build @rejects
  Scenario: Studio.AgentIdentity refuses a lifecycle change authored by the owner's own identity, naming the workspace identity it runs under
    Given the agent identity model in app/src/Studio/AgentIdentity.lex.tex
    When prism-code check has Lean's kernel prove Studio.AgentIdentity.refuses_the_owners_identity_running_the_lifecycle
    Then a change authored by afflom is answered with the owner refusal
    And prism-code build replays that request, answered with that refusal

  @VS-71 @build @rejects
  Scenario: Studio.AgentIdentity refuses a lifecycle change authored by any identity other than the workspace's, naming both
    Given the agent identity model in app/src/Studio/AgentIdentity.lex.tex
    When prism-code check has Lean's kernel prove Studio.AgentIdentity.refuses_any_other_identity
    Then a change by someone is answered with that refusal
    And prism-code build replays that request, answered with that refusal
