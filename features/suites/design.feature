Feature: design

  ISO 9241-210 human-centred design activities of the Studio UI Prism, in app/src/Studio/Gui/Design.lex.tex, inferred
  from the standard's public preview. The app answers studio TAB design TAB stage TAB ACTIVITY and studio TAB design TAB cycle.

  @VS-237 @build
  Scenario: Human-centred design begins by understanding and specifying the context of use (ISO 9241-210:2019 7.2, inferred from its public preview, ISO-9241-210-2019-PREVIEW), so Studio.Gui.Design traces that activity to Studio.Observer, which observes each participant in the journey they take part in
    Given the design model in app/src/Studio/Gui/Design.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Design.the_context_of_use_is_observed
    Then the context activity is traced to Studio.Observer
    And prism-code build replays that request, answered with that answer

  @VS-238 @build
  Scenario: User requirements are specified from user and other stakeholder needs (ISO 9241-210:2019 7.3, inferred), so Studio.Gui.Design traces that activity to Studio.Feedback, where players' masked feedback is distilled by consensus
    Given the design model in app/src/Studio/Gui/Design.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Design.user_requirements_come_from_feedback
    Then the requirements activity is traced to Studio.Feedback
    And prism-code build replays that request, answered with that answer

  @VS-239 @build
  Scenario: Human-centred design is iterative over its four activities (ISO 9241-210:2019 5.5 and clause 7, inferred), so Studio.Gui.Design answers the whole cycle, each activity traced to the Studio stage that carries it out, the Prism generation producing design solutions and the accessibility and usability rules evaluating them, with evaluation returning to the context of use
    Given the design model in app/src/Studio/Gui/Design.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Design.the_design_is_one_cycle
    Then the cycle lists the four activities and returns to the context of use
    And prism-code build replays that request, answered with that answer

  @VS-240 @build @rejects
  Scenario: Studio.Gui.Design refuses an activity that ISO 9241-210:2019 clause 7 does not name (inferred), naming the activities it has
    Given the design model in app/src/Studio/Gui/Design.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Design.refuses_an_activity_outside_clause_7
    Then an activity outside clause 7 is answered with the refusal
    And prism-code build replays that request, answered with that answer

  @VS-241 @build
  Scenario: Human-centred design is iterative (ISO 9241-210:2019 5.5, inferred), so Studio.Gui.Design proves that four steps of its cycle return to the context of use
    Given the design model in app/src/Studio/Gui/Design.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Design.the_design_cycle_closes
    Then the cycle returns to the context of use
