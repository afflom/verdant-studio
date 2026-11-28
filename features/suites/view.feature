Feature: view

  The view tree of each stakeholder relation of the D10 matrix, in app/src/Studio/Gui/View.lex.tex. The app answers
  studio TAB view TAB RELATION with the journeys that relation opens, each followed by its WCAG 2.2 audit.

  @VS-225 @build
  Scenario: Every stakeholder gets a representation bespoke to its need (REQ-CREATOR-BESPOKE), so Studio.Gui.View builds the view tree of a producer role of the D10 matrix (prov:wasGeneratedBy) from the journeys that generate: the creator journey and its own instances, each audited by Studio.Gui.Accessibility
    Given the view model in app/src/Studio/Gui/View.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.View.the_producer_relation_opens_creation
    Then the producer relation is answered with the creator and instances journeys, each conforming
    And prism-code build replays that request, answered with that answer

  @VS-226 @build
  Scenario: A role that acts on behalf of another (prov:actedOnBehalfOf in the D10 matrix) gets the journeys of delegation, coordination and approval, with its instances, so Studio.Gui.View builds that tree and audits each journey
    Given the view model in app/src/Studio/Gui/View.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.View.the_delegate_relation_opens_delegation
    Then the onBehalfOf relation is answered with the coordination, approval and instances journeys, each conforming
    And prism-code build replays that request, answered with that answer

  @VS-227 @build
  Scenario: A role a release is attributed to (prov:wasAttributedTo in the D10 matrix) gets the mirror journey that publishes the release, with its instances, so Studio.Gui.View builds that tree and audits each journey
    Given the view model in app/src/Studio/Gui/View.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.View.the_attributed_tree
    Then the attributed relation is answered with the mirror and instances journeys, each conforming
    And prism-code build replays that request, answered with that answer

  @VS-228 @build
  Scenario: A role that uses what Studio publishes (prov:used, a consumer in the D10 matrix) gets the mirror journey of the published release, so Studio.Gui.View builds that tree and audits it
    Given the view model in app/src/Studio/Gui/View.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.View.the_consumer_tree
    Then the consumer relation is answered with the mirror journey, conforming
    And prism-code build replays that request, answered with that answer

  @VS-229 @build
  Scenario: Players never log in to Studio and give feedback in the game, so Studio.Gui.View gives an observed role of the D10 matrix (W3C SOSA) no journey and names the Verdant Observer as how it takes part
    Given the view model in app/src/Studio/Gui/View.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.View.an_observed_role_opens_no_journey
    Then the observation relation is answered that it opens no journey
    And prism-code build replays that request, answered with that answer

  @VS-230 @build @rejects
  Scenario: Studio.Gui.View builds view trees only for the PROV-O and SOSA stakeholder relations of the D10 matrix, so it refuses any other relation, naming it
    Given the view model in app/src/Studio/Gui/View.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.View.refuses_a_relation_the_matrix_has_not
    Then an ESCO skill relation is answered with the refusal
    And prism-code build replays that request, answered with that answer

  @VS-231 @build
  Scenario: An agent in a stakeholder role gets the tree a person in that role gets (REQ-AGENT-USERS), so each journey of a Studio.Gui.View tree, with its audit, is the same for an agent as for a person
    Given the view model in app/src/Studio/Gui/View.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.View.an_agent_gets_the_tree_a_person_gets
    Then each journey and its audit are equal for an agent and a person
