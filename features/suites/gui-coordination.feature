Feature: gui-coordination

  The coordination journey of the Studio UI Prism, in app/src/Studio/Gui/Coordination.lex.tex. The app answers
  studio TAB gui TAB coordination TAB view TAB MAINTAINER TAB OTHER TAB drafted-or-empty TAB AGENT=MAINTAINER... with
  the journey, the view and the send action.

  @VS-101 @build
  Scenario: Studio.Gui.Coordination presents the coordination journey of the Studio UI Prism in Stitch Product.Experience's pattern, a journey whose view is chosen by facts and whose actions are enabled by model guards: with the agent linked to the maintainer (Studio.Coordination), the coordinationOpen view, its send action enabled once a message is drafted
    Given the coordination journey in app/src/Studio/Gui/Coordination.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Coordination.presents_the_open_view_with_send_enabled
    Then prism:alex with its linked agent and a drafted message sees coordinationOpen with send enabled
    And prism-code build replays that request, answered with that view

  @VS-102 @build
  Scenario: Studio.Gui.Coordination presents the coordinationUnlinked view, its send action disabled, when the other Prism ID is not linked to the maintainer, as Studio.Coordination carries no message for that pair
    Given the coordination journey in app/src/Studio/Gui/Coordination.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Coordination.presents_the_unlinked_view_with_send_disabled
    Then prism:alex with prism:bob, not linked, sees coordinationUnlinked with send disabled
    And prism-code build replays that request, answered with that view

  @VS-103 @build
  Scenario: The send action of the coordination journey is guarded by the link, so Studio.Gui.Coordination never enables it for a pair that is not a maintainer and its linked agent, whatever is drafted
    Given the coordination journey in app/src/Studio/Gui/Coordination.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Coordination.an_unlinked_pair_never_sends
    Then send is disabled for an unlinked pair, drafted or not
