Feature: gui-approval

  The approval journey of the Studio UI Prism, in app/src/Studio/Gui/Approval.lex.tex. The app answers
  studio TAB gui TAB approval TAB view TAB VIEWER TAB AGENT TAB MAINTAINER TAB SCOPE TAB blake3:DIGEST TAB AGENT=MAINTAINER...
  with the journey, its view and its actions.

  @VS-109 @build
  Scenario: Studio.Gui.Approval presents the approval journey of the Studio UI Prism in Stitch Product.Experience's pattern: a linked agent's content-addressed permission request (Studio.Agent.Contributor) shows in the approvalPending view, and the maintainer viewing it has the grant and reject actions enabled
    Given the approval journey in app/src/Studio/Gui/Approval.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Approval.the_maintainer_decides_a_pending_request
    Then prism:alex sees prism:agent's request for app/src on that content with grant and reject enabled
    And prism-code build replays that request, answered with that view

  @VS-110 @build
  Scenario: Only the maintainer grants or rejects a contributor's request, so Studio.Gui.Approval shows the request to any other viewer with grant and reject disabled
    Given the approval journey in app/src/Studio/Gui/Approval.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Approval.another_viewer_cannot_decide
    Then prism:agent viewing its own request sees grant and reject disabled
    And prism-code build replays that request, answered with that view

  @VS-111 @build
  Scenario: Studio.Gui.Approval presents a request Studio.Agent.Contributor refuses in the approvalRefused view with that refusal, grant and reject disabled
    Given the approval journey in app/src/Studio/Gui/Approval.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Approval.shows_why_a_request_is_refused
    Then a request naming latest shows approvalRefused with the content address refusal
    And prism-code build replays that request, answered with that view

  @VS-112 @build
  Scenario: The grant and reject actions of the approval journey are guarded, so Studio.Gui.Approval never enables them for a viewer who is not the request's maintainer, whatever is pending
    Given the approval journey in app/src/Studio/Gui/Approval.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Approval.only_the_maintainer_decides
    Then grant and reject are disabled for any other viewer
