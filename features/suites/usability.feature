Feature: usability

  ISO 9241-11 usability measures over the view tree of the Studio UI Prism, in app/src/Studio/Gui/Usability.lex.tex, inferred
  from the standard's public preview. The app answers studio TAB usability TAB journey TAB JOURNEY TAB ... and studio TAB usability TAB check TAB TREE.

  @VS-232 @build
  Scenario: Usability is effectiveness, efficiency and satisfaction in a context of use (ISO 9241-11:2018 3.1.1, inferred from its public preview, ISO-9241-11-2018-PREVIEW), so Studio.Gui.Usability measures a journey's view tree: the maintainer reaches the approval goal with both actions enabled, in one step, and satisfaction is left to the users' own feedback
    Given the usability model in app/src/Studio/Gui/Usability.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Usability.the_maintainer_reaches_the_approval_goal
    Then the maintainer's approval journey is measured as reached in one step
    And prism-code build replays that request, answered with that answer

  @VS-233 @build
  Scenario: Effectiveness is the accuracy and completeness with which users achieve specified goals (ISO 9241-11:2018 3.1.12, inferred), so Studio.Gui.Usability measures the approval journey of a viewer whose actions are disabled as not reaching its goal in that context, naming what its view says
    Given the usability model in app/src/Studio/Gui/Usability.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Usability.another_viewer_does_not_reach_the_approval_goal
    Then the agent viewing its own request is measured as not reaching the goal
    And prism-code build replays that request, answered with that answer

  @VS-234 @build
  Scenario: Efficiency is the resources used in relation to the results achieved (ISO 9241-11:2018 3.1.13, inferred), so Studio.Gui.Usability spends no step on a release a gate holds back, and its effectiveness names the gate that holds it
    Given the usability model in app/src/Studio/Gui/Usability.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Usability.a_held_release_names_what_it_needs
    Then a held release is measured as not reached, naming the failed gate
    And prism-code build replays that request, answered with that answer

  @VS-235 @build @rejects
  Scenario: Usability is measured against specified goals (ISO 9241-11:2018 3.1.1, inferred), so Studio.Gui.Usability refuses a view tree that opens with no journey and so names no goal
    Given the usability model in app/src/Studio/Gui/Usability.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Usability.refuses_a_tree_with_no_goal
    Then a tree with no journey is answered with the refusal
    And prism-code build replays that request, answered with that answer

  @VS-236 @build
  Scenario: An agent is measured as a person is (REQ-AGENT-USERS), so Studio.Gui.Usability gives the view tree an agent perceives exactly the usability measures it gives a person's, for every journey and request
    Given the usability model in app/src/Studio/Gui/Usability.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Usability.an_agent_and_a_person_get_the_same_measures
    Then the two measures are equal
