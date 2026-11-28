Feature: transport

  The agent's MCP tool calls as Stitch signed actions, in app/src/Studio/Agent/Interface/Transport.lex.tex. The app
  answers studio TAB transport TAB call TAB AGENT TAB MAINTAINER TAB TOOL TAB SCOPE TAB AUDIENCE TAB INSTANCE TAB
  unseen-or-seen TAB ISSUED TAB NOW TAB AGENT=MAINTAINER... with the call it runs, or a refusal.

  @VS-94 @build
  Scenario: Studio.Agent.Interface.Transport runs an MCP tools/call from the maintainer's linked agent as a Stitch signed action, only when Stitch.PrismId actionFresh holds (signed for this instance, its nonce unseen, issued within the 60000 ms window of Stitch's verifyAction) and actionAllowed holds (the tool inside the scope the maintainer granted), and hands it to the workspace operation of that name, the MCP tool Studio.Surface publishes
    Given the transport model in app/src/Studio/Agent/Interface/Transport.lex.tex
    When prism-code check has Lean's kernel prove Studio.Agent.Interface.Transport.runs_a_fresh_signed_call_inside_its_scope
    Then a call of next from prism:agent, linked to prism:alex, signed for verdant-studio, unseen and issued 30000 ms before now, runs next
    And prism-code build replays that request, answered with that answer

  @VS-95 @build @rejects
  Scenario: A signed action is bound to its audience (Stitch.PrismId actionFresh), so Studio.Agent.Interface.Transport refuses a call signed for another instance, naming both
    Given the transport model in app/src/Studio/Agent/Interface/Transport.lex.tex
    When prism-code check has Lean's kernel prove Studio.Agent.Interface.Transport.refuses_a_call_signed_for_another_audience
    Then a call signed for verdant-engine is answered with the audience refusal
    And prism-code build replays that request, answered with that refusal

  @VS-96 @build @rejects
  Scenario: A signed action runs once (Stitch.PrismId actionFresh), so Studio.Agent.Interface.Transport refuses a call whose nonce was already seen within the window
    Given the transport model in app/src/Studio/Agent/Interface/Transport.lex.tex
    When prism-code check has Lean's kernel prove Studio.Agent.Interface.Transport.refuses_a_replayed_call
    Then a call whose nonce was seen is answered with the replay refusal
    And prism-code build replays that request, answered with that refusal

  @VS-97 @build @rejects
  Scenario: A signed action is fresh only within the window of Stitch's verifyAction, 60000 ms, so Studio.Agent.Interface.Transport refuses a call issued farther from now, naming both times
    Given the transport model in app/src/Studio/Agent/Interface/Transport.lex.tex
    When prism-code check has Lean's kernel prove Studio.Agent.Interface.Transport.refuses_a_call_outside_the_window
    Then a call issued 90000 ms before now is answered with the freshness refusal
    And prism-code build replays that request, answered with that refusal

  @VS-98 @build @rejects
  Scenario: A delegate acts only inside its scope (Stitch.PrismId actionAllowed), so Studio.Agent.Interface.Transport refuses a tool the maintainer did not grant the agent, naming it
    Given the transport model in app/src/Studio/Agent/Interface/Transport.lex.tex
    When prism-code check has Lean's kernel prove Studio.Agent.Interface.Transport.refuses_a_tool_outside_the_granted_scope
    Then a call of publish under the scope next,check is answered with the scope refusal
    And prism-code build replays that request, answered with that refusal

  @VS-99 @build @rejects
  Scenario: Only the maintainer's linked agent calls its tools, so Studio.Agent.Interface.Transport refuses a call from an agent whose Prism ID is not linked to the maintainer's, by Studio.Agent.Contributor
    Given the transport model in app/src/Studio/Agent/Interface/Transport.lex.tex
    When prism-code check has Lean's kernel prove Studio.Agent.Interface.Transport.refuses_a_call_from_an_unlinked_agent
    Then a call from prism:agent with no link to prism:alex is answered with the unlinked refusal
    And prism-code build replays that request, answered with that refusal

  @VS-100 @open
  Scenario: The agent's MCP JSON-RPC messages reach Studio.Agent.Interface.Transport over Stitch's sealed relay as a custom MCP transport, each delivered only when Stitch.Relay deliveryAllowed holds; until an instance hosts the relay, the agent reaches the workspace operations through the GitHub App
    Given the call of VS-94
    When an instance hosts the Stitch relay
    Then the call arrives over it, delivered only when Stitch.Relay deliveryAllowed holds
