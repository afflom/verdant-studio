Feature: gui-mirror

  The mirror journey of the Studio UI Prism, in app/src/Studio/Gui/Mirror.lex.tex. The app answers
  studio TAB gui TAB mirror TAB view TAB VIEWER TAB MAINTAINER TAB REVISION TAB VERSION TAB INCREMENT TAB GATE... with
  the journey, its view and its action.

  @VS-200 @build
  Scenario: Studio.Gui.Mirror presents the mirror journey of the Studio UI Prism in Stitch Product.Experience's pattern: a release Studio.Runtime cuts once every gate passed shows in the mirrorReady view, and its maintainer has the publish action enabled, publishing to GitHub as a mirror only (REQ-SELF-RUNTIME)
    Given the mirror journey in app/src/Studio/Gui/Mirror.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Mirror.the_maintainer_publishes_a_cut_release
    Then prism:alex sees release 0.2.0 of revision c587c01 in mirrorReady with publish enabled
    And prism-code build replays that request, answered with that view

  @VS-201 @build
  Scenario: Only the maintainer publishes a release, so Studio.Gui.Mirror shows a cut release to any other viewer with publish disabled
    Given the mirror journey in app/src/Studio/Gui/Mirror.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Mirror.another_viewer_cannot_publish
    Then prism:agent sees the same release with publish disabled
    And prism-code build replays that request, answered with that view

  @VS-202 @build
  Scenario: Studio.Gui.Mirror presents a release Studio.Runtime does not cut in the mirrorHeld view with its refusal, publish disabled
    Given the mirror journey in app/src/Studio/Gui/Mirror.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Mirror.holds_a_release_with_a_failed_gate
    Then a failed export gate shows mirrorHeld with the gate refusal and publish disabled
    And prism-code build replays that request, answered with that view

  @VS-203 @build
  Scenario: The publish action of the mirror journey is guarded, so Studio.Gui.Mirror never enables it for a viewer who is not the maintainer, whatever is cut
    Given the mirror journey in app/src/Studio/Gui/Mirror.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Mirror.only_the_maintainer_publishes
    Then publish is disabled for any other viewer
