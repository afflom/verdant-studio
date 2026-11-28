Feature: accessibility

  WCAG 2.2 rules over the view tree of the Studio UI Prism, in app/src/Studio/Gui/Accessibility.lex.tex. The app answers
  studio TAB accessibility TAB journey TAB JOURNEY TAB ... and studio TAB accessibility TAB check TAB TREE with the audit.

  @VS-219 @build
  Scenario: The Studio UI meets the WCAG 2.2 success criteria a view tree can be checked against (W3C-WCAG22-JSON), so Studio.Gui.Accessibility audits the creator journey of the seed Verdant Studio and finds every title, view label and action name, role and state present
    Given the accessibility model in app/src/Studio/Gui/Accessibility.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Accessibility.the_creator_journey_conforms
    Then the creator journey of Verdant Studio is answered that it conforms
    And prism-code build replays that request, answered with that answer

  @VS-220 @build
  Scenario: WCAG 2.2 success criterion 3.3.1 Error Identification asks that an error be described in text, so Studio.Gui.Accessibility finds the approval journey's refused view conforming because it says why the request was refused
    Given the accessibility model in app/src/Studio/Gui/Accessibility.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Accessibility.a_refused_view_identifies_its_error
    Then the approval journey with a refused request is answered that it conforms
    And prism-code build replays that request, answered with that answer

  @VS-221 @build @rejects
  Scenario: WCAG 2.2 success criterion 2.4.2 Page Titled asks for a title that describes the purpose, so Studio.Gui.Accessibility fails a view tree that opens with no journey title, naming its first line
    Given the accessibility model in app/src/Studio/Gui/Accessibility.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Accessibility.refuses_a_view_tree_with_no_title
    Then a view tree with no journey line is answered with the 2.4.2 failure
    And prism-code build replays that request, answered with that answer

  @VS-222 @build @rejects
  Scenario: WCAG 2.2 success criterion 4.1.2 Name, Role, Value asks that a component's name, role and state be programmatically determined, so Studio.Gui.Accessibility fails an action that states neither enabled nor disabled, naming it
    Given the accessibility model in app/src/Studio/Gui/Accessibility.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Accessibility.refuses_an_action_with_no_state
    Then an action with no state is answered with the 4.1.2 failure
    And prism-code build replays that request, answered with that answer

  @VS-223 @build @rejects
  Scenario: WCAG 2.2 success criterion 3.3.1 Error Identification asks that an error be described in text, so Studio.Gui.Accessibility fails a refused view that does not say why, naming it
    Given the accessibility model in app/src/Studio/Gui/Accessibility.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Accessibility.refuses_an_error_it_does_not_describe
    Then a refusal with no description is answered with the 3.3.1 failure
    And prism-code build replays that request, answered with that answer

  @VS-224 @build
  Scenario: An agent is audited as a person is (REQ-AGENT-USERS), so Studio.Gui.Accessibility gives the view tree an agent perceives exactly the audit it gives a person's, for every journey and request
    Given the accessibility model in app/src/Studio/Gui/Accessibility.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Accessibility.an_agent_and_a_person_get_the_same_audit
    Then the two audits are equal
