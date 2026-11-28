Feature: user

  Verdant users, in app/src/Studio/User.lex.tex. The app answers studio TAB user TAB perceive TAB KIND TAB JOURNEY TAB ...
  with the journey's view, and studio TAB user TAB enact TAB KIND TAB JOURNEY TAB ACTION TAB ... with the action enacted or refused.

  @VS-204 @build
  Scenario: An agent is a Verdant user along with human users (REQ-AGENT-USERS), so Studio.User gives an agent the view of a journey of the Studio UI Prism through that journey's MCP resource, the same view Stitch Product.Experience gives a person
    Given the Verdant users model in app/src/Studio/User.lex.tex
    When prism-code check has Lean's kernel prove Studio.User.an_agent_perceives_its_journey_through_an_mcp_resource
    Then an agent perceiving the approval journey as prism:agent gets the approvalPending view through verdant-studio://gui/approval
    And prism-code build replays that request, answered with that answer

  @VS-205 @build
  Scenario: An agent enacts an action of the Studio UI Prism only when its guard enables it, as an MCP tools/call carried over Stitch by Studio.Agent.Interface.Transport
    Given the Verdant users model in app/src/Studio/User.lex.tex
    When prism-code check has Lean's kernel prove Studio.User.an_agent_enacts_an_enabled_action_over_stitch
    Then an agent enacting send on a linked, drafted coordination is answered that it enacted send over Stitch
    And prism-code build replays that request, answered with that answer

  @VS-206 @build @rejects
  Scenario: The same action guard gates the control a person presses and the call an agent makes (Stitch Product.Experience), so Studio.User refuses an agent the grant of its own request, as the approval journey disables it
    Given the Verdant users model in app/src/Studio/User.lex.tex
    When prism-code check has Lean's kernel prove Studio.User.refuses_an_agent_an_action_its_guard_disables
    Then an agent enacting grant on its own request is answered with the refusal
    And prism-code build replays that request, answered with that answer

  @VS-207 @build
  Scenario: A person enacts an enabled action of the Studio UI Prism through the Studio UI, so Studio.User enacts the maintainer's grant of its agent's request
    Given the Verdant users model in app/src/Studio/User.lex.tex
    When prism-code check has Lean's kernel prove Studio.User.a_person_enacts_an_enabled_action_through_the_studio_ui
    Then the maintainer enacting grant is answered that it enacted grant through the Studio UI
    And prism-code build replays that request, answered with that answer

  @VS-208 @build @rejects
  Scenario: A Verdant user is a human or an agent (REQ-AGENT-USERS), so Studio.User refuses a request from any other kind, naming it
    Given the Verdant users model in app/src/Studio/User.lex.tex
    When prism-code check has Lean's kernel prove Studio.User.refuses_a_user_of_no_known_kind
    Then a request from a robot is answered with the refusal
    And prism-code build replays that request, answered with that answer

  @VS-209 @build
  Scenario: An agent has the same graphical user interface experience of Verdant Studio as a person (REQ-AGENT-USERS), so Studio.User gives an agent exactly the view it gives a person, for every journey and every request
    Given the Verdant users model in app/src/Studio/User.lex.tex
    When prism-code check has Lean's kernel prove Studio.User.an_agent_perceives_the_view_a_human_perceives
    Then an agent's view equals a person's view

  @VS-210 @build
  Scenario: An agent acts under the same guards as a person (REQ-AGENT-USERS), so Studio.User enables an action for an agent exactly when it enables it for a person, for every journey, action and request
    Given the Verdant users model in app/src/Studio/User.lex.tex
    When prism-code check has Lean's kernel prove Studio.User.an_agent_is_held_to_the_guards_a_human_is_held_to
    Then an action is enabled for an agent exactly when it is enabled for a person
